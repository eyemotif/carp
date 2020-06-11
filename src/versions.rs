use semver::{Version, VersionReq};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compare_version(current: &str, given: &str) -> Result<bool> {
    let is_match = VersionReq::parse(current)?.matches(&Version::parse(given)?);
    return Ok(is_match);
}
