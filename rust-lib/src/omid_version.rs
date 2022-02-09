pub use crate::protobuf::OmidVersion;
use konst::{primitive::parse_u32, result::unwrap_ctx};

#[derive(Copy, Clone)]
struct RawOmidVersion<'a> {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release_label: Option<&'a str>,
}

static RAW_OMID_VERSION: RawOmidVersion = RawOmidVersion {
    major: unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MAJOR"))),
    minor: unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MINOR"))),
    patch: unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_PATCH"))),
    pre_release_label: option_env!("CARGO_PKG_VERSION_PRE"),
};

impl<'a> From<RawOmidVersion<'a>> for OmidVersion {
    fn from(raw: RawOmidVersion<'a>) -> Self {
        OmidVersion {
            major: raw.major,
            minor: raw.minor,
            patch: raw.patch,
            pre_release_label: raw.pre_release_label.map(|pre| pre.to_string()),
        }
    }
}

pub fn get_omid_version() -> OmidVersion {
    RAW_OMID_VERSION.into()
}
