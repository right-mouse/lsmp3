use super::*;
use glob::glob;
use id3::{Tag, TagLike};
use std::{ffi::OsString, fs::canonicalize, path::PathBuf};

pub fn list(file: &str) -> Result<Info, LsError> {
    let path = if file.is_empty() {
        match std::env::current_dir() {
            Ok(d) => d,
            Err(err) => return Err(LsError::IoCwdError(err)),
        }
    } else {
        PathBuf::from(file)
    };
    if !path.is_dir() && !path.is_file() {
        return Err(LsError::InvalidPath(path.into_os_string()));
    }

    let (results, file_type) = if path.is_dir() {
        let canonical_path = match canonicalize(&path) {
            Ok(p) => p,
            Err(err) => return Err(LsError::IoCanonError(path.into_os_string(), err)),
        };
        let expr = format!("{}/{}", canonical_path.into_os_string().into_string()?, "*.mp3");
        (glob(&expr)?.into_iter().collect(), FileType::Directory)
    } else {
        (vec![Ok(path)], FileType::File)
    };

    let mut entries = Vec::new();
    for result in results {
        let file = result?;
        let file_name = OsString::from(file.file_name().unwrap_or_default());
        let file_size = match file.metadata() {
            Ok(d) => d.len(),
            Err(err) => return Err(LsError::IoReadError(file_name, err)),
        };
        let tag = match Tag::read_from_path(&file) {
            Ok(t) => t,
            Err(err) => return Err(LsError::Id3Error(file_name, err)),
        };
        entries.push(Entry {
            file_name,
            file_size,
            title: tag
                .text_values_for_frame_id("TIT2")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            artist: tag
                .text_values_for_frame_id("TPE1")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            album: tag
                .text_values_for_frame_id("TALB")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            genre: tag
                .text_values_for_frame_id("TCON")
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            year: tag.year(),
            track: Track {
                number: tag.track(),
                total: tag.total_tracks(),
            },
        });
    }
    Ok(Info {
        path: if file.is_empty() { "$PWD" } else { file }.to_string(),
        entries,
        file_type,
    })
}
