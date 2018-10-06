//! Functions for handling self-updates and updates of Cockatrice files.

use std::fmt;
use itertools::Itertools;
use reqwest;
use super::version;

#[derive(Deserialize)]
struct LatestRelease {
    tag_name: String
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    commit: Commit
}

#[derive(Deserialize)]
struct Commit {
    sha: String
}

/// An error that can occur while checking for updates.
#[derive(Debug)]
pub enum Error {
    /// An error occurred in the `reqwest` crate.
    Reqwest(reqwest::Error),
    /// The latest release's tag name is not listed in the repo's tags.
    TagNotFound
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Reqwest(ref e) => e.fmt(f),
            Error::TagNotFound => write!(f, "Release tag not found.")
        }
    }
}

/// Returns a `reqwest::Client` identified as Lore Seeker Desktop via the `User-Agent` header.
pub fn client() -> Result<reqwest::Client, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(concat!("lore-seeker-desktop/", env!("CARGO_PKG_VERSION"))));
    Ok(reqwest::Client::builder().default_headers(headers).build()?)
}

/// Returns the tag name of the latest release of Lore Seeker Desktop published on GitHub.
pub fn latest_release_tag_name(client: &reqwest::Client) -> Result<String, Error> {
    Ok(
        client.get("https://api.github.com/repos/fenhl/lore-seeker-desktop/releases/latest")
            .send()?
            .error_for_status()?
            .json::<LatestRelease>()?
            .tag_name
    )
}

/// Returns `Ok(true)` if Lore Seeker Desktop is up to date, or `Ok(false)` if an update is available.
pub fn update_check() -> Result<bool, Error> {
    let client = client()?;
    let tag_name = latest_release_tag_name(&client)?;
    let tags = client.get("https://api.github.com/repos/fenhl/lore-seeker-desktop/tags").send()?.error_for_status()?.json::<Vec<Tag>>()?;
    let current_hash = if let Some((tag,)) = tags.into_iter().filter(|tag| tag.name == tag_name).collect_tuple() {
        tag.commit.sha
    } else {
        return Err(Error::TagNotFound);
    };
    Ok(version::GIT_COMMIT_HASH == current_hash)
}
