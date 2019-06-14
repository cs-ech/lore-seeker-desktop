#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

use lore_seeker_desktop::trice;

fn main() -> Result<(), trice::Error> {
    trice::install(true)
}
