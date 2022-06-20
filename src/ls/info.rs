use std::{ffi::OsString, os::unix::prelude::OsStrExt};
use tabled::Tabled;

fn display_os_string(os_str: &OsString) -> String {
    String::from_utf8_lossy(os_str.as_bytes()).to_string()
}

fn display_option_string(op_str: &Option<String>) -> String {
    match *op_str {
        Some(ref s) => s.clone(),
        None => Default::default(),
    }
}

/// Borrowed from https://github.com/dustin/go-humanize, licensed under the MIT license.
fn human_readable_size(s: &u64) -> String {
    const SUFFIXES: &'static [&'static str] = &["B", "kiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    const BASE: f64 = 1024.0;
    if *s < 10 {
        return format!("{} {}", s, SUFFIXES[0]);
    }
    let e = (*s as f64).log(BASE).floor();
    let suffix = SUFFIXES[e as usize];
    let val = (*s as f64 / BASE.powf(e) * 10.0 + 0.5).floor() / 10.0;
    format!("{0:3.2$} {1}", val, suffix, if val < 10.0 { 1 } else { 0 })
}

#[derive(Tabled)]
pub struct Info {
    #[tabled(rename = "NAME", display_with = "display_os_string")]
    pub file_name: OsString,

    #[tabled(rename = "SIZE", display_with = "human_readable_size")]
    pub file_size: u64,

    #[tabled(rename = "TITLE", display_with = "display_option_string")]
    pub title: Option<String>,

    #[tabled(rename = "ARTIST", display_with = "display_option_string")]
    pub artist: Option<String>,

    #[tabled(rename = "ALBUM", display_with = "display_option_string")]
    pub album: Option<String>,

    #[tabled(rename = "TRACK", display_with = "display_option_string")]
    pub track: Option<String>,
}
