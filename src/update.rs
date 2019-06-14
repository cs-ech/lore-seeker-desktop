//! Functions for handling self-updates and updates of Cockatrice files.

use std::{
    fmt,
    fs::File,
    io,
    path::Path
};
use itertools::Itertools;
use crate::{
    github::Repo,
    version
};

#[cfg(target_arch = "x86")]
static PLATFORM_ASSET: &'static str = "lore-seeker-windows-32bit.exe";
#[cfg(target_arch = "x86_64")]
static PLATFORM_ASSET: &'static str = "lore-seeker-windows-64bit.exe";

/// An error that can occur in the GitHub API.
#[derive(Debug)]
pub enum Error {
    /// The release asset we were looking for is not in the release.
    AssetNotFound,
    /// An I/O error occurred.
    Io(io::Error),
    /// An error occurred in the `reqwest` crate.
    Reqwest(reqwest::Error),
    /// The latest release's tag name is not listed in the repo's tags.
    TagNotFound
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AssetNotFound => write!(f, "Release asset not found."),
            Error::Io(ref e) => e.fmt(f),
            Error::Reqwest(ref e) => e.fmt(f),
            Error::TagNotFound => write!(f, "Release tag not found.")
        }
    }
}

/// Downloads the latest release of Lore Seeker Desktop and saves it to the given path.
pub fn download_update(client: &reqwest::Client, save_path: impl AsRef<Path>) -> Result<(), Error> {
    let download_url = {
        let (asset,) = Repo::new("fenhl", "lore-seeker-desktop")
            .latest_release(client)?
            .assets
            .into_iter()
            .filter(|asset| &asset.name == PLATFORM_ASSET)
            .collect_tuple().ok_or(Error::AssetNotFound)?;
        asset.browser_download_url
    };
    let mut response = client.get(&download_url).send()?.error_for_status()?;
    let mut save_file = File::open(save_path)?;
    response.copy_to(&mut save_file)?;
    Ok(())
}

/// Returns `Ok(true)` if Lore Seeker Desktop is up to date, or `Ok(false)` if an update is available.
pub fn update_check(client: &reqwest::Client) -> Result<bool, Error> {
    let repo = Repo::new("fenhl", "lore-seeker-desktop");
    let tag_name = repo.latest_release(&client)?.tag_name;
    let current_hash = if let Some((tag,)) = repo.tags(&client)?.into_iter().filter(|tag| tag.name == tag_name).collect_tuple() {
        tag.commit.sha
    } else {
        return Err(Error::TagNotFound);
    };
    Ok(version::GIT_COMMIT_HASH == current_hash)
}
