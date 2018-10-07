//! Clients for relevant parts of the GitHub API

#![allow(missing_docs)]

use std::fmt;
use reqwest;

/// An error that can occur in the GitHub API.
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

#[derive(Deserialize)]
pub struct Release {
    pub assets: Vec<ReleaseAsset>,
    pub tag_name: String
}

#[derive(Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String //TODO reqwest::Url
}

#[derive(Deserialize)]
pub struct Tag {
    pub name: String,
    pub commit: Commit
}

#[derive(Deserialize)]
pub struct Commit {
    pub sha: String
}

/// A GitHub repository. Provides API methods.
pub struct Repo {
    /// The GitHub user or organization who owns this repo.
    pub user: String,
    /// The name of the repo.
    pub name: String
}

impl Repo {
    pub fn new(user: impl ToString, name: impl ToString) -> Self {
        Repo {
            user: user.to_string(),
            name: name.to_string()
        }
    }

    pub fn latest_release(&self, client: &reqwest::Client) -> Result<Release, Error> {
        Ok(
            client.get(&format!("https://api.github.com/repos/{}/{}/releases/latest", self.user, self.name))
                .send()?
                .error_for_status()?
                .json::<Release>()?
        )
    }
}
