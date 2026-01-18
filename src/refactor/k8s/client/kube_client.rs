use anyhow::Result;
use http::Uri;
use kube::{Client, config::Config};
use std::{env, fs};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

/// Load a token from dev override or in-cluster service account
fn load_token() -> Option<String> {
    // DEV override via .env
    if let Ok(path) = env::var("RUSTCOST_TOKEN_PATH") {
        if let Ok(token) = fs::read_to_string(path) {
            return Some(token.trim().to_string());
        }
    }

    // PROD token mounted inside the pod
    let sa_token_path = "/var/run/secrets/kubernetes.io/serviceaccount/token";
    fs::read_to_string(sa_token_path)
        .ok()
        .map(|t| t.trim().to_string())
}

/// Load CA cert from dev override or in-cluster service account
fn load_ca_bytes() -> Option<Vec<u8>> {
    // 1. DEV override (ca.der or ca.crt)
    if let Ok(path) = env::var("RUSTCOST_CA_PATH") {
        if let Ok(bytes) = fs::read(&path) {

            // Check if it's DER already (common for Windows dev mode)
            if path.ends_with(".der") {
                return Some(bytes);
            }

            // Otherwise assume PEM and convert to DER
            if let Ok(cert) = pem_to_der(&bytes) {
                return Some(cert);
            }

            // If PEM conversion failed, return raw bytes (rustls may accept if clean PEM)
            return Some(bytes);
        }
    }

    // 2. PROD (in-cluster) likely PEM from service account
    let sa_path = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
    if let Ok(bytes) = fs::read(sa_path) {
        // Convert PEM → DER for rustls
        if let Ok(cert) = pem_to_der(&bytes) {
            return Some(cert);
        }
        return Some(bytes); // fallback
    }

    None
}

fn pem_to_der(pem: &[u8]) -> Result<Vec<u8>, ()> {
    let pem_str = std::str::from_utf8(pem).map_err(|_| ())?;

    // Extract only the certificate section (rustls expects clean DER)
    let begin = "-----BEGIN CERTIFICATE-----";
    let end = "-----END CERTIFICATE-----";

    let start = pem_str.find(begin).ok_or(())? + begin.len();
    let stop = pem_str.find(end).ok_or(())?;

    let base64_data = pem_str[start..stop]
        .trim()
        .replace("\r", "")
        .replace("\n", "");

    STANDARD.decode(base64_data).map_err(|_| ())
}

/// Build Kubernetes client for both DEV (Windows) and PROD (Linux)
pub async fn build_kube_client() -> Result<Client> {
    // Auto-detect: use kubeconfig on Windows, in-cluster config in prod
    let mut config = Config::infer().await?;

    //
    // WINDOWS DEV MODE: SSH tunnel or kubectl port-forward
    //
    if cfg!(windows) {
        // Default API tunnel: https://127.0.0.1:6443
        let tunnel_url = env::var("RUSTCOST_TUNNEL_URL")
            .unwrap_or_else(|_| "https://127.0.0.1:6443".to_string());

        // Override cluster API URL
        config.cluster_url = tunnel_url.parse::<Uri>()?;

        // Tunnels never match TLS hostname → allow invalid certs
        config.accept_invalid_certs = true;

        // Disable hostname checking (CN mismatch)
        config.tls_server_name = None;
    }

    //
    // TOKEN injection (manual or file)
    //
    if let Some(token) = load_token() {
        config.auth_info.token = Some(token.into());
    } else if let Ok(path) = env::var("RUSTCOST_TOKEN_PATH") {
        // fallback: let kube-rs read the file itself
        config.auth_info.token_file = Some(path);
    }

    //
    // CA certificate injection
    //
    if let Some(ca_bytes) = load_ca_bytes() {
        config.root_cert = Some(vec![ca_bytes]);
    }

    //
    // Build the client (kube-rs 2.x)
    //
    let client = Client::try_from(config)?;
    Ok(client)
}
