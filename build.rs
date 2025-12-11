// will act as a build script for cargo

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/odin.proto")?;
    Ok(())
}
