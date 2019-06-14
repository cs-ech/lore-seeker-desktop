//! Various utility functions.

use std::{
    fs::File,
    io::{
        self,
        prelude::*
    },
    time::Duration
};
use azul::dialogs::{
    MessageBoxIcon,
    YesNo::Yes,
    msg_box_ok,
    msg_box_yes_no
};
use wrapped_enum::wrapped_enum;

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

/// Returns a `reqwest::Client` identified as Lore Seeker Desktop via the `User-Agent` header.
pub fn client() -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(concat!("lore-seeker-desktop/", env!("CARGO_PKG_VERSION"))));
    Ok(reqwest::Client::builder().default_headers(headers).build()?)
}

/// Displays an error message as a dialog, but returns normally after OK is clicked.
pub fn error_message(title: &str, message: &str) {
    msg_box_ok(title, message, MessageBoxIcon::Error);
}

/// Displays an error message as a dialog, then panics after OK is clicked.
pub fn fatal_message(title: &str, message: &str) -> ! {
    error_message(title, message);
    panic!("{}: {}", title, message);
}

/// Returns a `reqwest::Client` which also authenticates itself to the GitHub API.
pub fn release_client() -> Result<reqwest::Client, ReleaseClientError> {
    let mut headers = reqwest::header::HeaderMap::new();
    let mut token = String::default();
    File::open("assets/release-token")?.read_to_string(&mut token)?;
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("token {}", token))?);
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(concat!("lore-seeker-desktop/", env!("CARGO_PKG_VERSION"))));
    Ok(reqwest::Client::builder().default_headers(headers).timeout(Duration::from_secs(600)).build()?)
}

/// Asks the user a yes/no question and returns the answer.
pub fn yesno(message: &str) -> bool {
    msg_box_yes_no("Lore Seeker", message, MessageBoxIcon::Question, Yes) == Yes
}
