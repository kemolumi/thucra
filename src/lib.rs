pub mod consts;
pub mod routes;
pub mod plugin;
pub mod schema;

use std::{ path::Path, sync::Arc, time::Instant };

use axum::{ Router, routing::get };
use axum_server::tls_rustls::RustlsConfig;
use tokio::{ fs::create_dir, process };

pub struct AppState {}

/// Must be called first for any interaction with the router.
pub async fn init() -> Result<(), ()> {
    tracing_subscriber::fmt().init();
    rustls::crypto::ring::default_provider().install_default().unwrap();

    tracing::info!("Checking SSL store...");

    match Path::new("store/key.pem").exists() && Path::new("store/cert.pem").exists() {
        true => {
            tracing::info!("SSL found. Launching server...");
            return Ok(());
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

    return Ok(());
}

/// The core setup, this is what `main.rs` should call.
pub async fn core() {
    tracing::info!("Hello, world (world here is {})! :3", *consts::HOST);

    tracing::info!("Serving in secure context...");
    let tls_config = RustlsConfig::from_pem_file("store/cert.pem", "store/key.pem").await.unwrap();
    axum_server
        ::bind_rustls(*consts::HOST, tls_config)
        .serve(app().await.into_make_service()).await
        .unwrap();
}

/// Creates an app.
pub async fn app() -> Router {
    tracing::info!("Initializing server state...");
    let boot_time = Instant::now();

    let app_state = Arc::new(AppState {});

    tracing::info!(
        "Server started succesfully. (Boot time: {}ms)",
        boot_time.elapsed().as_millis()
    );

    Router::new().route("/plugin", get(routes::plugin::handler))
}
