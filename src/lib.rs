//! A library crate with common functionality of the different Lore Seeker Desktop frontends.

#![cfg_attr(test, deny(warnings))]
#![warn(trivial_casts)]
#![deny(missing_docs, unused, unused_qualifications)]
#![forbid(unused_import_braces)]

extern crate itertools;
extern crate reqwest;
#[macro_use] extern crate serde_derive;

pub mod update;
pub mod version;
