use serde_json::Value;

use crate::vnpt::schema::{ client_request::ClientRequest, server_response::ServerJsonResponse };

pub async fn invoke(request: &ClientRequest) -> Value {
    if request.args.len() == 0 || request.args[0].len() == 0 {
        return ServerJsonResponse::new(-1, b"", "License not found".to_owned()).to_value();
    }

    ServerJsonResponse::new(1, b"", "Set license successfully".to_owned()).to_value()
}
