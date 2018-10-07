//! Cockatrice integration

use std::{
    fmt,
    io::self,
    process::Command
};
use itertools::Itertools;
use reqwest;
use tempfile;
use super::{
    github::{
        self,
        Repo
    },
    util::client
};

#[cfg(target_arch = "x86")]
static PLATFORM_SUFFIX: &'static str = "win32.exe";
#[cfg(target_arch = "x86_64")]
static PLATFORM_SUFFIX: &'static str = "win64.exe";

/// An error that can occur while installing Cockatrice.
#[derive(Debug)]
pub enum OtherError {
    /// The Cockatrice installer exited with an error exit code.
    Installer,
    /// The asset for the local platform was not found in the current release.
    MissingAsset
}

wrapped_enum! {
    /// An error that can occur while installing Cockatrice.
    #[derive(Debug)]
    pub enum Error {
        #[allow(missing_docs)]
        GitHub(github::Error),
        #[allow(missing_docs)]
        Io(io::Error),
        #[allow(missing_docs)]
        Other(OtherError),
        #[allow(missing_docs)]
        Reqwest(reqwest::Error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::GitHub(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::Other(OtherError::Installer) => write!(f, "Cockatrice Setup failed."),
            Error::Other(OtherError::MissingAsset) => write!(f, "Could not find download link for Cockatrice."),
            Error::Reqwest(ref e) => e.fmt(f)
        }
    }
}

/// Downloads and installs Cockatrice using the interactive installer.
pub fn install(debug: bool) -> Result<(), Error> {
    if debug { eprintln!("making reqwest client"); }
    let client = client()?;
    if debug { eprintln!("determining download URL"); }
    let download_url = {
        let release_assets = Repo::new("Cockatrice", "Cockatrice").latest_release(&client)?.assets;
        let (asset,) = release_assets.into_iter()
            .filter(|asset| asset.name.ends_with(PLATFORM_SUFFIX))
            .collect_tuple().ok_or(OtherError::MissingAsset)?;
        asset.browser_download_url
    };
    if debug { eprintln!("download URL is {:?}", download_url); }
    if debug { eprintln!("making tempfile"); }
    let mut installer_file = tempfile::Builder::new()
        .prefix("Cockatrice")
        .suffix(".exe")
        .tempfile()?;
    if debug { eprintln!("making download request"); }
    let mut response = client.get(&download_url).send()?.error_for_status()?;
    if debug { eprintln!("downloading installer"); }
    response.copy_to(&mut installer_file)?;
    let installer_path = installer_file.into_temp_path();
    if debug { eprintln!("running installer, path is {:?}", installer_path); }
    //if !Command::new(&installer_path).status()?.success() {
    if !Command::new("cmd").arg("/C").arg(&installer_path).status()?.success() { //HACK use `cmd` to get the UAC prompt to display
        return Err(OtherError::Installer.into());
    }
    Ok(())
}
