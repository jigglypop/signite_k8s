fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/auth.proto");
    
    tonic_build::compile_protos("protos/auth.proto")?;
    
    Ok(())
} 