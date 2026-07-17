use crate::core::error::{bad_request, AppResult};
use crate::models::TimestampAction;
use chrono::TimeZone;

pub fn convert_timestamp(timestamp: i64, action: &TimestampAction) -> AppResult<(String, String)> {
    match action {
        TimestampAction::ToDatetime => {
            let dt = chrono::Local.timestamp_opt(timestamp, 0).single()
                .ok_or_else(|| bad_request("无效的时间戳"))?;
            Ok((dt.format("%Y-%m-%d %H:%M:%S").to_string(), "to_datetime".to_string()))
        }
        TimestampAction::ToTimestamp => {
            Ok((timestamp.to_string(), "to_timestamp".to_string()))
        }
    }
}
