use crate::core::error::{bad_request, AppResult};
use crate::models::MatchDetail;
use regex::Regex;

pub fn test_regex(pattern: &str, text: &str, _flags: Option<&str>) -> AppResult<(Vec<String>, Vec<MatchDetail>)> {
    let re = Regex::new(pattern).map_err(|e| bad_request(format!("正则表达式错误: {}", e)))?;

    let matches: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();
    let match_details: Vec<MatchDetail> = re
        .captures_iter(text)
        .map(|caps| {
            let m = caps.get(0).unwrap();
            let groups: Vec<String> = caps
                .iter()
                .skip(1)
                .map(|g| g.map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect();
            MatchDetail {
                match_text: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                groups,
            }
        })
        .collect();

    Ok((matches, match_details))
}
