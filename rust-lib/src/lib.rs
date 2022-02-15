mod raw_omid_version;

include!(concat!(env!("OUT_DIR"), "/open_meta_id.rs"));

pub use raw_omid_version::omid_version;

pub fn create_root_identity<S: Into<Vec<u8>>>(string: S) -> String {
    String::from_utf8_lossy(&string.into()).to_string()
}

pub fn challenge_root_identity<R, C>(_root_certificate: R, _challenge: C) -> String
    where
        R: AsRef<str>,
        C: AsRef<str>,
{
    todo!()
}

pub fn sign_intermediate_identity<R, I>(_root_certificate: R, _intermediate_certificate: I) -> String
    where
        R: AsRef<str>,
        I: AsRef<str>,
{
    todo!()
}

pub fn challenge_intermediate_identity<I, C>(_intermediate_certificate: I, _challenge: C) -> String
    where
        I: AsRef<str>,
        C: AsRef<str>,
{
    todo!()
}
