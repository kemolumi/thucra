use base64::{ Engine, engine::general_purpose };
use serde::{ Deserialize, Serialize };
use serde_json::Value;

type Base64Data = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerJsonResponse {
    code: i32,
    data: Base64Data,
    #[serde(rename = "error")]
    description: String,
}
impl ServerJsonResponse {
    pub fn new<T: AsRef<[u8]>>(code: i32, data: T, description: String) -> Self {
        let data: Base64Data = general_purpose::STANDARD.encode(data);

        ServerJsonResponse { code, data, description }
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
