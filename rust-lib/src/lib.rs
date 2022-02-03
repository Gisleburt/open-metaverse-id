pub fn create_root_identity<S: Into<Vec<u8>>>(string: S) -> String {
    String::from_utf8_lossy(&string.into()).to_string()
}
