fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-changed=proto/user.proto");
    println!("cargo:rerun-if-changed=proto/post.proto");
    println!("cargo:rerun-if-changed=build.rs");

    // tonic_prost_build::configure()
    //     .build_server(true)
    //     .compile_protos(&["proto/user.proto", "proto/post.proto"], &["proto"])?;
    Ok(())
}
