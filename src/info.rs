use serde::{
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::ffi::OsString;
use tabled::Tabled;

fn display_os_string(os_str: &OsString) -> String {
    os_str.to_string_lossy().to_string()
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

fn display_vec_string(v: &[String]) -> String {
    v.join("/")
}

fn serialize_vec_string<S>(v: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v.len() {
        0 => s.serialize_none(),
        1 => s.serialize_str(&v[0]),
        len => {
            let mut seq = s.serialize_seq(Some(len))?;
            for e in v {
                seq.serialize_element(e)?;
            }
            seq.end()
        }
    }
}

fn display_track(track: &Track) -> String {
    match track.number {
        Some(n) => {
            let mut s = n.to_string();
            if let Some(t) = track.total {
                s.push('/');
                s.push_str(&t.to_string());
            }
            s
        }
        None => Default::default(),
    }
}

fn is_track_empty(track: &Track) -> bool {
    track.number.is_none()
}

/// Converts a size to a human readable size. Borrowed from https://github.com/dustin/go-humanize, licensed under the
/// MIT license.
fn human_readable_size(s: &u64) -> String {
    const SUFFIXES: &[&str] = &["B", "kiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    const BASE: f64 = 1024.0;
    if *s < 10 {
        return format!("{:3} {}", s, SUFFIXES[0]);
    }
    let e = (*s as f64).log(BASE).floor();
    let suffix = SUFFIXES[e as usize];
    let val = (*s as f64 / BASE.powf(e) * 10.0 + 0.5).floor() / 10.0;
    format!("{:3.precision$} {}", val, suffix, precision = usize::from(val < 10.0))
}

/// The type of a list path.
#[derive(Debug, PartialEq, Eq)]
pub enum PathType {
    /// A file path.
    File,

    /// A directory path.
    Directory,
}

/// A container for the results of a list operation along with the original path and the path type.
#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    /// The path that was listed.
    pub path: String,

    /// The type of the path.
    pub path_type: PathType,

    /// The results of the list operation.
    pub entries: Vec<Entry>,
}

/// The track metadata for a file.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Track {
    /// The track number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// The total number of tracks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
}

/// A result from a list operation.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Tabled)]
pub struct Entry {
    /// The name of the file.
    #[tabled(rename = "NAME")]
    #[tabled(display_with = "display_os_string")]
    #[serde(serialize_with = "serialize_os_string")]
    pub name: OsString,

    /// The size of the file.
    #[tabled(rename = "SIZE")]
    #[tabled(display_with = "human_readable_size")]
    pub size: u64,

    /// The track title.
    #[tabled(rename = "TITLE")]
    #[tabled(display_with = "display_vec_string")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_vec_string")]
    pub title: Vec<String>,

    /// The track title sort order (only used for sorting, if present).
    #[tabled(skip)]
    #[serde(skip_serializing)]
    pub title_sort_order: Option<Vec<String>>,

    /// The artist.
    #[tabled(rename = "ARTIST")]
    #[tabled(display_with = "display_vec_string")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_vec_string")]
    pub artist: Vec<String>,

    /// The artist sort order (only used for sorting, if present).
    #[tabled(skip)]
    #[serde(skip_serializing)]
    pub artist_sort_order: Option<Vec<String>>,

    /// The album.
    #[tabled(rename = "ALBUM")]
    #[tabled(display_with = "display_vec_string")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_vec_string")]
    pub album: Vec<String>,

    /// The album sort order (only used for sorting, if present).
    #[tabled(skip)]
    #[serde(skip_serializing)]
    pub album_sort_order: Option<Vec<String>>,

    /// The year.
    #[tabled(rename = "YEAR")]
    #[tabled(display_with = "display_option_i32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,

    /// The track number.
    #[tabled(rename = "TRACK")]
    #[tabled(display_with = "display_track")]
    #[serde(skip_serializing_if = "is_track_empty")]
    pub track: Track,

    /// The genre.
    #[tabled(rename = "GENRE")]
    #[tabled(display_with = "display_vec_string")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_vec_string")]
    pub genre: Vec<String>,
}
