use tonic_build::compile_protos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/")
        .build_client(false)
        .protoc_arg("--experimental_allow_proto3_optional");
    compile_protos("proto/auth.proto")?;
    compile_protos("proto/accounts.proto")?;
    compile_protos("proto/chat.proto")?;
    compile_protos("proto/media.proto")?;
    compile_protos("proto/timeline.proto")?;
    compile_protos("proto/user.proto")?;

    Ok(())
}
