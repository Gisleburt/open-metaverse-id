use crate::cryptography::signature_alogrithm::SignatureAlgorithm;
use crate::omid_version::OmidVersion;

pub struct Signature {
    pub omid_version: OmidVersion,
    pub signature_algo: SignatureAlgorithm,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}
