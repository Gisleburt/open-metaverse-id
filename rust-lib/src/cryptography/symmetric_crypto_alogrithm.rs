use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymmetricCryptoAlgorithmError {
    #[error("Invalid enum value for Symmetric Crypto Alogrithm `{0}`")]
    InvalidSymmetricCryptoAlgorithm(i32),
}

#[repr(i32)]
pub enum SymmetricCryptoAlgorithm {
    Aes256 = 1,
}

impl TryFrom<i32> for SymmetricCryptoAlgorithm {
    type Error = SymmetricCryptoAlgorithmError;

    fn try_from(algo: i32) -> Result<Self, Self::Error> {
        match algo {
            1 => Ok(Self::Aes256),
            _ => Err(SymmetricCryptoAlgorithmError::InvalidSymmetricCryptoAlgorithm(algo as i32)),
        }
    }
}
