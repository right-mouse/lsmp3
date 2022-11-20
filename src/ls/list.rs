use super::*;
use id3::TagLike;
use itertools::{Either, Itertools};
use std::{ffi::OsString, iter, path::PathBuf};
use walkdir::WalkDir;

/// The options for listing MP3s.
pub struct ListOptions<'a> {
    /// The list of properties to sort by, in order of priority.
    pub sort_by: &'a [SortBy],
    /// Whether to reverse the order while sorting.
    pub reverse: &'a bool,
    /// Whether to list subdirectories recursively.
    pub recursive: &'a bool,
}

/// Lists MP3s for all the given paths. The paths can be either files or directories. If no paths are provided, the
/// current working directory is used.
pub fn list(paths: &Vec<String>, options: &ListOptions) -> Result<Vec<Info>, LsError> {
    if paths.is_empty() {
        list_path(PathBuf::from("."), options)
    } else {
        paths
            .iter()
            .map(|p| list_path(PathBuf::from(p), options))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.into_iter().flatten().collect())
    }
}

fn list_path(path: PathBuf, options: &ListOptions) -> Result<Vec<Info>, LsError> {
    if !path.is_dir() && !path.is_file() {
        return Err(LsError::InvalidPath(path.into_os_string()));
    }

    let (path_type, walk_entries) = if path.is_dir() {
        // If the given path is a directory, walk through it and attempt to parse all files. Assume the ones that fail
        // to parse aren't mp3 files and skip them.
        (
            PathType::Directory,
            WalkDir::new(&path)
                .max_depth(1)
                .follow_links(true)
                .sort_by_file_name()
                .into_iter()
                .filter_map(|entry| match entry {
                    Ok(dir_entry) => {
                        let file_type = dir_entry.file_type();
                        if file_type.is_file() {
                            match dir_entry.metadata() {
                                Ok(meta) => match id3::Tag::read_from_path(dir_entry.path()) {
                                    Ok(tag) => {
                                        Some(Ok(Either::Left((dir_entry.file_name().to_owned(), meta.len(), tag))))
                                    }
                                    Err(err) => match err.kind {
                                        id3::ErrorKind::Io(err) => {
                                            Some(Err(LsError::IoReadError(dir_entry.into_path().into_os_string(), err)))
                                        }
                                        _ => None, // Assume it's not an mp3 file and skip.
                                    },
                                },
                                Err(err) => Some(Err(LsError::IoReadError(
                                    dir_entry.into_path().into_os_string(),
                                    err.into(),
                                ))),
                            }
                        } else if file_type.is_dir() {
                            if *options.recursive && dir_entry.path() != path {
                                Some(Ok(Either::Right(dir_entry.into_path())))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(err) => Some(Err(LsError::IoReadError(path.as_os_str().to_owned(), err.into()))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )
    } else {
        // If the given path is a file, attempt to parse the file as an mp3.
        (
            PathType::File,
            vec![Either::Left((
                OsString::from(path.file_name().unwrap_or_default()),
                path.metadata()
                    .map_err(|err| LsError::IoReadError(path.as_os_str().to_owned(), err))?
                    .len(),
                id3::Tag::read_from_path(&path).map_err(|err| LsError::Id3Error(path.as_os_str().to_owned(), err))?,
            ))],
        )
    };

    let (files, mut subdirs): (Vec<_>, Vec<_>) = walk_entries.into_iter().partition_map(|entry| entry);
    subdirs.sort_unstable();

    let mut entries: Vec<_> = files
        .into_iter()
        .map(|file| Entry {
            file_name: file.0,
            file_size: file.1,
            title: tag_string_values(&file.2, "TIT2"),
            title_sort_order: tag_option_string_values(&file.2, "TSOT"),
            artist: tag_string_values(&file.2, "TPE1"),
            artist_sort_order: tag_option_string_values(&file.2, "TSOP"),
            album: tag_string_values(&file.2, "TALB"),
            album_sort_order: tag_option_string_values(&file.2, "TSOA"),
            genre: tag_string_values(&file.2, "TCON"),
            year: file.2.year().or_else(|| file.2.date_recorded().map(|d| d.year)),
            track: Track {
                number: file.2.track(),
                total: file.2.total_tracks(),
            },
        })
        .collect();
    entries.sort_unstable_by(|a, b| {
        let ord = cmp_entry(a, b, options.sort_by);
        if *options.reverse {
            ord.reverse()
        } else {
            ord
        }
    });

    iter::once(Ok(vec![Info {
        path: path.to_string_lossy().to_string(),
        path_type,
        entries,
    }]))
    .chain(subdirs.into_iter().map(|p| list_path(p, options)))
    .collect::<Result<Vec<_>, _>>()
    .map(|v| v.into_iter().flatten().collect())
}

#[inline]
fn tag_string_values(tag: &id3::Tag, frame_id: &str) -> Vec<String> {
    tag_option_string_values(tag, frame_id).unwrap_or_default()
}

#[inline]
fn tag_option_string_values(tag: &id3::Tag, frame_id: &str) -> Option<Vec<String>> {
    tag.text_values_for_frame_id(frame_id)
        .map(|v| v.into_iter().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect())
}
