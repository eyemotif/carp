use semver::Version;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compare_version(current: &str, given: &str) -> Result<bool> {
    return Ok(Version::parse(current)? < Version::parse(given)?);
}
