use std::env;

fn main() {
    let base_path = env::var("BASE_PATH").unwrap_or_else(|_| String::from(""));
    println!("cargo:rustc-env=BASE_PATH={}", base_path);
}
