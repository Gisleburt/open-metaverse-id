use crate::cryptography::asymmetric_crypto_alogrithm::AsymmetricCryptoAlgorithm;
use crate::cryptography::password_metadata::PasswordMetaData;
use crate::omid_version::OmidVersion;

pub struct RootIdentityDocument {
    pub omid_version: OmidVersion,
    pub crypto_algo: AsymmetricCryptoAlgorithm,
    pub password_meta_data: PasswordMetaData,
    pub encrypted_private_key: Vec<u8>,
    pub description: String,
}
