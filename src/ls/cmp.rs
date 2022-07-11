use super::*;
use clap::clap_derive::ArgEnum;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum SortBy {
    FileName,
    FileSize,
    Title,
    Artist,
    Album,
    Year,
    Track,
    Genre,
}

/// Performs a case insensitive comparison.
fn cmp_vec_string(a: &Vec<String>, b: &Vec<String>) -> Ordering {
    a.iter()
        .map(|s| s.to_lowercase())
        .cmp(b.iter().map(|s| s.to_lowercase()))
}

/// Compares the given key for an entry.
fn cmp_entry_key(a: &Entry, b: &Entry, key: &SortBy) -> Ordering {
    match key {
        SortBy::FileName => a.file_name.cmp(&b.file_name),
        SortBy::FileSize => a.file_size.cmp(&b.file_size),
        SortBy::Title => cmp_vec_string(&a.title, &b.title),
        SortBy::Artist => cmp_vec_string(&a.artist, &b.artist),
        SortBy::Album => cmp_vec_string(&a.album, &b.album),
        SortBy::Year => a.year.cmp(&b.year),
        SortBy::Track => a.track.cmp(&b.track),
        SortBy::Genre => cmp_vec_string(&a.genre, &b.genre),
    }
}

/// Compares the given keys for an entry in order. If the comparison for the first key yields an equal result, the next
/// key is compared until either the result is non-equal or all keys have been compared.
pub fn cmp_entry(a: &Entry, b: &Entry, keys: &[SortBy]) -> Ordering {
    if keys.is_empty() {
        return Ordering::Equal;
    }
    match cmp_entry_key(a, b, &keys[0]) {
        Ordering::Equal => cmp_entry(a, b, &keys[1..]),
        other => other,
    }
}
