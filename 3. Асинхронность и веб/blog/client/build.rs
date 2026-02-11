fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "proto/dto.proto",
                "proto/auth.proto",
                "proto/general.proto",
                "proto/user.proto",
                "proto/post.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
