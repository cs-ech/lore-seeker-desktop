#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate cargo_metadata;
extern crate lore_seeker_desktop;
extern crate semver;
#[macro_use] extern crate wrapped_enum;

use semver::{
    SemVerError,
    Version
};
use lore_seeker_desktop::update;

#[derive(Debug)]
enum OtherError {
    MissingPackage
}

wrapped_enum! {
    #[derive(Debug)]
    enum Error {
        Cargo(cargo_metadata::Error),
        Other(OtherError),
        SemVer(SemVerError),
        Update(update::Error)
    }
}

fn main() -> Result<(), Error> {
    let local_version = cargo_metadata::metadata(None)?.packages.first().ok_or(OtherError::MissingPackage)?.version.parse::<Version>()?;
    let remote_version = update::latest_release_tag_name(&update::client()?)?[1..].parse::<Version>()?;
    println!("local: {}, remote: {}", local_version, remote_version); //DEBUG
    unimplemented!(); //TODO
}
