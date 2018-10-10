//! Various utility functions.

use std::{
    fs::File,
    io::{
        self,
        prelude::*
    }
};
#[cfg(windows)]
use nwg::{
    self,
    constants::{
        MessageButtons,
        MessageChoice,
        MessageIcons,
        MessageParams
    },
    fatal_message
};
use reqwest;

/// Returns a `reqwest::Client` identified as Lore Seeker Desktop via the `User-Agent` header.
pub fn client() -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(concat!("lore-seeker-desktop/", env!("CARGO_PKG_VERSION"))));
    Ok(reqwest::Client::builder().default_headers(headers).build()?)
}

wrapped_enum! {
    /// An error that can occur in `release_client`.
    #[derive(Debug)]
    pub enum ReleaseClientError {
        #[allow(missing_docs)]
        InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
        #[allow(missing_docs)]
        Io(io::Error),
        #[allow(missing_docs)]
        Reqwest(reqwest::Error)
    }
}

/// Returns a `reqwest::Client` which also authenticates itself to the GitHub API.
pub fn release_client() -> Result<reqwest::Client, ReleaseClientError> {
    let mut headers = reqwest::header::HeaderMap::new();
    let mut token = String::default();
    File::open("assets/release-token")?.read_to_string(&mut token)?;
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&token)?);
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(concat!("lore-seeker-desktop/", env!("CARGO_PKG_VERSION"))));
    Ok(reqwest::Client::builder().default_headers(headers).build()?)
}

/// Asks the user a yes/no question and returns the answer.
#[cfg(windows)]
pub fn yesno(message: &str) -> bool {
    let choice = nwg::message(&MessageParams {
        title: "Lore Seeker",
        content: message,
        buttons: MessageButtons::YesNo,
        icons: MessageIcons::Question
    });
    match choice {
        MessageChoice::Yes => true,
        MessageChoice::No => false,
        c => { fatal_message("Lore Seeker fatal error", &format!("Yes/no message returned unexpected choice: {:?}", c)); }
    }
}
