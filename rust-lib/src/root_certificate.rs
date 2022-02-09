pub use crate::protobuf::{
    RootIdentityCertificate, RootIdentityDocument, SignedRootIdentityCertificate,
};

pub fn create_root_identity() -> RootIdentityDocument {
    todo!()
}

pub fn create_root_certificate(
    _root_document: RootIdentityDocument,
) -> SignedRootIdentityCertificate {
    todo!()
}

pub fn verify_root_certificate(_root_certificate: RootIdentityCertificate) -> bool {
    todo!()
}
