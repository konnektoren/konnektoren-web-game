use std::env;

fn main() {
    let base_path = env::var("BASE_PATH").unwrap_or_else(|_| String::from(""));
    let chat_api_url = env::var("KONNEKTOREN_CHAT_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1/chat"));
    println!("cargo:rustc-env=BASE_PATH={}", base_path);
    println!("cargo:rustc-env=KONNEKTOREN_CHAT_API_URL={}", chat_api_url);
}
