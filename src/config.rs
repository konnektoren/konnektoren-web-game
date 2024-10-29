pub const BASE_PATH: &str = env!("BASE_PATH");
pub const CHAT_API_URL: &str = env!("KONNEKTOREN_CHAT_API_URL");
pub const REVIEWS_API_URL: &str = env!("KONNEKTOREN_REVIEWS_API_URL");
pub const TON_API_URL: &str = env!("KONNEKTOREN_TON_API_URL");
pub const TONCONNECT_MANIFEST_URL: &str = env!("KONNEKTOREN_TONCONNECT_MANIFEST_URL");
pub const PAYMENT_ADDRESS: &str = env!("KONNEKTOREN_PAYMENT_ADDRESS");
pub const V1_API_URL: &str = env!("KONNEKTOREN_V1_API_URL");

pub fn full_path(path: &str) -> String {
    format!("{}{}", BASE_PATH, path)
}
