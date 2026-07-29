pub mod consts;
pub mod vnpt;
pub mod app;

use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };

/// Must be called first for any interaction with the router.
pub fn init() -> Result<WorkerGuard, ()> {
    let Some(home) = std::env::home_dir() else {
        tracing::error!("No user home folder found.");
        return Err(());
    };

    let app_folder = format!(
        "{}/.local/share/com.kemolumi.thucra",
        home.as_os_str().to_str().unwrap()
    );

    let file_appender = tracing_appender::rolling::daily(
        format!("{app_folder}/logs"),
        "thucra.log"
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::Registry
        ::default()
        .with(tracing_subscriber::fmt::layer().with_ansi(false).with_writer(non_blocking))
        .with(tracing_subscriber::fmt::layer())
        .init();

    rustls::crypto::ring::default_provider().install_default().unwrap();

    tracing::info!("Checking SSL store...");

    let (key_location, cert_location) = (
        format!("{app_folder}/certs/key.pem"),
        format!("{app_folder}/certs/cert.pem"),
    );

    match Path::new(&key_location).exists() && Path::new(&cert_location).exists() {
        true => {
            tracing::info!("SSL found.");
            return Ok(_guard);
        }
        false => {
            tracing::warn!("No SSL store found, generating new ones...");
        }
    }

    match std::fs::create_dir(format!("{app_folder}/certs")) {
        Ok(_) => {}
        Err(error) => {
            tracing::error!("Can't create directory: {error}");
        }
    }

    let status = std::process::Command
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
            &key_location,
            "-out",
            &cert_location,
            "-subj",
            "/CN=localhost",
            "-addext",
            "subjectAltName=DNS:localhost,IP:127.0.0.1",
        ])
        .output();

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

    return Ok(_guard);
}
