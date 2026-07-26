use serde::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientRequest {
    pub args: Vec<String>,
    pub domain: String,
    #[serde(rename = "funcCallback")]
    pub function_callback: String,
    #[serde(rename = "functionID")]
    pub function_id: i32,
}
