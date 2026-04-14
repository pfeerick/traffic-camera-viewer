fn main() {
    // Inject BUILD_DATE as a compile-time env var. No rerun-if-changed guard
    // so the date is always current — Cargo re-runs this script on every build.
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATE={date}");
}
