fn main() {
    println!("cargo:rerun-if-changed=./proto");
    tonic_build::compile_protos("./proto/hello.proto").unwrap();
    tonic_build::compile_protos("./proto/ondo_remote.proto").unwrap();
    cargo_emit::rerun_if_changed!(
        "./proto/hello.proto",
        "./proto/ondo_remote.proto",
        "Cargo.toml",
        "Cargo.lock",
        "build.rs"
    );
}
