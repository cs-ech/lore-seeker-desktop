#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate cargo_metadata;
extern crate lore_seeker_desktop;
extern crate reqwest;
extern crate semver;
extern crate tempfile;
#[macro_use] extern crate wrapped_enum;

use std::{
    cmp::Ordering::*,
    fs::File,
    io::{
        self,
        prelude::*
    },
    process::Command
};
use semver::{
    SemVerError,
    Version
};
use lore_seeker_desktop::{
    github::{
        self,
        Repo
    },
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
        GitHub(github::Error),
        Io(io::Error),
        Other(OtherError),
        Reqwest(reqwest::Error),
        SemVer(SemVerError)
    }
}

fn main() -> Result<(), Error> {
    //TODO make sure working dir is clean and on master and up to date with remote and remote is up to date. Alternatively, make sure we're on gitdir master and up to date
    let repo = Repo::new("fenhl", "lore-seeker-desktop");
    let client = util::client()?;
    let local_version = cargo_metadata::metadata(None)?.packages.first().ok_or(OtherError::MissingPackage)?.version.parse::<Version>()?;
    let remote_version = repo.latest_release(&client)?.tag_name[1..].parse::<Version>()?;
    match local_version.cmp(&remote_version) {
        Less => { return Err(OtherError::VersionRegression.into()); }
        Equal => { return Err(OtherError::SameVersion.into()); }
        Greater => ()
    }
    if !Command::new("cargo").arg("build").arg("--bin=lore-seeker-windows").arg("--release").status()?.success() { return Err(OtherError::Command.into()); }
    if !Command::new("cargo").arg("+stable-i686-pc-windows-msvc").arg("build").arg("--bin=lore-seeker-windows").arg("--release").arg("--target-dir=target-x86").status()?.success() { return Err(OtherError::Command.into()); }
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
    repo.release_attach(&client, &release, "lore-seeker-windows-64bit.exe", "application/vnd.microsoft.portable-executable", File::open("target/release/lore-seeker-windows.exe")?)?;
    repo.release_attach(&client, &release, "lore-seeker-windows-32bit.exe", "application/vnd.microsoft.portable-executable", File::open("target-x86/release/lore-seeker-windows.exe")?)?;
    repo.publish_release(&client, release)?;
    Ok(())
}
