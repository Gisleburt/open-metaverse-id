use crate::cryptography::asymmetric_crypto_alogrithm::AsymmetricCryptoAlgorithm;
use crate::cryptography::self_signature::SelfSignature;
use crate::omid_version::OmidVersion;

pub struct RootIdentityCertificate {
    pub omid_version: OmidVersion,
    pub asymmetric_crypto_algo: AsymmetricCryptoAlgorithm,
    pub public_key: Vec<u8>,
    pub not_before: i64,
    pub not_after: i64,
    pub description: String,
}

pub struct SignedRootIdentityCertificate {
    pub root_identity_certificate: RootIdentityCertificate,
    pub self_signature: SelfSignature,
}

#[cfg(test)]
mod tests {}
