use super::{Info, LsError};
use glob::glob;
use id3::{Tag, TagLike};
use std::{ffi::OsString, fs::canonicalize, path::PathBuf};

pub fn list(dir: &str) -> Result<Vec<Info>, LsError> {
    let path = if dir.is_empty() {
        match std::env::current_dir() {
            Ok(d) => d,
            Err(err) => return Err(LsError::IoCwdError(err)),
        }
    } else {
        PathBuf::from(dir)
    };
    if !path.is_dir() {
        return Err(LsError::InvalidPath(path.into_os_string()));
    }
    let canonical_path = match canonicalize(&path) {
        Ok(p) => p,
        Err(err) => return Err(LsError::IoCanonError(path.into_os_string(), err)),
    };
    let expr = format!("{}/{}", canonical_path.into_os_string().into_string()?, "*.mp3");

    let mut results = Vec::new();
    for entry in glob(&expr)? {
        let file = entry?;
        let file_name = OsString::from(file.file_name().unwrap_or_default());
        let file_size = match file.metadata() {
            Ok(d) => d.len(),
            Err(err) => return Err(LsError::IoReadError(file_name, err)),
        };
        let tag = match Tag::read_from_path(&file) {
            Ok(t) => t,
            Err(err) => return Err(LsError::Id3Error(file_name, err)),
        };
        results.push(Info {
            file_name,
            file_size,
            title: tag.text_values_for_frame_id("TIT2").map(|v| v.join("/")),
            artist: tag.text_values_for_frame_id("TPE1").map(|v| v.join("/")),
            album: tag.text_values_for_frame_id("TALB").map(|v| v.join("/")),
            track: tag.track().map(|n| {
                let mut s = n.to_string();
                match tag.total_tracks() {
                    Some(t) => {
                        s.push_str("/");
                        s.push_str(&t.to_string());
                    }
                    _ => {}
                }
                s
            }),
        });
    }
    Ok(results)
}
