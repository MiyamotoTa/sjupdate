use anyhow::{Context, Result};
use clap::Parser;

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
    #[arg(short, long, default_value = "https://git.sheetjs.com/sheetjs/sheetjs/tags.rss")]
    url: String,
    /// The path to the directory containing the package.json file. Must be relative to the project root.
    #[arg(short, long, default_value = ".")]
    directory: String,
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
        install_latest_version(&latest_release.version.to_string(), &args.directory)?;
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

fn install_latest_version(latest_version: &str, directory: &str) -> Result<()> {
    // The latest version URL is assumed to be in the format: https://cdn.sheetjs.com/xlsx-0.19.3/xlsx-0.19.3.tgz
    let latest_version_url = format!("https://cdn.sheetjs.com/xlsx-{}/xlsx-{}.tgz", latest_version, latest_version);
    println!("Downloading the latest version from: {}", latest_version_url);

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