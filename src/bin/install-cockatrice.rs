#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate lore_seeker_desktop;

use lore_seeker_desktop::trice;

fn main() -> Result<(), trice::Error> {
    trice::install(true)
}
