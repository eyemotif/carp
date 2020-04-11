use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use toml::Value;

pub fn read_dependencies(path: &Path) -> Result<Value, Error> {
    let file_contents = fs::read_to_string(path)?;
    return match file_contents.parse::<Value>() {
        Ok(v) => Ok(v),
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };
}
pub fn parse_dependencies(value: Value) -> Option<HashMap<String, String>> {
    let top = value.as_table()?;
    let mut map = HashMap::new();
    for (k, v) in top.get("dependencies")?.as_table()? {
        map.insert(k.to_string(), v.as_str()?.to_string());
    }
    return Some(map);
}
pub fn convert_dependencies(dependencies: &HashMap<String, String>) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    for (k, v) in dependencies {
        map.insert(k.to_string(), Value::String(v.to_string()));
    }
    return map;
}

pub fn write_dependencies(path: &Path, dependencies: &HashMap<String, String>) -> Result<(), Error> {
    let mut toml_file = read_dependencies(path)?;
    toml_file["dependencies"] = Value::from(convert_dependencies(dependencies));
    let toml_string = match toml::ser::to_string(&toml_file) {
        Ok(v) => v,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };
    return fs::write(path, toml_string);
}
