use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordGenerateRequest {
    #[serde(default = "default_length")]
    pub length: usize,
    #[serde(default = "default_true")]
    pub include_symbols: bool,
    #[serde(default = "default_true")]
    pub include_numbers: bool,
    #[serde(default = "default_true")]
    pub include_uppercase: bool,
    #[serde(default = "default_true")]
    pub include_lowercase: bool,
}

fn default_length() -> usize { 12 }
fn default_true() -> bool { true }

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTypes {
    pub lowercase: bool,
    pub uppercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordGenerateResponse {
    pub password: String,
    pub length: usize,
    pub character_types: CharacterTypes,
}
