// use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct EventManagerStatusResponse {
    pub request_status: bool,
    pub request_msg: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct EventManagerStatusRequest {
    pub query: String,
}
