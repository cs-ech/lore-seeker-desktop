//! A library crate with common functionality of the different Lore Seeker Desktop frontends.

#![cfg_attr(test, deny(warnings))]
#![warn(trivial_casts)]
#![deny(missing_docs, unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate itertools;
#[cfg(windows)] extern crate native_windows_gui as nwg;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate tempfile;
#[macro_use] extern crate wrapped_enum;

pub mod github;
pub mod trice;
pub mod update;
pub mod util;
pub mod version;
