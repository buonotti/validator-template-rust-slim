use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct EndpointResponse {
    status_code: i32,
    raw_data: Value,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ValidationItem {
    response: EndpointResponse,
}
