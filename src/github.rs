//! Clients for relevant parts of the GitHub API

#![allow(missing_docs)] //TODO

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
    pub body: String,
    pub id: u64,
    pub name: String,
    pub tag_name: String,
    pub upload_url: String //TODO reqwest::Url
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

    /// Creates a draft release, which can be published using `Repo::publish_release`.
    pub fn create_release(&self, client: &reqwest::Client, name: String, tag_name: String, body: String) -> Result<Release, Error> {
        Ok(
            client.post(&format!("https://api.github.com/repos/{}/{}/releases", self.user, self.name))
                .json(&json!({
                    "body": body,
                    "draft": true,
                    "name": name,
                    "tag_name": tag_name
                }))
                .send()?
                .error_for_status()?
                .json::<Release>()?
        )
    }

    pub fn publish_release(&self, client: &reqwest::Client, release: Release) -> Result<Release, Error> {
        Ok(
            client.patch(&format!("https://api.github.com/repos/{}/{}/releases/{}", self.user, self.name, release.id))
                .json(&json!({"draft": false}))
                .send()?
                .error_for_status()?
                .json::<Release>()?
        )
    }

    pub fn release_attach(&self, client: &reqwest::Client, release: &Release, name: &str, content_type: &'static str, body: impl Into<reqwest::Body>) -> Result<ReleaseAsset, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static(content_type));
        Ok(
            client.post(&release.upload_url.replace("{?name,label}", ""))
                .query(&[("name", name)])
                .headers(headers)
                .body(body)
                .send()?
                .error_for_status()?
                .json::<ReleaseAsset>()?
        )
    }

    pub fn tags(&self, client: &reqwest::Client) -> Result<Vec<Tag>, Error> {
        Ok(
            client.get(&format!("https://api.github.com/repos/{}/{}/tags", self.user, self.name))
                .send()?
                .error_for_status()?
                .json::<Vec<Tag>>()?
        )
    }
}
