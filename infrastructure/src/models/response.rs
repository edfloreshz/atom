use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgrestResponse {
    code: String,
    details: String,
    hint: Option<String>,
    message: String,
}
