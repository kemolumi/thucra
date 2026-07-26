pub mod check;
pub mod license;

use axum::extract::ws;
use serde_json::Value;

use crate::schema::client_request::ClientRequest;

#[derive(Debug, Clone)]
pub struct PluginCommands {}

impl PluginCommands {
    pub async fn invoke(&self, request: ClientRequest) -> Result<ws::Message, ()> {
        let payload = match request.function_id {
            6 => Ok(license::check::invoke(&request).await),
            7 => Ok(check::invoke().await),
            _ => {
                tracing::error!("Unimplemented command {}", request.function_id);
                Err(())
            }
        };

        match payload {
            Ok(payload) => Ok(self.compose_message(payload, request)),
            Err(_) => Err(()),
        }
    }

    fn compose_message(&self, payload: Value, request: ClientRequest) -> ws::Message {
        ws::Message::text(format!("{payload}*{}", request.function_callback))
    }
}
