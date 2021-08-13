use crate::versions;
use crates_io_api::{Error, SyncClient};
use semver::{Version, VersionReq};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const AGENT: &str = "carp_bot (https://github.com/eyemotif/carp)";
const RATE_LIMIT: std::time::Duration = std::time::Duration::from_millis(1000);

pub fn crate_versions(name: &str) -> Result<Vec<String>> {
    let client = SyncClient::new(AGENT, RATE_LIMIT)?;
    return Ok(client
        .get_crate(name)?
        .versions
        .into_iter()
        .map(|x| x.num)
        .collect::<Vec<String>>());
}
pub fn crate_exists(name: &str) -> Result<bool> {
    let client = SyncClient::new(AGENT, RATE_LIMIT)?;
    return match client.get_crate(name) {
        Ok(_) => Ok(true),
        Err(e) => match e {
            Error::NotFound(_) => Ok(false),
            other => Err(other.into()),
        },
    };
}
pub fn crate_latest(name: &str) -> Result<String> {
    let mut versions = crate_versions(name)?;
    return Ok(versions.remove(0));
}
pub fn crate_has_version(name: &str, ver: &str) -> Result<bool> {
    let version_compare = VersionReq::parse(ver)?;
    for version_str in crate_versions(name)? {
        if version_compare.matches(&Version::parse(&version_str)?) {
            return Ok(true);
        }
    }
    return Ok(false);
}
pub fn crate_get_update(name: &str, ver: &str) -> Result<Option<String>> {
    let latest = crate_latest(name)?;
    if versions::compare_version(&latest, ver)? {
        return Ok(None);
    } else {
        return Ok(Some(latest));
    }
}
