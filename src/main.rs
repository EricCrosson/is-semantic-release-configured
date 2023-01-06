//! Binary to test if semantic-release is configured.
//!
//! The intended use-case is in shell-script control flow.
//!
//! Note: this tool currently only checks for the existence of configuration,
//! it does not validate the content. This tool does not care if your
//! configuration will be rejected by semantic-release.

#[forbid(unsafe_code)]
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};

fn does_releaserc_file_exist() -> Result<bool> {
    let basename = ".releaserc";
    let extensions: [&str; 5] = ["yaml", "yml", "json", "js", "cjs"];

    if Path::new(basename).exists() {
        return Ok(true);
    }

    for extension in extensions {
        if Path::new(&format!("{basename}.{extension}")).exists() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn does_release_config_file_exist() -> Result<bool> {
    let basename = "release.config";
    let extensions: [&str; 2] = ["js", "cjs"];

    for extension in extensions {
        if Path::new(&format!("{basename}.{extension}")).exists() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn does_package_manifest_have_release_property() -> Result<bool> {
    let package_manifest_path = Path::new("package.json");
    if !package_manifest_path.exists() {
        return Ok(false);
    }

    // Reading a file into a string before invoking Serde is faster than
    // invoking Serde from a BufReader, see
    // https://github.com/serde-rs/json/issues/160
    let mut string = String::new();
    File::open(package_manifest_path)?.read_to_string(&mut string)?;
    let package_manifest: serde_json::Value = serde_json::from_str(&string)
        .with_context(|| format!("Unable to parse JSON from file {:?}", package_manifest_path))?;

    Ok(package_manifest
        .as_object()
        .map(|manifest| manifest.contains_key("release"))
        .unwrap_or_default())
}

/// The configuration rules, according to the [semantic-release readme]:
///
/// semantic-release’s options, mode and plugins can be set via either:
///
/// - A `.releaserc` file, written in YAML or JSON, with optional extensions: `.yaml`/`.yml`/`.json`/`.js`/`.cjs`
/// - A `release.config.(js|cjs)` file that exports an object
/// - A `release` key in the project's package.json file
///
/// [semantic-release readme]:
///   https://github.com/semantic-release/semantic-release/blob/master/docs/usage/configuration.md#configuration-file
fn is_semantic_release_configured() -> Result<bool> {
    if does_releaserc_file_exist()? {
        return Ok(true);
    }
    if does_release_config_file_exist()? {
        return Ok(true);
    }
    if does_package_manifest_have_release_property()? {
        return Ok(true);
    }
    Ok(false)
}

fn main() -> Result<()> {
    let is_configured = is_semantic_release_configured()?;

    if is_configured {
        return Ok(());
    } else {
        std::process::exit(1);
    }
}
