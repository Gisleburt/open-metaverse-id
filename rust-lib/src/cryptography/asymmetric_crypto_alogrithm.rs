use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsymmetricCryptoAlgorithmError {
    #[error("Invalid enum value for Asymmetric Crypto Alogrithm `{0}`")]
    InvalidAsymmetricCryptoAlgorithm(i32),
}

#[repr(i32)]
pub enum AsymmetricCryptoAlgorithm {
    ECDH = 1,
}

impl TryFrom<i32> for AsymmetricCryptoAlgorithm {
    type Error = AsymmetricCryptoAlgorithmError;

    fn try_from(algo: i32) -> Result<Self, Self::Error> {
        match algo {
            1 => Ok(Self::ECDH),
            _ => Err(AsymmetricCryptoAlgorithmError::InvalidAsymmetricCryptoAlgorithm(algo as i32)),
        }
    }
}
