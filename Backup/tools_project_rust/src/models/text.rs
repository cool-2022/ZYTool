use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAction {
    JsonFormat,
    Base64Encode,
    Base64Decode,
    UrlEncode,
    UrlDecode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextProcessRequest {
    pub text: String,
    pub action: TextAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextProcessResponse {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextCompareRequest {
    pub text1: String,
    pub text2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineDifference {
    pub line: usize,
    pub text1: String,
    pub text2: String,
    #[serde(rename = "type")]
    pub diff_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareSummary {
    pub total_lines: usize,
    pub different_lines: usize,
    pub identical: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextCompareResponse {
    pub differences: Vec<LineDifference>,
    pub summary: CompareSummary,
}
