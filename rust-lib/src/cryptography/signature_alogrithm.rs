use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignatureAlgorithmError {
    #[error("Invalid enum value for signature Alogrithm `{0}`")]
    InvalidSignatureAlgorithm(i32),
}

#[repr(i32)]
pub enum SignatureAlgorithm {
    ECDH = 1,
}

impl TryFrom<i32> for SignatureAlgorithm {
    type Error = SignatureAlgorithmError;

    fn try_from(algo: i32) -> Result<Self, Self::Error> {
        match algo {
            1 => Ok(Self::ECDH),
            _ => Err(SignatureAlgorithmError::InvalidSignatureAlgorithm(
                algo as i32,
            )),
        }
    }
}
