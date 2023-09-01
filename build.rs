// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::compile_protos("protos/uranium.proto")?;
//     Ok(())
// }

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/uranium.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        // .build_client(true) // don't compile the client code
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
        .out_dir("./app/src/common")
        .compile(&[proto_file], &["proto"])?;

    Ok(())
}
