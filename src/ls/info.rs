use serde::{
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::{ffi::OsString, os::unix::prelude::OsStrExt};
use tabled::Tabled;

fn display_os_string(os_str: &OsString) -> String {
    String::from_utf8_lossy(os_str.as_bytes()).to_string()
}

fn serialize_os_string<S>(os_str: &OsString, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&display_os_string(os_str))
}

fn display_option_i32(op_i32: &Option<i32>) -> String {
    match *op_i32 {
        Some(i) => i.to_string(),
        None => Default::default(),
    }
}

fn display_option_vec_string(op_vec_str: &Option<Vec<String>>) -> String {
    match *op_vec_str {
        Some(ref v) => v.join("/"),
        None => Default::default(),
    }
}

fn serialize_option_vec_string<S>(op_vec_str: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *op_vec_str {
        Some(ref v) => {
            if v.len() == 1 {
                s.serialize_str(&v[0])
            } else {
                let mut seq = s.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
        None => s.serialize_none(),
    }
}

fn display_track(track: &Track) -> String {
    match track.number {
        Some(n) => {
            let mut s = n.to_string();
            match track.total {
                Some(t) => {
                    s.push_str("/");
                    s.push_str(&t.to_string());
                }
                _ => {}
            }
            s
        }
        None => Default::default(),
    }
}

fn is_track_empty(track: &Track) -> bool {
    track.number.is_none()
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Track {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
}

#[derive(Serialize, Deserialize, Tabled)]
pub struct Entry {
    #[tabled(rename = "NAME")]
    #[tabled(display_with = "display_os_string")]
    #[serde(serialize_with = "serialize_os_string")]
    pub file_name: OsString,

    #[tabled(rename = "SIZE")]
    #[tabled(display_with = "human_readable_size")]
    pub file_size: u64,

    #[tabled(rename = "TITLE")]
    #[tabled(display_with = "display_option_vec_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_string")]
    pub title: Option<Vec<String>>,

    #[tabled(rename = "ARTIST")]
    #[tabled(display_with = "display_option_vec_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_string")]
    pub artist: Option<Vec<String>>,

    #[tabled(rename = "ALBUM")]
    #[tabled(display_with = "display_option_vec_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_string")]
    pub album: Option<Vec<String>>,

    #[tabled(rename = "YEAR")]
    #[tabled(display_with = "display_option_i32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,

    #[tabled(rename = "TRACK")]
    #[tabled(display_with = "display_track")]
    #[serde(skip_serializing_if = "is_track_empty")]
    pub track: Track,

    #[tabled(rename = "GENRE")]
    #[tabled(display_with = "display_option_vec_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_string")]
    pub genre: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

pub struct Info {
    pub path: String,
    pub entries: Vec<Entry>,
    pub file_type: FileType,
}
