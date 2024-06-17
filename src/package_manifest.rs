use std::fs::read_to_string;

use anyhow::Result;
use lenient_semver::parse;
use semver::Version;
use serde_json::{from_str, Value};

pub fn get_current_version(path: &str) -> Result<Option<Version>> {
    let json_value = read_json_value_from_file(path)?;

    let current_version = parse_current_xlsx_version(json_value);

    Ok(current_version)
}

fn read_json_value_from_file(path: &str) -> Result<Value> {
    let file = read_to_string(format!("{}/package.json", path))?;
    let json_value = from_str(&file)?;

    Ok(json_value)
}

fn parse_current_xlsx_version(json_value: Value) -> Option<Version> {
    match json_value["dependencies"]["xlsx"].as_str() {
        None => None,
        Some(current_url) => {
            // Parse the version from the current version of xlsx.
            // The version is considered to be the part of the URL after the last slash.
            // The URL is assumed to be in the format: https://cdn.sheetjs.com/xlsx-0.19.3/xlsx-0.19.3.tgz
            let version = current_url.split('/').nth(3).unwrap().split('-').nth(1)?;
            let version = parse(version).unwrap();

            Some(version)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::read_json_value_from_file;

    #[test]
    fn test_read_json_value_from_file() {
        let path = "tests/fixtures";
        let json_value = read_json_value_from_file(path).unwrap();

        assert_eq!(
            json_value["dependencies"]["xlsx"].as_str(),
            Some("https://cdn.sheetjs.com/xlsx-0.19.3/xlsx-0.19.3.tgz")
        );
    }

    #[test]
    fn test_parse_current_xlsx_version() {
        let json_value = serde_json::json!({
            "dependencies": {
                "xlsx": "https://cdn.sheetjs.com/xlsx-0.19.3/xlsx-0.19.3.tgz"
            }
        });

        let version = super::parse_current_xlsx_version(json_value);

        assert_eq!(version, Some(semver::Version::new(0, 19, 3)));
    }
}
