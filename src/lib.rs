//! A library crate with common functionality of the different Lore Seeker Desktop frontends.

#![cfg_attr(test, deny(warnings))]
#![warn(trivial_casts)]
#![deny(missing_docs, unused, unused_qualifications)]
#![forbid(unused_import_braces)]

pub mod github;
pub mod trice;
pub mod update;
pub mod util;
pub mod version;
