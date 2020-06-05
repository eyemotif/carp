use crate::cratesio;
use crate::utils;

use std::io::{Error, ErrorKind};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn help() {
    println!(
        "carp: A basic CLI Rust dependency manager.
    - carp help: Prints this message
    - carp list: Lists all dependencies
    - carp add <crate name> [crate version]: Adds a dependency
    - carp rem <dependency name>: Removes a dependency
    - carp change <dependency name> <crate version>: Changes a dependency's version
    - carp check [crate name]: Checks if a dependency is up to date, or all dependencies if none are specified
    - carp update [crate name]: Updates a dependency, or all dependencies if none are specified"
    )
}
pub fn list() -> Result<()> {
    for (k, v) in utils::read_parse_dependencies(&utils::get_toml_path())? {
        println!("{} ({})", k, v);
    }
    return Ok(());
}
pub fn add(name: &str, version: Option<&str>) -> Result<String> {
    if !cratesio::crate_exists(name)? {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Crate '{}' does not exist", name),
        )
        .into());
    }

    let latest = cratesio::crate_latest(name)?;
    let ver = match version {
        Some(v) => {
            if cratesio::crate_has_version(name, v)? {
                v
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Crate '{}' does not have the version '{}'", name, v),
                )
                .into());
            }
        }
        None => &latest,
    };

    let path = &utils::get_toml_path();
    let mut dependencies = utils::read_parse_dependencies(path)?;
    if dependencies.contains_key(name) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Dependency '{}' already added", name),
        )
        .into());
    }
    dependencies.insert(String::from(name), String::from(ver));

    utils::write_dependencies(path, &dependencies)?;
    return Ok(format!("+ {} ({})", name, ver));
}
pub fn rem(name: &str) -> Result<String> {
    let path = &utils::get_toml_path();
    let mut dependencies = utils::read_parse_dependencies(path)?;
    match dependencies.remove_entry(name) {
        Some(_) => (),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Dependency '{}' not added", name),
            )
            .into())
        }
    };

    utils::write_dependencies(path, &dependencies)?;
    return Ok(format!("- {}", name));
}
pub fn change(name: &str, version: &str) -> Result<String> {
    let path = &utils::get_toml_path();
    let mut dependencies = utils::read_parse_dependencies(path)?;
    if !dependencies.contains_key(name) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Dependency '{}' not added", name),
        )
        .into());
    }

    if !cratesio::crate_exists(name)? {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Crate '{}' does not exist", name),
        )
        .into());
    }
    if !cratesio::crate_has_version(name, version)? {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Crate '{}' does not have the version '{}'", name, version),
        )
        .into());
    }
    let old_version = dependencies
        .insert(String::from(name), String::from(version))
        .unwrap();

    utils::write_dependencies(path, &dependencies)?;
    return Ok(format!("* {} ({}) -> ({})", name, old_version, version));
}
pub fn check(name: &str) -> Result<Option<String>> {
    let path = &utils::get_toml_path();
    let dependencies = utils::read_parse_dependencies(path)?;
    if !dependencies.contains_key(name) {}
    let current_ver = match dependencies.get(name) {
        Some(value) => value,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Dependency '{}' not added", name),
            )
            .into());
        }
    };

    if !cratesio::crate_exists(name)? {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Crate '{}' does not exist", name),
        )
        .into());
    }
    return match cratesio::crate_updated(name, current_ver)? {
        Some(new_version) => Ok(Some(format!(
            "! {} ({}): ({})",
            name, current_ver, new_version,
        ))),
        None => Ok(None),
    };
}
pub fn check_all() -> Result<Vec<String>> {
    let path = &utils::get_toml_path();
    let mut result = Vec::<String>::new();
    for (k, _) in utils::read_parse_dependencies(path)? {
        let chck = check(&k)?;
        if chck.is_some() {
            result.push(chck.unwrap())
        }
    }
    return Ok(result);
}
pub fn update(name: &str) -> Result<Option<String>> {
    match check(name)? {
        Some(_) => {
            let latest = cratesio::crate_latest(name)?;
            return Ok(Some(change(name, &latest)?));
        }
        None => return Ok(None),
    };
}
pub fn update_all() -> Result<Vec<String>> {
    let path = &utils::get_toml_path();
    let mut result = Vec::<String>::new();
    for (k, _) in utils::read_parse_dependencies(path)? {
        let updt = update(&k)?;
        if updt.is_some() {
            result.push(updt.unwrap())
        }
    }
    return Ok(result);
}
