use axum::{ extract::{ WebSocketUpgrade, ws::WebSocket }, response::IntoResponse };

use crate::vnpt::{ plugin::PluginCommands, schema::client_request::ClientRequest };

async fn recv_model(socket: &mut WebSocket) -> Option<ClientRequest> {
    let Some(Ok(message)) = socket.recv().await else {
        return None;
    };

    let payload = match message.to_text() {
        Ok(payload) => payload,
        Err(_) => {
            return None;
        }
    };

    let request = match serde_json::from_str::<ClientRequest>(payload) {
        Ok(request) => request,
        Err(_) => {
            return None;
        }
    };

    Some(request)
}

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(callback)
}

async fn callback(mut socket: WebSocket) {
    let commands = PluginCommands {};

    while let Some(request) = recv_model(&mut socket).await {
        match commands.invoke(request).await {
            Ok(response) => {
                let sent = socket.send(response).await;
                tracing::info!("Server {}", sent.is_ok());
            }
            Err(_) => {
                break;
            }
        }
    }
}
