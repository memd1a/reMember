fn main() {
    println!("cargo:rustc-cdylib-link-arg=exports.def");
}