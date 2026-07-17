use crate::core::error::{bad_request, AppResult};
use crate::models::{CompareSummary, LineDifference, TextAction};
use base64::prelude::*;

pub fn process_text(action: &TextAction, text: &str) -> AppResult<String> {
    match action {
        TextAction::JsonFormat => {
            let data: serde_json::Value = serde_json::from_str(text)
                .map_err(|e| bad_request(format!("无效的JSON格式: {}", e)))?;
            Ok(serde_json::to_string_pretty(&data).unwrap_or_default())
        }
        TextAction::Base64Encode => Ok(BASE64_STANDARD.encode(text.as_bytes())),
        TextAction::Base64Decode => {
            let bytes = BASE64_STANDARD
                .decode(text)
                .map_err(|e| bad_request(format!("Base64解码失败: {}", e)))?;
            String::from_utf8(bytes).map_err(|e| bad_request(format!("Base64解码失败: {}", e)))
        }
        TextAction::UrlEncode => Ok(urlencoding::encode(text).to_string()),
        TextAction::UrlDecode => {
            urlencoding::decode(text)
                .map(|s| s.to_string())
                .map_err(|e| bad_request(format!("URL解码失败: {}", e)))
        }
    }
}

pub fn compare_text(text1: &str, text2: &str) -> (Vec<LineDifference>, CompareSummary) {
    let lines1: Vec<&str> = text1.lines().collect();
    let lines2: Vec<&str> = text2.lines().collect();
    let max_lines = lines1.len().max(lines2.len());
    let mut differences = Vec::new();

    for i in 0..max_lines {
        let line1 = lines1.get(i).unwrap_or(&"").to_string();
        let line2 = lines2.get(i).unwrap_or(&"").to_string();

        if line1 != line2 {
            let diff_type = if line1.is_empty() {
                "added"
            } else if line2.is_empty() {
                "removed"
            } else {
                "modified"
            };
            differences.push(LineDifference {
                line: i + 1,
                text1: line1,
                text2: line2,
                diff_type: diff_type.to_string(),
            });
        }
    }

    let summary = CompareSummary {
        total_lines: max_lines,
        different_lines: differences.len(),
        identical: differences.is_empty(),
    };

    (differences, summary)
}
