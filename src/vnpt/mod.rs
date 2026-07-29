use std::{ net::SocketAddr, sync::Arc, time::Duration };

use axum::{ Router, routing::get };
use axum_server::tls_rustls::RustlsConfig;

use crate::{ consts, vnpt::plugin::PluginCommands };

pub mod token;
pub mod schema;
pub mod routes;
pub mod plugin;

pub struct AppState {
    pub plugin: PluginCommands,
}

/// Implementation of VNPT CA Plugin.
///
/// The impl requires a task for reading `control` to response to.
pub struct VnptCa {
    tls_config: RustlsConfig,
}

impl VnptCa {
    pub async fn new() -> Self {
        let home = std::env::home_dir().unwrap();

        let app_folder = format!(
            "{}/.local/share/com.kemolumi.thucra",
            home.as_os_str().to_str().unwrap()
        );

        let (key_location, cert_location) = (
            format!("{app_folder}/certs/key.pem"),
            format!("{app_folder}/certs/cert.pem"),
        );

        let tls_config = RustlsConfig::from_pem_file(cert_location, key_location).await.unwrap();

        VnptCa {
            tls_config,
        }
    }

    pub async fn launch(&mut self) {
        loop {
            match self.start().await {
                Ok(_) => {}
                Err(()) => {
                    tracing::error!(
                        "Service failed to launch. Waits a bit before checking again..."
                    );
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }
        }
    }

    async fn start(&mut self) -> Result<(), ()> {
        tracing::info!("Starting service...");

        for port in consts::VNPT_CA_PORT_RANGE {
            tracing::info!("Hello, world! (world here is 127.0.0.1:{}) :3", port);

            let server = axum_server::bind_rustls(
                format!("127.0.0.1:{port}").parse::<SocketAddr>().unwrap(),
                self.tls_config.clone()
            );

            match server.serve(VnptCa::app().await.into_make_service()).await {
                Ok(_) => {
                    tracing::warn!("VNPT CA Plugin stopped.");
                    return Ok(());
                }
                Err(_) => {
                    tracing::warn!("Port {port} is used. :(");
                    continue;
                }
            }
        }

        tracing::error!("Failed to start VNPT CA service.");
        return Err(());
    }

    async fn app() -> Router {
        let app_state = Arc::new(AppState { plugin: PluginCommands {} });

        Router::new().route("/plugin", get(routes::plugin::handler)).with_state(app_state)
    }
}
