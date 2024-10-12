pub const BASE_PATH: &str = env!("BASE_PATH");

pub fn full_path(path: &str) -> String {
    format!("{}{}", BASE_PATH, path)
}
