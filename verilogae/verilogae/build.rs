// Alternative to the build.rs solution, the following command can be used to build the project:
// RUSTFLAGS="-C linker=ld -C link-arg=-L/opt/homebrew/Cellar/zstd/1.5.5/lib" cargo build --target aarch64-apple-darwin

fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/zstd/1.5.5/lib");
}
