use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use sha2::{Digest, Sha256};

use crate::core::config::SETTINGS;

const VERSION_PREFIX: &str = "v1:";

/// 由 SECRET_KEY 派生 32 字节 AES-256 密钥
fn cipher() -> Aes256Gcm {
    let key = Sha256::digest(SETTINGS.secret_key.as_bytes());
    Aes256Gcm::new_from_slice(&key).expect("AES-256 key 长度必须为 32 字节")
}

/// 加密为 `v1:base64(nonce + 密文)` 格式
pub fn encrypt(plaintext: &str) -> String {
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    rand::rng().fill_bytes(&mut nonce_bytes);

    let ciphertext = cipher()
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_bytes())
        .expect("加密失败");

    let mut buf = nonce_bytes.to_vec();
    buf.extend_from_slice(&ciphertext);
    format!("{}{}", VERSION_PREFIX, B64.encode(buf))
}

/// 解密；非 `v1:` 前缀视为明文原样返回（兼容历史数据）
pub fn decrypt(stored: &str) -> Result<String, String> {
    if !stored.starts_with(VERSION_PREFIX) {
        return Ok(stored.to_string());
    }

    let buf = B64
        .decode(&stored[VERSION_PREFIX.len()..])
        .map_err(|e| format!("base64 解码失败: {}", e))?;
    if buf.len() < 13 {
        return Err("密文长度非法".to_string());
    }

    let (nonce_bytes, ciphertext) = buf.split_at(12);
    let plaintext = cipher()
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| "解密失败（密钥不匹配或数据损坏）".to_string())?;

    String::from_utf8(plaintext).map_err(|e| format!("明文编码非法: {}", e))
}

/// 是否已是加密格式
#[allow(dead_code)]
pub fn is_encrypted(stored: &str) -> bool {
    stored.starts_with(VERSION_PREFIX)
}
