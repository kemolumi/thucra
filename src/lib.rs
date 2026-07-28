pub mod consts;
pub mod vnpt;
pub mod app;

use std::path::Path;
use tokio::{ fs::create_dir, process };
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };

/// Must be called first for any interaction with the router.
pub async fn init() -> Result<Option<WorkerGuard>, ()> {
    let mut guard = None;
    match std::env::home_dir() {
        Some(home) => {
            let file_appender = tracing_appender::rolling::daily(
                format!("{}/.local/share/thucra/logs", home.as_os_str().to_str().unwrap()),
                "thucra.log"
            );
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
            guard = Some(_guard);

            tracing_subscriber::Registry
                ::default()
                .with(tracing_subscriber::fmt::layer().with_ansi(false).with_writer(non_blocking))
                .with(tracing_subscriber::fmt::layer())
                .init();
        }
        None => {
            eprintln!("No use home folder found, fallback to console logging.");
            tracing_subscriber::fmt::init();
        }
    }

    rustls::crypto::ring::default_provider().install_default().unwrap();

    tracing::info!("Checking SSL store...");

    match Path::new("store/key.pem").exists() && Path::new("store/cert.pem").exists() {
        true => {
            tracing::info!("SSL found. Launching server...");
            return Ok(guard);
        }
        false => {
            tracing::warn!("No SSL store found, generating new ones...");
        }
    }

    match create_dir("store").await {
        Ok(_) => {}
        Err(error) => {
            tracing::error!("Can't create directory: {error}");
        }
    }

    let status = process::Command
        ::new("openssl")
        .args([
            "req",
            "-x509",
            "-newkey",
            "rsa:4096",
            "-sha256",
            "-days",
            "3650",
            "-nodes",
            "-keyout",
            "store/key.pem",
            "-out",
            "store/cert.pem",
            "-subj",
            "/CN=localhost",
            "-addext",
            "subjectAltName=DNS:localhost,IP:127.0.0.1",
        ])
        .output().await;

    let success = match status {
        Ok(status) => status.status.success(),
        Err(error) => {
            tracing::error!("Can't create SSL certificates: {error}");
            return Err(());
        }
    };

    match success {
        true => {
            tracing::info!(
                "New SSL store created. This operation shouldn't happens more than once."
            );
        }
        false => {
            tracing::error!("`openssl` returns a non-success exit code.");
            return Err(());
        }
    }

    return Ok(guard);
}
