//! Functions for handling self-updates and updates of Cockatrice files.

use itertools::Itertools;
use super::{
    github::{
        self,
        Repo
    },
    util::client,
    version
};

/// Returns `Ok(true)` if Lore Seeker Desktop is up to date, or `Ok(false)` if an update is available.
pub fn update_check() -> Result<bool, github::Error> {
    let client = client()?;
    let repo = Repo::new("fenhl", "lore-seeker-desktop");
    let tag_name = repo.latest_release(&client)?.tag_name;
    let current_hash = if let Some((tag,)) = repo.tags(&client)?.into_iter().filter(|tag| tag.name == tag_name).collect_tuple() {
        tag.commit.sha
    } else {
        return Err(github::Error::TagNotFound);
    };
    Ok(version::GIT_COMMIT_HASH == current_hash)
}
