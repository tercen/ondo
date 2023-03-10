fn main() {
    println!("cargo:rerun-if-changed=./proto");
    tonic_build::compile_protos("./proto/hello.proto").unwrap();
    tonic_build::compile_protos("./proto/ondo.proto").unwrap();
    cargo_emit::rerun_if_changed!(
        "./proto/hello.proto",
        "./proto/ondo.proto",
        "Cargo.toml",
        "Cargo.lock",
        "build.rs"
    );
}
