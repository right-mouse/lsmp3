use lsmp3::*;
use std::{env, path::PathBuf};

/// Creates an owned String or OsString from a string literal.
macro_rules! s {
    ($str:literal) => {
        $str.into()
    };
}

#[inline]
fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata")
}

#[test]
fn test_list_single_file() {
    let path = test_data_dir()
        .join("id3v24_most_tags.mp3")
        .into_os_string()
        .into_string()
        .unwrap();
    assert_eq!(
        list(
            &vec![path.clone()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            }
        )
        .unwrap(),
        vec![Info {
            path,
            path_type: PathType::File,
            entries: vec![Entry {
                file_name: s!("id3v24_most_tags.mp3"),
                file_size: 23017,
                title: vec![s!("Best Song Ever")],
                title_sort_order: None,
                artist: vec![s!("Someone")],
                artist_sort_order: None,
                album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                album_sort_order: None,
                year: Some(2002),
                track: Track {
                    number: Some(3),
                    total: None
                },
                genre: vec![s!("Pop")]
            }]
        }]
    )
}

#[test]
fn test_list_symlink_file() {
    let path = test_data_dir()
        .join("some_tags")
        .join("id3v24_most_tags.mp3")
        .into_os_string()
        .into_string()
        .unwrap();
    assert_eq!(
        list(
            &vec![path.clone()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            }
        )
        .unwrap(),
        vec![Info {
            path,
            path_type: PathType::File,
            entries: vec![Entry {
                file_name: s!("id3v24_most_tags.mp3"),
                file_size: 23017,
                title: vec![s!("Best Song Ever")],
                title_sort_order: None,
                artist: vec![s!("Someone")],
                artist_sort_order: None,
                album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                album_sort_order: None,
                year: Some(2002),
                track: Track {
                    number: Some(3),
                    total: None
                },
                genre: vec![s!("Pop")]
            }]
        }]
    )
}

#[test]
fn test_list_multiple_files() {
    let (path1, path2) = (
        test_data_dir()
            .join("id3v23_most_tags.mp3")
            .into_os_string()
            .into_string()
            .unwrap(),
        test_data_dir()
            .join("id3v24_most_tags.mp3")
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    assert_eq!(
        list(
            &vec![path1.clone(), path2.clone()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            }
        )
        .unwrap(),
        vec![
            Info {
                path: path1,
                path_type: PathType::File,
                entries: vec![Entry {
                    file_name: s!("id3v23_most_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                }]
            },
            Info {
                path: path2,
                path_type: PathType::File,
                entries: vec![Entry {
                    file_name: s!("id3v24_most_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                }]
            }
        ]
    )
}

#[test]
fn test_list_invalid_file() {
    assert!(matches!(
        list(
            &vec![test_data_dir()
                .join("no_id3.mp3")
                .into_os_string()
                .into_string()
                .unwrap()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            },
        )
        .err()
        .unwrap(),
        LsError::Id3Error { .. }
    ));
}

#[test]
fn test_list_dir() {
    let path = test_data_dir()
        .join("some_tags")
        .into_os_string()
        .into_string()
        .unwrap();
    assert_eq!(
        list(
            &vec![path.clone()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            }
        )
        .unwrap(),
        vec![Info {
            path,
            path_type: PathType::Directory,
            entries: vec![
                Entry {
                    file_name: s!("id3v23_most_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v23_some_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                },
                Entry {
                    file_name: s!("id3v24_most_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v24_some_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                }
            ]
        }]
    )
}

#[test]
fn test_list_symlink_dir() {
    let path = test_data_dir()
        .join("most_tags")
        .into_os_string()
        .into_string()
        .unwrap();
    assert_eq!(
        list(
            &vec![path.clone()],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            }
        )
        .unwrap(),
        vec![Info {
            path,
            path_type: PathType::Directory,
            entries: vec![
                Entry {
                    file_name: s!("id3v23_most_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v23_some_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                },
                Entry {
                    file_name: s!("id3v24_most_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v24_some_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                }
            ]
        }]
    )
}

#[test]
fn test_list_multiple_dirs() {
    let (path1, path2) = (
        test_data_dir()
            .join("some_tags")
            .into_os_string()
            .into_string()
            .unwrap(),
        test_data_dir()
            .join("most_tags")
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    let results = list(
        &vec![path1.clone(), path2.clone()],
        &ListOptions {
            sort_by: &[SortBy::FileName],
            reverse: &false,
            recursive: &false,
        },
    )
    .unwrap();

    // The tags are already checked in other tests, so just check the order.
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].path, path1);
    assert_eq!(results[0].entries.len(), 4);
    assert_eq!(results[0].entries[0].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[0].entries[1].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[0].entries[2].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[0].entries[3].file_name, "id3v24_some_tags.mp3");
    assert_eq!(results[1].path, path2);
    assert_eq!(results[1].entries.len(), 4);
    assert_eq!(results[1].entries[0].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[1].entries[1].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[1].entries[2].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[1].entries[3].file_name, "id3v24_some_tags.mp3");
}

#[test]
fn test_list_cwd() {
    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(test_data_dir()).is_ok());
    assert_eq!(
        list(
            &vec![],
            &ListOptions {
                sort_by: &[SortBy::FileName],
                reverse: &false,
                recursive: &false,
            },
        )
        .unwrap(),
        vec![Info {
            path: s!("."),
            path_type: PathType::Directory,
            entries: vec![
                Entry {
                    file_name: s!("id3v23_all_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever"), s!("Really Cool Song")],
                    title_sort_order: Some(vec![s!("Ever, Best Song")]),
                    artist: vec![s!("Someone"), s!("Noone")],
                    artist_sort_order: Some(vec![s!("One, Some")]),
                    album: vec![
                        s!("Billboard Year-End Hot 100 singles of 2002"),
                        s!("Top 100 Hits of 2002")
                    ],
                    album_sort_order: Some(vec![s!("2002, Hot 100 Singles")]),
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: Some(100)
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v23_most_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v23_no_tags.mp3"),
                    file_size: 22950,
                    title: vec![],
                    title_sort_order: None,
                    artist: vec![],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: None,
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                },
                Entry {
                    file_name: s!("id3v23_some_tags.mp3"),
                    file_size: 22993,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                },
                Entry {
                    file_name: s!("id3v24_all_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever"), s!("Really Cool Song")],
                    title_sort_order: Some(vec![s!("Ever, Best Song")]),
                    artist: vec![s!("Someone"), s!("Noone")],
                    artist_sort_order: Some(vec![s!("One, Some")]),
                    album: vec![
                        s!("Billboard Year-End Hot 100 singles of 2002"),
                        s!("Top 100 Hits of 2002")
                    ],
                    album_sort_order: Some(vec![s!("2002, Hot 100 Singles")]),
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: Some(100)
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v24_most_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![s!("Billboard Year-End Hot 100 singles of 2002")],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: Some(3),
                        total: None
                    },
                    genre: vec![s!("Pop")]
                },
                Entry {
                    file_name: s!("id3v24_no_tags.mp3"),
                    file_size: 22950,
                    title: vec![],
                    title_sort_order: None,
                    artist: vec![],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: None,
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                },
                Entry {
                    file_name: s!("id3v24_some_tags.mp3"),
                    file_size: 23017,
                    title: vec![s!("Best Song Ever")],
                    title_sort_order: None,
                    artist: vec![s!("Someone")],
                    artist_sort_order: None,
                    album: vec![],
                    album_sort_order: None,
                    year: Some(2002),
                    track: Track {
                        number: None,
                        total: None
                    },
                    genre: vec![]
                }
            ]
        }]
    );
    assert!(env::set_current_dir(cwd).is_ok());
}

#[test]
fn test_list_dir_recursive() {
    let path = test_data_dir().into_os_string().into_string().unwrap();
    let results = list(
        &vec![path.clone()],
        &ListOptions {
            sort_by: &[SortBy::FileName],
            reverse: &false,
            recursive: &true,
        },
    )
    .unwrap();

    // The tags are already checked in other tests, so just check the order.
    let (subpath1, subpath2) = (
        test_data_dir()
            .join("most_tags")
            .into_os_string()
            .into_string()
            .unwrap(),
        test_data_dir()
            .join("some_tags")
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].path, path);
    assert_eq!(results[0].entries.len(), 8);
    assert_eq!(results[0].entries[0].file_name, "id3v23_all_tags.mp3");
    assert_eq!(results[0].entries[1].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[0].entries[2].file_name, "id3v23_no_tags.mp3");
    assert_eq!(results[0].entries[3].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[0].entries[4].file_name, "id3v24_all_tags.mp3");
    assert_eq!(results[0].entries[5].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[0].entries[6].file_name, "id3v24_no_tags.mp3");
    assert_eq!(results[0].entries[7].file_name, "id3v24_some_tags.mp3");
    assert_eq!(results[1].path, subpath1);
    assert_eq!(results[1].entries.len(), 4);
    assert_eq!(results[1].entries[0].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[1].entries[1].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[1].entries[2].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[1].entries[3].file_name, "id3v24_some_tags.mp3");
    assert_eq!(results[2].path, subpath2);
    assert_eq!(results[2].entries.len(), 4);
    assert_eq!(results[2].entries[0].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[2].entries[1].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[2].entries[2].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[2].entries[3].file_name, "id3v24_some_tags.mp3");
}

#[test]
fn test_order_reverse() {
    let path = test_data_dir().into_os_string().into_string().unwrap();
    let results = list(
        &vec![path.clone()],
        &ListOptions {
            sort_by: &[SortBy::FileName],
            reverse: &true,
            recursive: &false,
        },
    )
    .unwrap();

    // The tags are already checked in other tests, so just check the order.
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].path, path);
    assert_eq!(results[0].entries.len(), 8);
    assert_eq!(results[0].entries[0].file_name, "id3v24_some_tags.mp3");
    assert_eq!(results[0].entries[1].file_name, "id3v24_no_tags.mp3");
    assert_eq!(results[0].entries[2].file_name, "id3v24_most_tags.mp3");
    assert_eq!(results[0].entries[3].file_name, "id3v24_all_tags.mp3");
    assert_eq!(results[0].entries[4].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[0].entries[5].file_name, "id3v23_no_tags.mp3");
    assert_eq!(results[0].entries[6].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[0].entries[7].file_name, "id3v23_all_tags.mp3");
}

#[test]
fn test_order_by_multiple_fields() {
    let path = test_data_dir().into_os_string().into_string().unwrap();
    let results = list(
        &vec![path.clone()],
        &ListOptions {
            sort_by: &[SortBy::Album, SortBy::Title, SortBy::Track, SortBy::FileName],
            reverse: &false,
            recursive: &false,
        },
    )
    .unwrap();

    // The tags are already checked in other tests, so just check the order.
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].path, path);
    assert_eq!(results[0].entries.len(), 8);
    assert_eq!(results[0].entries[0].file_name, "id3v23_no_tags.mp3");
    assert_eq!(results[0].entries[1].file_name, "id3v24_no_tags.mp3");
    assert_eq!(results[0].entries[2].file_name, "id3v23_some_tags.mp3");
    assert_eq!(results[0].entries[3].file_name, "id3v24_some_tags.mp3");
    assert_eq!(results[0].entries[4].file_name, "id3v23_all_tags.mp3");
    assert_eq!(results[0].entries[5].file_name, "id3v24_all_tags.mp3");
    assert_eq!(results[0].entries[6].file_name, "id3v23_most_tags.mp3");
    assert_eq!(results[0].entries[7].file_name, "id3v24_most_tags.mp3");
}
