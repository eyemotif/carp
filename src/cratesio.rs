use crate::versions;
use crates_io_api::{Error, SyncClient};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn crate_versions(name: &str) -> Result<Vec<String>> {
    let client = SyncClient::new("carp_bot", std::time::Duration::from_millis(1000))?;
    return Ok(client
        .get_crate(name)?
        .versions
        .into_iter()
        .map(|x| x.num)
        .collect::<Vec<String>>());
}
pub fn crate_exists(name: &str) -> Result<bool> {
    let client = SyncClient::new("carp_bot", std::time::Duration::from_millis(1000))?;
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
    let versions = crate_versions(name)?;
    return Ok(versions.contains(&String::from(ver)));
}
pub fn crate_updated(name: &str, ver: &str) -> Result<Option<String>> {
    let latest = crate_latest(name)?;
    if versions::compare_version(&latest, ver)? {
        return Ok(None);
    } else {
        return Ok(Some(latest));
    }
}
