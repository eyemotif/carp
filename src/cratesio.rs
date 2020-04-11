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
pub fn crate_updated(name: &str, ver: &str) -> Result<bool, Error> {
    return Ok(compare_version(&crate_latest(name)?, ver));
}
