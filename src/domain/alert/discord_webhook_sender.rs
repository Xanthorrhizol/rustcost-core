use anyhow::{anyhow, Result};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use tracing::{debug, warn};

use crate::core::persistence::info::fixed::alerts::alert_rule_entity::{AlertRuleEntity, AlertSeverity};

pub struct DiscordWebhookSender {
    client: Client,
}

impl Default for DiscordWebhookSender {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl DiscordWebhookSender {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Sends an alert to Discord using embeds and retries on non-2xx responses.
    pub async fn send(&self, webhook_url: &str, rule: &AlertRuleEntity, message: &str) -> Result<()> {
        let payload = DiscordWebhookPayload {
            content: None,
            embeds: vec![DiscordEmbed {
                title: rule.name.clone(),
                description: Some(message.to_string()),
                color: Self::color_for(&rule.severity),
            }],
        };

        self.post_with_retry(webhook_url, &payload, 2).await
    }

    async fn post_with_retry(
        &self,
        webhook_url: &str,
        payload: &DiscordWebhookPayload,
        attempts: usize,
    ) -> Result<()> {
        let mut last_status: Option<StatusCode> = None;

        for attempt in 1..=attempts {
            let resp = self.client.post(webhook_url).json(payload).send().await?;
            let status = resp.status();
            debug!(attempt, status = ?status, "discord_webhook_response");
            if status.is_success() {
                // Discord returns 204 on success; any 2xx is accepted.
                return Ok(());
            }

            // Capture a small error body to aid debugging without logging the URL.
            let body = resp.text().await.unwrap_or_default();
            warn!(
                attempt,
                status = ?status,
                body = %body,
                "discord_webhook_non_success"
            );
            last_status = Some(status);
        }

        Err(anyhow!(
            "Discord webhook failed after retries (last status: {:?})",
            last_status
        ))
    }

    fn color_for(severity: &AlertSeverity) -> u32 {
        match severity {
            AlertSeverity::Info => 0x3498db,
            AlertSeverity::Warning => 0xf1c40f,
            AlertSeverity::Critical => 0xe74c3c,
        }
    }
}

#[derive(Serialize)]
struct DiscordWebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize)]
struct DiscordEmbed {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    color: u32,
}
