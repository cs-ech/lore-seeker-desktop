#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

use std::{
    cmp::Ordering::*,
    fs::File,
    io::{
        self,
        prelude::*
    },
    process::Command
};
use itertools::Itertools;
use semver::{
    SemVerError,
    Version
};
use wrapped_enum::wrapped_enum;
use lore_seeker_desktop::{
    github::Repo,
    util
};

#[derive(Debug)]
enum OtherError {
    Command,
    MissingPackage,
    SameVersion,
    VersionRegression
}

wrapped_enum! {
    #[derive(Debug)]
    enum Error {
        Cargo(cargo_metadata::Error),
        Io(io::Error),
        Other(OtherError),
        ReleaseClient(util::ReleaseClientError),
        Reqwest(reqwest::Error),
        SemVer(SemVerError)
    }
}

fn main() -> Result<(), Error> {
    //TODO make sure working dir is clean and on master and up to date with remote and remote is up to date. Alternatively, make sure we're on gitdir master and up to date
    let repo = Repo::new("fenhl", "lore-seeker-desktop");
    let client = util::release_client()?;
    let metadata = cargo_metadata::MetadataCommand::default().exec()?;
    let (pkg,) = metadata.packages.into_iter().filter(|pkg| pkg.name == "lore-seeker").collect_tuple().ok_or(OtherError::MissingPackage)?;
    let local_version = pkg.version;
    let remote_version = repo.latest_release(&client)?.tag_name[1..].parse::<Version>()?;
    match local_version.cmp(&remote_version) {
        Less => { return Err(OtherError::VersionRegression.into()); }
        Equal => { return Err(OtherError::SameVersion.into()); }
        Greater => {}
    }
    if !Command::new("rustup").arg("update").arg("stable").status()?.success() { return Err(OtherError::Command.into()); }
    if !Command::new("rustup").arg("update").arg("stable-i686-pc-windows-msvc").status()?.success() { return Err(OtherError::Command.into()); }
    if !Command::new("cargo").arg("build").arg("--bin=lore-seeker-desktop").arg("--release").status()?.success() { return Err(OtherError::Command.into()); }
    if !Command::new("cargo").arg("+stable-i686-pc-windows-msvc").arg("build").arg("--bin=lore-seeker-desktop").arg("--release").arg("--target-dir=target-x86").status()?.success() { return Err(OtherError::Command.into()); }
    let release_notes = {
        let mut release_notes_file = tempfile::Builder::new()
            .prefix("lore-seeker-desktop-release-notes")
            .suffix(".md")
            .tempfile()?;
        if !Command::new("nano").arg(release_notes_file.path()).status()?.success() { return Err(OtherError::Command.into()); }
        let mut buf = String::default();
        release_notes_file.read_to_string(&mut buf)?;
        buf
    };
    let release = repo.create_release(&client, format!("Lore Seeker Desktop {}", local_version), format!("v{}", local_version), release_notes)?;
    repo.release_attach(&client, &release, "lore-seeker-windows-64bit.exe", "application/vnd.microsoft.portable-executable", File::open("target/release/lore-seeker-desktop.exe")?)?;
    repo.release_attach(&client, &release, "lore-seeker-windows-32bit.exe", "application/vnd.microsoft.portable-executable", File::open("target-x86/release/lore-seeker-desktop.exe")?)?;
    repo.publish_release(&client, release)?;
    Ok(())
}
