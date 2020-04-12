use crates_io_api::{Error, SyncClient};

fn compare_version(crt: &str, given: &str) -> bool {
    return crt == given;
}

pub fn crate_versions(name: &str) -> Result<Vec<String>, Error> {
    let client = SyncClient::new();
    return Ok(client
        .get_crate(name)?
        .versions
        .into_iter()
        .map(|x| x.num)
        .collect::<Vec<String>>());
}
pub fn crate_exists(name: &str) -> bool {
    let client = SyncClient::new();
    match client.get_crate(name) {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn crate_latest(name: &str) -> Result<String, Error> {
    let mut versions = crate_versions(name)?;
    return Ok(versions.remove(0));
}
pub fn crate_has_version(name: &str, ver: &str) -> Result<bool, Error> {
    let versions = crate_versions(name)?;
    return Ok(versions.contains(&String::from(ver)));
}
pub fn crate_updated(name: &str, ver: &str) -> Result<Option<String>, Error> {
    let latest = crate_latest(name)?;
    if compare_version(&latest, ver) {
        return Ok(None);
    } else {
        return Ok(Some(latest));
    }
}
