fn main() {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATE={date}");
    tauri_build::build();
}
