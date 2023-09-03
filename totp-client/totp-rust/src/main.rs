use std::time::{SystemTime, UNIX_EPOCH};

use data_encoding::BASE32_NOPAD;
use ring::hmac::{self, Context};

fn main() {
    let dummy_key: &str = "CQAPZTCGBILT3O4NOKXQVB4EZC7HZ77BQBREBDSQKUSVYT2SOHS64TYK325NZOVNP222CXSWCKM73LPGOPB3MLPQEOEPNBAXKAHYPKY";
    let key = decode_key(dummy_key).unwrap_or_else(|err| {
        println!("Error decoding the key: {}", err);
        std::process::exit(1);
    });

    let ts = get_timestamp().unwrap_or_else(|| {
        println!("Error getting the timestamp");
        std::process::exit(1);
    });

    let totp_code = get_totp_code(&get_hmac(&key, &ts));

    println!("Current TOTP code: {:06}", totp_code);
}

fn decode_key(secret_key: &str) -> Result<Vec<u8>, data_encoding::DecodeError> {
    let key = secret_key.trim().to_uppercase();
    BASE32_NOPAD.decode(key.as_bytes())
}

fn get_timestamp() -> Option<[u8; 8]> {
    let mut time_bytes = [0u8; 8];
    let now = SystemTime::now().duration_since(UNIX_EPOCH);

    if let Ok(now_duration) = now {
        time_bytes.copy_from_slice(&(now_duration.as_secs() / 30).to_be_bytes());
        return Some(time_bytes);
    }
    None
}

fn get_hmac(secret_bytes: &[u8], time_bytes: &[u8]) -> Vec<u8> {
    let hash = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret_bytes);
    let mut hmac_context = Context::with_key(&hash);
    hmac_context.update(time_bytes);

    hmac_context.sign().as_ref().to_vec()
}

fn get_totp_code(hash: &[u8]) -> u32 {
    let offset = (hash[hash.len() - 1] & 0x0F) as usize;
    let bytes = &hash[offset..];

    let truncated = bytes
        .iter()
        .fold(0u32, |acc, &byte| (acc << 8) | byte as u32)
        & 0x7FFFFFFF;

    truncated % 1_000_000
}
