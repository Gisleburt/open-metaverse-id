include!(concat!(env!("OUT_DIR"), "/open_meta_id.rs"));


impl From<open_meta_id::omid_version::OmidVersion> for OmidVersion {
    fn from(omid: open_meta_id::OmidVersion) -> Self {
        OmidVersion {
            major: omid.major,
            minor: omid.minor,
            patch: omid.patch,
            pre_release_label: omid.pre_release_label,
        }
    }
}
