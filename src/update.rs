//! Functions for handling self-updates and updates of Cockatrice files.

use itertools::Itertools;
use reqwest;
use super::{
    github::{
        self,
        Repo,
        Tag
    },
    util::client,
    version
};

/// Returns the tag name of the latest release of Lore Seeker Desktop published on GitHub.
pub fn latest_release_tag_name(client: &reqwest::Client) -> Result<String, github::Error> {
    Ok(Repo::new("fenhl", "lore-seeker-desktop").latest_release(client)?.tag_name)
}

/// Returns `Ok(true)` if Lore Seeker Desktop is up to date, or `Ok(false)` if an update is available.
pub fn update_check() -> Result<bool, github::Error> {
    let client = client()?;
    let tag_name = latest_release_tag_name(&client)?;
    let tags = client.get("https://api.github.com/repos/fenhl/lore-seeker-desktop/tags").send()?.error_for_status()?.json::<Vec<Tag>>()?;
    let current_hash = if let Some((tag,)) = tags.into_iter().filter(|tag| tag.name == tag_name).collect_tuple() {
        tag.commit.sha
    } else {
        return Err(github::Error::TagNotFound);
    };
    Ok(version::GIT_COMMIT_HASH == current_hash)
}
