use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub author: String,
    pub category: Option<String>,
    pub description: String,
    pub id: u64,
    pub merge_request_id: u64,
    pub sub_category: Option<String>,
    pub url: String,
}
