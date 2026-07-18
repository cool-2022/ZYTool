use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexTestRequest {
    pub pattern: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchDetail {
    #[serde(rename = "match")]
    pub match_text: String,
    pub start: usize,
    pub end: usize,
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexTestResponse {
    pub matches: Vec<String>,
    pub match_count: usize,
    pub match_details: Vec<MatchDetail>,
}
