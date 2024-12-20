use std::env;

fn main() {
    let base_path = env::var("BASE_PATH").unwrap_or_else(|_| String::from(""));
    let chat_api_url = env::var("KONNEKTOREN_CHAT_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1/chat"));
    let ton_api_url = env::var("KONNEKTOREN_TON_API_URL")
        .unwrap_or_else(|_| String::from("https://testnet.tonapi.io/v2"));
    let tonconnect_manifest_url =
        env::var("KONNEKTOREN_TONCONNECT_MANIFEST_URL").unwrap_or_else(|_| {
            String::from("https://konnektoren.help/assets/tonconnect-manifest.json")
        });
    let payment_address = env::var("KONNEKTOREN_PAYMENT_ADDRESS").unwrap_or_else(|_| {
        String::from("0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119")
    });
    let konnektoren_v1_api_url = env::var("KONNEKTOREN_V1_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1"));

    let google_client_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| {
        String::from("60985549793-nhej48uob30k5lbq6rvppbme67fdc7u9.apps.googleusercontent.com")
    });
    let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| String::from("http://localhost:8080/backup"));

    println!("cargo:rustc-env=BASE_PATH={}", base_path);
    println!("cargo:rustc-env=KONNEKTOREN_CHAT_API_URL={}", chat_api_url);
    println!("cargo:rustc-env=KONNEKTOREN_TON_API_URL={}", ton_api_url);
    println!(
        "cargo:rustc-env=KONNEKTOREN_TONCONNECT_MANIFEST_URL={}",
        tonconnect_manifest_url
    );
    println!(
        "cargo:rustc-env=KONNEKTOREN_PAYMENT_ADDRESS={}",
        payment_address
    );
    println!(
        "cargo:rustc-env=KONNEKTOREN_V1_API_URL={}",
        konnektoren_v1_api_url
    );
    println!("cargo:rustc-env=GOOGLE_CLIENT_ID={}", google_client_id);
    println!(
        "cargo:rustc-env=GOOGLE_REDIRECT_URI={}",
        google_redirect_uri
    );
}
