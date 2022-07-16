use super::*;
use clap::clap_derive::ArgEnum;
use std::cmp::Ordering;

/// A property to sort by.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum SortBy {
    /// Sort by file name.
    FileName,

    /// Sort by file size.
    FileSize,

    /// Sort by track title.
    Title,

    /// Sort by artist.
    Artist,

    /// Sort by album.
    Album,

    /// Sort by year.
    Year,

    /// Sort by track number.
    Track,

    /// Sort by genre.
    Genre,
}

/// Performs a case insensitive comparison. The sort order vectors are used for the comparison if provided.
#[inline]
fn cmp_vec_string(
    a: &Vec<String>,
    b: &Vec<String>,
    a_sort_order: &Option<Vec<String>>,
    b_sort_order: &Option<Vec<String>>,
) -> Ordering {
    a_sort_order
        .as_ref()
        .unwrap_or(a)
        .iter()
        .map(|s| s.to_lowercase())
        .cmp(b_sort_order.as_ref().unwrap_or(b).iter().map(|s| s.to_lowercase()))
}

/// Compares the given key for an `Entry`.
#[inline]
fn cmp_entry_key(a: &Entry, b: &Entry, key: &SortBy) -> Ordering {
    match key {
        SortBy::FileName => a.file_name.cmp(&b.file_name),
        SortBy::FileSize => a.file_size.cmp(&b.file_size),
        SortBy::Title => cmp_vec_string(&a.title, &b.title, &a.title_sort_order, &b.title_sort_order),
        SortBy::Artist => cmp_vec_string(&a.artist, &b.artist, &a.artist_sort_order, &b.artist_sort_order),
        SortBy::Album => cmp_vec_string(&a.album, &b.album, &a.album_sort_order, &b.album_sort_order),
        SortBy::Year => a.year.cmp(&b.year),
        SortBy::Track => a.track.cmp(&b.track),
        SortBy::Genre => cmp_vec_string(&a.genre, &b.genre, &None, &None),
    }
}

/// Compares the given keys for an `Entry` in order. If the comparison for the first key yields an equal result, the
/// next key is compared and the process repeats until either the result is non-equal or all keys have been compared.
pub fn cmp_entry(a: &Entry, b: &Entry, keys: &[SortBy]) -> Ordering {
    if keys.is_empty() {
        return Ordering::Equal;
    }
    match cmp_entry_key(a, b, &keys[0]) {
        Ordering::Equal => cmp_entry(a, b, &keys[1..]),
        other => other,
    }
}
