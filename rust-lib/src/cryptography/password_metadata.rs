use crate::cryptography::symmetric_crypto_alogrithm::SymmetricCryptoAlgorithm;

pub struct PasswordMetaData {
    pub symmetric_crypto_algorithm: SymmetricCryptoAlgorithm,
    pub salt: std::vec::Vec<u8>,
    pub nonce: std::vec::Vec<u8>,
}

// pub fn create_key<KEY_SIZE>(password: String) -> (PasswordMetaData, Key<KEY_SIZE>) {
//     todo!()
// }
