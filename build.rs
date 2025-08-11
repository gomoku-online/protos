use glob::glob;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/");

    let proto_files: Vec<PathBuf> = glob("protos/**/*.proto")?
        .filter_map(Result::ok)
        .collect();

    if proto_files.is_empty() {
        println!("cargo:warning=No proto files found. Skipping protoc generation.");
        return Ok(());
    }

    let include_paths = &[PathBuf::from("protos/")];

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &proto_files,
            include_paths,
        )?;

    Ok(())
}