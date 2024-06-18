use anyhow::{Context, Result};
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

use crate::package_manifest::get_current_version;
use crate::release::{convert_to_release, find_latest_release, Release};
use crate::rss::get_feed_items;

mod package_manifest;
mod release;
mod rss;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// RSS feed URL for the sheetjs releases
    #[arg(
        short,
        long,
        default_value = "https://git.sheetjs.com/sheetjs/sheetjs/tags.rss"
    )]
    url: String,
    /// The path to the directory containing the package.json file. Must be relative to the project root.
    #[arg(short, long, default_value = ".")]
    directory: String,
    /// The package manager to use. Supported values are "npm", "yarn", and "pnpm".
    #[arg(short, long, default_value_t, value_enum)]
    package_manager: PackageManager,
}

#[derive(Debug, Clone, ValueEnum, Default)]
enum PackageManager {
    #[default]
    Npm,
    Yarn,
    Pnpm,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let latest_release = find_latest_release_from_feed(&args.url)?;
    if check_update_required(&latest_release, &args.directory)? {
        install_latest_version(
            &latest_release.version.to_string(),
            &args.directory,
            &args.package_manager,
        )?;
    }

    Ok(())
}

fn find_latest_release_from_feed(url: &str) -> Result<Release> {
    println!("Fetching feed items from {}", url);
    let feed_items =
        get_feed_items(url).with_context(|| format!("Failed to get feed items from {}", url))?;
    println!("Found {} feed items", feed_items.len());

    println!("Converting feed items to releases");
    let versions = convert_to_release(feed_items);
    println!("Found {} releases", versions.len());

    println!("Finding the latest release");
    let latest_release =
        find_latest_release(versions).with_context(|| "Failed to find the latest release")?;
    println!("Latest release: {:?}", latest_release);

    Ok(latest_release)
}

fn check_update_required(release: &Release, package_manifest_directory: &str) -> Result<bool> {
    match get_current_version(package_manifest_directory)? {
        None => {
            println!("xlsx is not found in the package.json file");
            Ok(false)
        }
        Some(current_version) => {
            println!("Current version: {:?}", current_version);
            let latest_version = &release.version;
            if current_version < *latest_version {
                println!("A new version is available: {}", latest_version);
                Ok(true)
            } else {
                println!("The current version is up to date");
                Ok(false)
            }
        }
    }
}

fn install_latest_version(
    latest_version: &str,
    directory: &str,
    package_manager: &PackageManager,
) -> Result<()> {
    // The latest version URL is assumed to be in the format: https://cdn.sheetjs.com/xlsx-0.19.3/xlsx-0.19.3.tgz
    let latest_version_url = format!(
        "https://cdn.sheetjs.com/xlsx-{}/xlsx-{}.tgz",
        latest_version, latest_version
    );
    println!(
        "Downloading the latest version from: {}",
        latest_version_url
    );

    match package_manager {
        PackageManager::Npm => install_latest_version_npm(&latest_version_url, directory),
        PackageManager::Yarn => install_latest_version_yarn(&latest_version_url, directory),
        PackageManager::Pnpm => install_latest_version_pnpm(&latest_version_url, directory),
    }
}

fn install_latest_version_npm(latest_version_url: &str, directory: &str) -> Result<()> {
    // Run the npm rm command to uninstall the current version
    println!("Uninstalling the current version");
    let _ = std::process::Command::new("npm")
        .arg("rm")
        .arg("xlsx")
        .current_dir(directory)
        .output()
        .context("Failed to uninstall the current version")?;

    // Run the npm install command to install the latest version
    println!("Installing the latest version");
    let _ = std::process::Command::new("npm")
        .arg("install")
        .arg(latest_version_url)
        .current_dir(directory)
        .output()
        .context("Failed to install the latest version")?;

    Ok(())
}

fn install_latest_version_pnpm(latest_version_url: &str, directory: &str) -> Result<()> {
    // Run the pnpm remove command to uninstall the current version
    println!("Uninstalling the current version");
    let _ = std::process::Command::new("pnpm")
        .arg("rm")
        .arg("xlsx")
        .current_dir(directory)
        .output()
        .context("Failed to uninstall the current version")?;

    // Run the pnpm add command to install the latest version
    println!("Installing the latest version");
    let _ = std::process::Command::new("pnpm")
        .arg("install")
        .arg(latest_version_url)
        .current_dir(directory)
        .output()
        .context("Failed to install the latest version")?;

    Ok(())
}

fn install_latest_version_yarn(latest_version_url: &str, directory: &str) -> Result<()> {
    // Run the yarn remove command to uninstall the current version
    println!("Uninstalling the current version");
    let _ = std::process::Command::new("yarn")
        .arg("remove")
        .arg("xlsx")
        .current_dir(directory)
        .output()
        .context("Failed to uninstall the current version")?;

    // Run the yarn add command to install the latest version
    println!("Installing the latest version");
    let _ = std::process::Command::new("yarn")
        .arg("add")
        .arg(latest_version_url)
        .current_dir(directory)
        .output()
        .context("Failed to install the latest version")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_argument() {
        // Test the possible values
        assert_eq!(
            PackageManager::value_variants(),
            &[
                PackageManager::Npm,
                PackageManager::Yarn,
                PackageManager::Pnpm,
            ]
        );

        // Test the possible value
        assert_eq!(
            PackageManager::Npm.to_possible_value(),
            Some(PossibleValue::new("npm"))
        );
        assert_eq!(
            PackageManager::Yarn.to_possible_value(),
            Some(PossibleValue::new("yarn"))
        );
        assert_eq!(
            PackageManager::Pnpm.to_possible_value(),
            Some(PossibleValue::new("pnpm"))
        );
    }
}
