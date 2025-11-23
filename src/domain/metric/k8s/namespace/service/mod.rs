use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use chrono::{DateTime, Utc};
use crate::api::dto::{info_dto::K8sListQuery, metrics_dto::RangeQuery};
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::service::{info_k8s_pod_service, info_unit_price_service};
use crate::domain::metric::k8s::common::dto::{CommonMetricValuesDto, FilesystemMetricDto, MetricGetResponseDto, MetricScope, MetricSeriesDto, NetworkMetricDto, UniversalMetricPointDto};
use crate::domain::metric::k8s::common::service_helpers::{aggregate_cost_points, apply_costs, build_cost_summary_dto, build_cost_trend_dto, build_raw_summary_value};
use crate::domain::metric::k8s::pod::service::build_pod_response_from_infos;

fn group_pods_by_namespace(pods: Vec<InfoPodEntity>) -> HashMap<String, Vec<InfoPodEntity>> {
    let mut map: HashMap<String, Vec<InfoPodEntity>> = HashMap::new();
    for pod in pods {
        if let Some(ns) = &pod.namespace {
            map.entry(ns.clone()).or_default().push(pod);
        }
    }
    map
}

fn build_namespace_response(
    namespace: &str,
    per_pod_response: &MetricGetResponseDto,
) -> MetricGetResponseDto {
    let all_points: Vec<UniversalMetricPointDto> =
        per_pod_response.series.iter().flat_map(|s| s.points.clone()).collect();

    let aggregated_points = aggregate_namespace_points(all_points);

    MetricGetResponseDto {
        start: per_pod_response.start,
        end: per_pod_response.end,
        scope: "namespace".to_string(),
        target: Some(namespace.to_string()),
        granularity: per_pod_response.granularity.clone(),
        series: vec![MetricSeriesDto {
            key: namespace.to_string(),
            name: namespace.to_string(),
            scope: MetricScope::Namespace,
            points: aggregated_points,
        }],
        total: None,
        limit: None,
        offset: None,
    }
}

/// Aggregate namespace-level metrics by summing pod-level points per timestamp.
///
/// Behavior:
/// - CPU & Memory: SUM — represents the total resource usage of the namespace
/// - Filesystem: SUM — total usage across pods
/// - Network: SUM — total traffic per namespace
///
/// Null Handling Rules:
/// - Null values are skipped during aggregation
/// - If all values for a field are null → output will be null
/// - Zero (0) is treated as a valid data point (not null)
pub fn aggregate_namespace_points(
    points: Vec<UniversalMetricPointDto>,
) -> Vec<UniversalMetricPointDto> {
    let mut buckets: BTreeMap<DateTime<Utc>, Vec<UniversalMetricPointDto>> = BTreeMap::new();

    for p in points {
        buckets.entry(p.time).or_default().push(p);
    }

    let mut out = Vec::with_capacity(buckets.len());

    for (time, bucket) in buckets {
        // CPU / Memory
        let mut cpu_sum = 0.0;
        let mut cpu_count = 0.0;

        let mut cpu_core_sum = 0.0;
        let mut cpu_core_count = 0.0;

        let mut mem_sum = 0.0;
        let mut mem_count = 0.0;

        let mut mem_working_sum = 0.0;
        let mut mem_working_count = 0.0;

        let mut mem_rss_sum = 0.0;
        let mut mem_rss_count = 0.0;

        let mut mem_pf_sum = 0.0;
        let mut mem_pf_count = 0.0;

        // Filesystem SUM
        let mut fs_used_sum = 0.0;
        let mut fs_used_count = 0.0;
        let mut fs_capacity_sum = 0.0;
        let mut fs_capacity_count = 0.0;
        let mut fs_inodes_used_sum = 0.0;
        let mut fs_inodes_used_count = 0.0;
        let mut fs_inodes_sum = 0.0;
        let mut fs_inodes_count = 0.0;

        // Network SUM
        let mut rx_sum = 0.0;
        let mut rx_count = 0.0;
        let mut tx_sum = 0.0;
        let mut tx_count = 0.0;
        let mut rx_err_sum = 0.0;
        let mut rx_err_count = 0.0;
        let mut tx_err_sum = 0.0;
        let mut tx_err_count = 0.0;

        for p in &bucket {
            // CPU
            if let Some(v) = p.cpu_memory.cpu_usage_nano_cores {
                cpu_sum += v;
                cpu_count += 1.0;
            }
            if let Some(v) = p.cpu_memory.cpu_usage_core_nano_seconds {
                cpu_core_sum += v;
                cpu_core_count += 1.0;
            }

            // MEMORY
            if let Some(v) = p.cpu_memory.memory_usage_bytes {
                mem_sum += v;
                mem_count += 1.0;
            }
            if let Some(v) = p.cpu_memory.memory_working_set_bytes {
                mem_working_sum += v;
                mem_working_count += 1.0;
            }
            if let Some(v) = p.cpu_memory.memory_rss_bytes {
                mem_rss_sum += v;
                mem_rss_count += 1.0;
            }
            if let Some(v) = p.cpu_memory.memory_page_faults {
                mem_pf_sum += v;
                mem_pf_count += 1.0;
            }

            // FILESYSTEM (SUM)
            if let Some(fs) = &p.filesystem {
                if let Some(v) = fs.used_bytes {
                    fs_used_sum += v;
                    fs_used_count += 1.0;
                }
                if let Some(v) = fs.capacity_bytes {
                    fs_capacity_sum += v;
                    fs_capacity_count += 1.0;
                }
                if let Some(v) = fs.inodes_used {
                    fs_inodes_used_sum += v;
                    fs_inodes_used_count += 1.0;
                }
                if let Some(v) = fs.inodes {
                    fs_inodes_sum += v;
                    fs_inodes_count += 1.0;
                }
            }

            // NETWORK (SUM)
            if let Some(net) = &p.network {
                if let Some(v) = net.rx_bytes {
                    rx_sum += v;
                    rx_count += 1.0;
                }
                if let Some(v) = net.tx_bytes {
                    tx_sum += v;
                    tx_count += 1.0;
                }
                if let Some(v) = net.rx_errors {
                    rx_err_sum += v;
                    rx_err_count += 1.0;
                }
                if let Some(v) = net.tx_errors {
                    tx_err_sum += v;
                    tx_err_count += 1.0;
                }
            }
        }

        out.push(UniversalMetricPointDto {
            time,
            cpu_memory: CommonMetricValuesDto {
                cpu_usage_nano_cores: (cpu_count > 0.0).then_some(cpu_sum),
                cpu_usage_core_nano_seconds: (cpu_core_count > 0.0).then_some(cpu_core_sum),
                memory_usage_bytes: (mem_count > 0.0).then_some(mem_sum),
                memory_working_set_bytes: (mem_working_count > 0.0).then_some(mem_working_sum),
                memory_rss_bytes: (mem_rss_count > 0.0).then_some(mem_rss_sum),
                memory_page_faults: (mem_pf_count > 0.0).then_some(mem_pf_sum),
                ..Default::default()
            },
            filesystem: Some(FilesystemMetricDto {
                used_bytes: (fs_used_count > 0.0).then_some(fs_used_sum),
                capacity_bytes: (fs_capacity_count > 0.0).then_some(fs_capacity_sum),
                inodes_used: (fs_inodes_used_count > 0.0).then_some(fs_inodes_used_sum),
                inodes: (fs_inodes_count > 0.0).then_some(fs_inodes_sum),
                ..Default::default()
            }),
            network: Some(NetworkMetricDto {
                rx_bytes: (rx_count > 0.0).then_some(rx_sum),
                tx_bytes: (tx_count > 0.0).then_some(tx_sum),
                rx_errors: (rx_err_count > 0.0).then_some(rx_err_sum),
                tx_errors: (tx_err_count > 0.0).then_some(tx_err_sum),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    out
}


async fn build_namespace_cost_response(
    namespace: &str,
    mut per_pod_response: MetricGetResponseDto,
) -> Result<MetricGetResponseDto> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    apply_costs(&mut per_pod_response, &unit_prices);
    let points = aggregate_cost_points(&per_pod_response.series);

    Ok(MetricGetResponseDto {
        start: per_pod_response.start,
        end: per_pod_response.end,
        scope: "namespace".to_string(),
        target: Some(namespace.to_string()),
        granularity: per_pod_response.granularity,
        series: vec![MetricSeriesDto {
            key: namespace.to_string(),
            name: namespace.to_string(),
            scope: MetricScope::Namespace,
            points,
        }],
        total: None,
        limit: None,
        offset: None,
    })
}

async fn namespace_pods(namespace: &str) -> Result<Vec<InfoPodEntity>> {
    let pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: Some(namespace.to_string()),
        label_selector: None,
        node_name: None,
    })
    .await?;

    if pods.is_empty() {
        return Err(anyhow!("namespace '{}' has no pods", namespace));
    }

    Ok(pods)
}

pub async fn get_metric_k8s_namespaces_raw(q: RangeQuery) -> Result<Value> {
    let all_pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: None,
        label_selector: None,
        node_name: None,
    })
    .await?;

    let ns_map = group_pods_by_namespace(all_pods);
    if ns_map.is_empty() {
        return Ok(json!({ "status": "no data" }));
    }

    let mut series = Vec::new();
    let mut base_response: Option<MetricGetResponseDto> = None;

    for (ns, pods) in ns_map {
        let per_pod = build_pod_response_from_infos(q.clone(), pods, Some(ns.clone()))?;
        let aggregated = build_namespace_response(&ns, &per_pod);
        if base_response.is_none() {
            base_response = Some(aggregated.clone());
        }
        series.push(aggregated.series[0].clone());
    }

    if let Some(mut base) = base_response {
        base.target = None;
        base.series = series;
        return Ok(serde_json::to_value(base)?);
    }

    Ok(json!({ "status": "no data" }))
}

pub async fn get_metric_k8s_namespace_raw(namespace: String, q: RangeQuery) -> Result<Value> {
    let pods = namespace_pods(&namespace).await?;
    let per_pod = build_pod_response_from_infos(q, pods, Some(namespace.clone()))?;
    let aggregated = build_namespace_response(&namespace, &per_pod);
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespace_raw_summary(namespace: String, q: RangeQuery) -> Result<Value> {
    let pods = namespace_pods(&namespace).await?;
    let per_pod = build_pod_response_from_infos(q, pods.clone(), Some(namespace.clone()))?;
    let aggregated = build_namespace_response(&namespace, &per_pod);
    build_raw_summary_value(&aggregated, MetricScope::Namespace, pods.len())
}

pub async fn get_metric_k8s_namespaces_raw_summary(q: RangeQuery) -> Result<Value> {
    let all_pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: None,
        label_selector: None,
        node_name: None,
    })
    .await?;

    if all_pods.is_empty() {
        return Ok(json!({ "status": "no data" }));
    }

    let per_pod = build_pod_response_from_infos(q, all_pods.clone(), None)?;
    let aggregated = build_namespace_response("all", &per_pod);
    build_raw_summary_value(&aggregated, MetricScope::Namespace, all_pods.len())
}

pub async fn get_metric_k8s_namespace_raw_efficiency(_namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_supported",
        "message": "Namespace efficiency not supported yet"
    }))
}

pub async fn get_metric_k8s_namespaces_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_supported",
        "message": "Namespace efficiency not supported yet"
    }))
}

async fn build_namespace_cost(
    namespace: Option<String>,
    q: RangeQuery,
) -> Result<MetricGetResponseDto> {
    let pods = if let Some(ns) = namespace.clone() {
        namespace_pods(&ns).await?
    } else {
        info_k8s_pod_service::list_k8s_pods(K8sListQuery {
            namespace: None,
            label_selector: None,
            node_name: None,
        })
        .await?
    };

    if pods.is_empty() {
        return Err(anyhow!("no pods available for namespace cost calculation"));
    }

    let per_pod = build_pod_response_from_infos(q, pods, namespace.clone())?;
    Ok(build_namespace_response(namespace.as_deref().unwrap_or("all"), &per_pod))
}

pub async fn get_metric_k8s_namespaces_cost(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q).await?;
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespace_cost(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace), q).await?;
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespaces_cost_summary(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_summary_dto(&cost_response, MetricScope::Namespace, None, &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespace_cost_summary(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace.clone()), q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_summary_dto(
        &cost_response,
        MetricScope::Namespace,
        Some(namespace),
        &unit_prices,
    );
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespaces_cost_trend(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_trend_dto(&cost_response, MetricScope::Namespace, None)?;
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespace_cost_trend(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace.clone()), q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_trend_dto(
        &cost_response,
        MetricScope::Namespace,
        Some(namespace),
    )?;
    Ok(serde_json::to_value(dto)?)
}
