extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "../contracts/Cryptography.proto",
            "../contracts/IntermediateIdentity.proto",
            "../contracts/OpenMetaId.proto",
            "../contracts/RootIdentity.proto",
        ],
        &["../contracts/"],
    )
    .unwrap();
}
