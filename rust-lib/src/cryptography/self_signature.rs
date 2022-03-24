use crate::cryptography::signature_alogrithm::SignatureAlgorithm;

pub struct SelfSignature {
    pub signature_algo: SignatureAlgorithm,
    pub signature: Vec<u8>,
}
