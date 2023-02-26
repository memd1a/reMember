fn main() {
    let link_arg = if cfg!(target_env = "msvc") {
        "/DEF:exports.def"
    } else {
        "exports.def"
    };


    println!("cargo:rustc-cdylib-link-arg={}", link_arg);
}