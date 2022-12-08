
fn main() {
    let kernel = std::env::var_os("CARGO_BIN_FILE_BINARY_DEPS_binary_deps").unwrap().into_string().unwrap();
    let kernel = std::path::Path::new(&kernel);

    println!("cargo:rustc-env=BINDEPS_ENV={}", kernel.display());
}