#![warn(missing_docs)]

//! List MP3s with title, artist, album, year, track and genre metadata.
//!
//! This module contains basic methods to list and compare (for sorting) MP3 files from the local filesystem.

mod cmp;
mod error;
mod info;
mod list;

pub use cmp::*;
pub use error::*;
pub use info::*;
pub use list::*;
