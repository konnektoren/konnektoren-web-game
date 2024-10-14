use std::env;

fn main() {
    let base_path = env::var("BASE_PATH").unwrap_or_else(|_| String::from(""));
    let chat_api_url = env::var("KONNEKTOREN_CHAT_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1/chat"));
    let reviews_api_url = env::var("REVIEWS_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1/reviews"));
    let ton_api_url = env::var("KONNEKTOREN_TON_API_URL")
        .unwrap_or_else(|_| String::from("https://testnet.tonapi.io/v2"));
    let tonconnect_manifest_url =
        env::var("KONNEKTOREN_TONCONNECT_MANIFEST_URL").unwrap_or_else(|_| {
            String::from("https://konnektoren.help/assets/tonconnect-manifest.json")
        });
    let payment_address = env::var("KONNEKTOREN_PAYMENT_ADDRESS").unwrap_or_else(|_| {
        String::from("0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119")
    });
    println!("cargo:rustc-env=BASE_PATH={}", base_path);
    println!("cargo:rustc-env=KONNEKTOREN_CHAT_API_URL={}", chat_api_url);
    println!("cargo:rustc-env=REVIEWS_API_URL={}", reviews_api_url);
    println!("cargo:rustc-env=KONNEKTOREN_TON_API_URL={}", ton_api_url);
    println!(
        "cargo:rustc-env=KONNEKTOREN_TONCONNECT_MANIFEST_URL={}",
        tonconnect_manifest_url
    );
    println!(
        "cargo:rustc-env=KONNEKTOREN_PAYMENT_ADDRESS={}",
        payment_address
    );
}
