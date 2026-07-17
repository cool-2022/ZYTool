use serde::{Deserialize, Serialize};

use crate::models::base::BaseResponse;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimestampAction {
    ToDatetime,
    ToTimestamp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampConvertRequest {
    pub timestamp: i64,
    pub action: TimestampAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampConvertResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub result: String,
    pub timestamp: i64,
    pub action: String,
}
