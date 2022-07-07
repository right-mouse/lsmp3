use super::*;
use id3::TagLike;
use std::{ffi::OsString, fs, path::PathBuf};

pub struct ListOptions<'a> {
    pub sort_by: &'a [SortBy],
}

pub fn list(paths: &Vec<String>, options: &ListOptions) -> Result<Vec<Info>, LsError> {
    if paths.len() == 0 {
        let path = std::env::current_dir().map_err(|err| LsError::IoCwdError(err))?;
        Ok(vec![list_path(path, options)?])
    } else {
        paths
            .iter()
            .map(|p| list_path(PathBuf::from(p), options))
            .collect::<Result<Vec<_>, _>>()
    }
}

fn list_path(path: PathBuf, options: &ListOptions) -> Result<Info, LsError> {
    if !path.is_dir() && !path.is_file() {
        return Err(LsError::InvalidPath(path.into_os_string()));
    }

    let (path_type, files) = if path.is_dir() {
        // If the given path is a directory, walk through the directory and attempt to parse all files, skipping the
        // ones that fail to parse (assume they aren't mp3 files).
        (
            PathType::Directory,
            fs::read_dir(&path)
                .map_err(|err| LsError::IoReadError(path.as_os_str().to_owned(), err))?
                .into_iter()
                .filter_map(|entry| match entry {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                match dir_entry.metadata() {
                                    Ok(meta) => match id3::Tag::read_from_path(dir_entry.path()) {
                                        Ok(tag) => Some(Ok((dir_entry.file_name(), meta.len(), tag))),
                                        Err(err) => match err.kind {
                                            id3::ErrorKind::Io(err) => {
                                                Some(Err(LsError::IoReadError(dir_entry.path().into_os_string(), err)))
                                            }
                                            _ => None, // Assume it's not an mp3 file and skip.
                                        },
                                    },
                                    Err(err) => Some(Err(LsError::IoReadError(dir_entry.path().into_os_string(), err))),
                                }
                            } else {
                                None
                            }
                        }
                        Err(err) => Some(Err(LsError::IoReadError(dir_entry.path().into_os_string(), err))),
                    },
                    Err(err) => Some(Err(LsError::IoReadError(path.as_os_str().to_owned(), err))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )
    } else {
        // If the given path is a file, attempt to parse the file as an mp3.
        (
            PathType::File,
            vec![(
                OsString::from(path.file_name().unwrap_or_default()),
                path.metadata()
                    .map_err(|err| LsError::IoReadError(path.as_os_str().to_owned(), err))?
                    .len(),
                id3::Tag::read_from_path(&path).map_err(|err| LsError::Id3Error(path.as_os_str().to_owned(), err))?,
            )],
        )
    };

    let mut entries: Vec<_> = files
        .into_iter()
        .map(|file| Entry {
            file_name: file.0,
            file_size: file.1,
            title: file
                .2
                .text_values_for_frame_id("TIT2")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            artist: file
                .2
                .text_values_for_frame_id("TPE1")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            album: file
                .2
                .text_values_for_frame_id("TALB")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            genre: file
                .2
                .text_values_for_frame_id("TCON")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            year: file.2.year(),
            track: Track {
                number: file.2.track(),
                total: file.2.total_tracks(),
            },
        })
        .collect();
    entries.sort_unstable_by(|a, b| cmp_entry(a, b, options.sort_by));
    Ok(Info {
        path: path.to_string_lossy().to_string(),
        entries,
        path_type,
    })
}
