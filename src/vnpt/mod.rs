use std::{ net::SocketAddr, sync::{ Arc } };

use axum::{ Router, routing::get };
use axum_server::{ Handle, tls_rustls::RustlsConfig };
use tokio::sync::{ Mutex, mpsc };

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
    vnpt_ca_restart_stopper: Arc<Mutex<mpsc::Receiver<()>>>,
    handle: Arc<Mutex<Handle<SocketAddr>>>,
    tls_config: RustlsConfig,
}

impl VnptCa {
    pub async fn new(vnpt_ca_restart_stopper: Arc<Mutex<mpsc::Receiver<()>>>) -> Self {
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
            vnpt_ca_restart_stopper,
            handle: Arc::new(Mutex::new(Handle::new())),
            tls_config,
        }
    }

    pub async fn launch(&mut self) {
        let owned_stopper = self.vnpt_ca_restart_stopper.clone();
        let handle = self.handle.clone();

        // Wait for tripping message to destroy the server.
        tokio::task::spawn(async move {
            let mut owned_stopper = owned_stopper.lock().await;
            while owned_stopper.recv().await.is_some() {
                handle.lock().await.shutdown();
            }
        });

        // The handle destroyed the server, recover right away.
        loop {
            match self.start().await {
                Ok(_) => {},
                Err(()) => {
                    tracing::error!("Service failed to launch.");
                }
            }
        }
    }

    async fn start(&mut self) -> Result<(), ()> {
        tracing::info!("Starting service...");

        for port in consts::VNPT_CA_PORT_RANGE {
            tracing::info!("Hello, world! (world here is 127.0.0.1:{}) :3", port);

            let handle = Handle::new();

            {
                let mut owned_handle = self.handle.lock().await;
                *owned_handle = handle.clone();
            }

            let server = axum_server
                ::bind_rustls(
                    format!("127.0.0.1:{port}").parse::<SocketAddr>().unwrap(),
                    self.tls_config.clone()
                )
                .handle(handle);

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
