#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate cargo_metadata;
extern crate lore_seeker_desktop;
extern crate reqwest;
extern crate semver;
#[macro_use] extern crate wrapped_enum;

use std::cmp::Ordering::*;
use semver::{
    SemVerError,
    Version
};
use lore_seeker_desktop::{
    github,
    update,
    util
};

#[derive(Debug)]
enum OtherError {
    MissingPackage,
    SameVersion,
    VersionRegression
}

wrapped_enum! {
    #[derive(Debug)]
    enum Error {
        Cargo(cargo_metadata::Error),
        GitHub(github::Error),
        Other(OtherError),
        Reqwest(reqwest::Error),
        SemVer(SemVerError)
    }
}

fn main() -> Result<(), Error> {
    let local_version = cargo_metadata::metadata(None)?.packages.first().ok_or(OtherError::MissingPackage)?.version.parse::<Version>()?;
    let remote_version = update::latest_release_tag_name(&util::client()?)?[1..].parse::<Version>()?;
    match local_version.cmp(&remote_version) {
        Less => { return Err(OtherError::VersionRegression.into()); }
        Equal => { return Err(OtherError::SameVersion.into()); }
        Greater => ()
    }
    unimplemented!();
    //TODO cargo build --bin=lore-seeker-desktop --release
    //TODO cargo +stable-pc-i686-msvc build --bin=lore-seeker-desktop --release --target-dir=target-x86
    //TODO upload new release (POST /repos/fenhl/lore-seeker-desktop/releases)
    //TODO attach target/release/lore-seeker-windows.exe as lore-seeker-windows-64bit.exe
    //TODO attach target-x8/release/lore-seeker-windows.exe as lore-seeker-windows-32bit.exe
}
