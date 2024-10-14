pub const BASE_PATH: &str = env!("BASE_PATH");
pub const CHAT_API_URL: &str = env!("KONNEKTOREN_CHAT_API_URL");

pub fn full_path(path: &str) -> String {
    format!("{}{}", BASE_PATH, path)
}
