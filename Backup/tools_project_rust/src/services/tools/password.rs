use crate::core::error::{bad_request, AppResult};
use rand::{rng, Rng};

const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn generate_password(
    length: usize,
    include_symbols: bool,
    include_numbers: bool,
    include_uppercase: bool,
    include_lowercase: bool,
) -> AppResult<String> {
    let mut chars = String::new();
    if include_lowercase {
        chars.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if include_uppercase {
        chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_numbers {
        chars.push_str("0123456789");
    }
    if include_symbols {
        chars.push_str(SYMBOLS);
    }

    if chars.is_empty() {
        return Err(bad_request("至少需要选择一种字符类型"));
    }

    let char_vec: Vec<char> = chars.chars().collect();
    let mut rng = rng();
    let password: String = (0..length)
        .map(|_| char_vec[rng.random_range(0..char_vec.len())])
        .collect();

    Ok(password)
}
