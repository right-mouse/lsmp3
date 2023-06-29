#![warn(missing_docs)]

//! List MP3s with title, artist, album, year, track and genre metadata.
//!
//! Works similar to `ls`, but ignores all files that are not MP3s with valid ID3 tags. Various options are provided for
//! sorting. In addition to a human readable table format, JSON output is also supported.

use clap::{clap_derive::ArgEnum, CommandFactory, Parser, ValueHint};
use serde_json::{json, Value};
use std::{error::Error, io::Write};
use tabled::Table;

#[inline]
fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn error(err: impl Error) -> ! {
    _ = Args::command()
        .error(clap::ErrorKind::Io, capitalize_first_letter(&err.to_string()))
        .print();
    _ = std::io::stdout().lock().flush();
    _ = std::io::stderr().lock().flush();
    std::process::exit(1)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Format {
    Table,
    Json,
}

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The FILEs to list information about (the current directory by default)
    #[clap(value_hint = ValueHint::AnyPath)]
    file: Vec<String>,

    /// The output format to use
    #[clap(long = "format", short = 'f')]
    #[clap(value_name = "WORD")]
    #[clap(arg_enum)]
    #[clap(default_value = "table")]
    format: Format,

    /// Reverse order while sorting
    #[clap(long = "reverse", short = 'r')]
    reverse: bool,

    /// List subdirectories recursively
    #[clap(long = "recursive", short = 'R')]
    recursive: bool,

    /// Sort by WORD (can be set multiple times)
    #[clap(long = "sort", short = 's')]
    #[clap(value_name = "WORD")]
    #[clap(arg_enum)]
    #[clap(multiple = true)]
    #[clap(number_of_values = 1)]
    #[clap(default_value = "name")]
    sort_by: Vec<lsmp3::SortBy>,
}

#[inline]
fn to_table(res: &[lsmp3::Entry]) -> String {
    if res.is_empty() {
        Default::default()
    } else {
        Table::new(res)
            .with(tabled::Style::blank())
            .with(tabled::Modify::new(tabled::object::Segment::all()).with(tabled::Alignment::left()))
            .to_string()
            + "\n"
    }
}

#[inline]
fn to_json(res: &[lsmp3::Entry]) -> Value {
    serde_json::to_value(res).unwrap_or_else(|err| error(err))
}

fn main() {
    let args = Args::parse();

    let results = lsmp3::list(
        &args.file,
        &lsmp3::ListOptions {
            sort_by: &args.sort_by,
            reverse: &args.reverse,
            recursive: &args.recursive,
        },
    )
    .unwrap_or_else(|err| error(err));
    match args.format {
        Format::Table => {
            let mut tables = Vec::with_capacity(results.len());
            if results.len() == 1 {
                tables.push(to_table(&results[0].entries));
            } else {
                let (files, dirs): (Vec<_>, Vec<_>) =
                    results.into_iter().partition(|f| f.path_type == lsmp3::PathType::File);
                if !files.is_empty() {
                    let mut f = files.into_iter().flat_map(|f| f.entries).collect::<Vec<_>>();
                    f.sort_unstable_by(|a, b| {
                        let ord = lsmp3::cmp_entry(a, b, &args.sort_by);
                        if args.reverse {
                            ord.reverse()
                        } else {
                            ord
                        }
                    });
                    tables.push(to_table(&f));
                }
                if !dirs.is_empty() {
                    tables.extend(dirs.iter().map(|f| format!("{}:\n{}", f.path, to_table(&f.entries))));
                }

                for (i, table) in tables.iter().enumerate() {
                    print!("{}", table);
                    if i < tables.len() - 1 {
                        println!();
                    }
                }
            }
        }
        Format::Json => {
            let mut values = Vec::with_capacity(results.len());
            if results.len() == 1 {
                values.push(to_json(&results[0].entries));
            } else {
                let (files, dirs): (Vec<_>, Vec<_>) =
                    results.into_iter().partition(|f| f.path_type == lsmp3::PathType::File);
                if !files.is_empty() {
                    let mut f = files.into_iter().flat_map(|f| f.entries).collect::<Vec<_>>();
                    f.sort_unstable_by(|a, b| {
                        let ord = lsmp3::cmp_entry(a, b, &args.sort_by);
                        if args.reverse {
                            ord.reverse()
                        } else {
                            ord
                        }
                    });
                    values.push(to_json(&f));
                }
                if !dirs.is_empty() {
                    values.extend(dirs.iter().map(|f| {
                        json!({
                            "path": f.path,
                            "values": to_json(&f.entries),
                        })
                    }));
                }

                print!(
                    "{}",
                    if values.len() == 1 {
                        serde_json::to_string(&values[0])
                    } else {
                        serde_json::to_string(&values)
                    }
                    .unwrap_or_else(|err| error(err))
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates an owned String or OsString from a string literal.
    macro_rules! s {
        ($str:literal) => {
            $str.into()
        };
    }

    fn get_test_entries() -> Vec<lsmp3::Entry> {
        vec![
            lsmp3::Entry {
                name: s!("Some.mp3"),
                size: 8080,
                title: vec![s!("Two"), s!("titles")],
                title_sort_order: None,
                artist: vec![s!("Three"), s!("cool"), s!("artists")],
                artist_sort_order: None,
                album: vec![s!("Dual"), s!("Album")],
                album_sort_order: None,
                year: Some(2020),
                track: lsmp3::Track {
                    number: Some(2),
                    total: Some(3),
                },
                genre: vec![s!("Trip-Hop"), s!("Hip-Hop")],
            },
            lsmp3::Entry {
                name: s!("None.mp3"),
                size: 4,
                title: vec![],
                title_sort_order: None,
                artist: vec![],
                artist_sort_order: None,
                album: vec![],
                album_sort_order: None,
                year: None,
                track: lsmp3::Track {
                    number: None,
                    total: None,
                },
                genre: vec![],
            },
        ]
    }

    #[test]
    fn test_to_table() {
        assert_eq!(
            to_table(&get_test_entries()),
            format!(
                "{}\n{}\n{}\n",
                " NAME       SIZE      TITLE        ARTIST               ALBUM        YEAR   TRACK   GENRE            ",
                " Some.mp3   7.9 kiB   Two/titles   Three/cool/artists   Dual/Album   2020   2/3     Trip-Hop/Hip-Hop ",
                " None.mp3     4 B                                                                                    "
            )
        )
    }

    #[test]
    fn test_to_json() {
        assert_eq!(
            to_json(&get_test_entries()),
            json!([
                {
                    "album": [
                        "Dual",
                        "Album"
                    ],
                    "artist": [
                        "Three",
                        "cool",
                        "artists"
                    ],
                    "name": "Some.mp3",
                    "size": 8080,
                    "genre": [
                        "Trip-Hop",
                        "Hip-Hop"
                    ],
                    "title": [
                        "Two",
                        "titles"
                    ],
                    "track": {
                        "number": 2,
                        "total": 3
                    },
                    "year": 2020
                },
                {
                    "name": "None.mp3",
                    "size": 4
                }
            ])
        )
    }

    #[test]
    fn verify_args() {
        Args::command().debug_assert()
    }
}
