#![warn(missing_docs)]

//! List MP3s with title, artist, album, year, track and genre metadata.
//!
//! Works similar to `ls`, but ignores all files that are not MP3s with valid ID3 tags. Various options are provided for
//! sorting. In addition to a human readable table format, JSON output is also supported.

use clap::{clap_derive::ArgEnum, CommandFactory, Parser, ValueHint};
use lsmp3::ls;
use serde_json::{json, Value};
use std::{error::Error, io::Write};
use tabled::Table;

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
    JSON,
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
    #[clap(default_value = "file-name")]
    sort_by: Vec<ls::SortBy>,
}

#[inline]
fn to_table(res: &[ls::Entry]) -> String {
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
fn to_json(res: &[ls::Entry]) -> Value {
    serde_json::to_value(res).unwrap_or_else(|err| error(err))
}

fn main() {
    let args = Args::parse();

    let results = ls::list(
        &args.file,
        &ls::ListOptions {
            sort_by: &args.sort_by,
            reverse: &args.reverse,
            recursive: &args.recursive,
        },
    )
    .unwrap_or_else(|err| error(err));
    let (mut tables, mut values): (Vec<String>, Vec<Value>) = match args.format {
        Format::Table => (Vec::with_capacity(results.len()), Vec::with_capacity(0)),
        Format::JSON => (Vec::with_capacity(0), Vec::with_capacity(results.len())),
    };
    if results.len() == 1 {
        match args.format {
            Format::Table => tables.push(to_table(&results[0].entries)),
            Format::JSON => values.push(to_json(&results[0].entries)),
        }
    } else {
        let (files, dirs): (Vec<_>, Vec<_>) = results.into_iter().partition(|f| f.path_type == ls::PathType::File);
        if !files.is_empty() {
            let mut f = files.into_iter().map(|f| f.entries).flatten().collect::<Vec<_>>();
            f.sort_unstable_by(|a, b| {
                let ord = ls::cmp_entry(a, b, &args.sort_by);
                if args.reverse {
                    ord.reverse()
                } else {
                    ord
                }
            });
            match args.format {
                Format::Table => tables.push(to_table(&f)),
                Format::JSON => values.push(to_json(&f)),
            }
        }
        if !dirs.is_empty() {
            match args.format {
                Format::Table => tables.extend(dirs.iter().map(|f| format!("{}:\n{}", f.path, to_table(&f.entries)))),
                Format::JSON => values.extend(dirs.iter().map(|f| {
                    json!({
                        "path": f.path,
                        "values": to_json(&f.entries),
                    })
                })),
            }
        }
    }

    print!(
        "{}",
        match args.format {
            Format::Table => tables.join("\n"),
            Format::JSON => if values.len() == 1 {
                serde_json::to_string(&values[0])
            } else {
                serde_json::to_string(&values)
            }
            .unwrap_or_else(|err| error(err)),
        }
    )
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

    fn get_test_entries() -> Vec<ls::Entry> {
        vec![
            ls::Entry {
                file_name: s!("Some.mp3"),
                file_size: 8080,
                title: vec![s!("Two"), s!("titles")],
                title_sort_order: None,
                artist: vec![s!("Three"), s!("cool"), s!("artists")],
                artist_sort_order: None,
                album: vec![s!("Dual"), s!("Album")],
                album_sort_order: None,
                year: Some(2020),
                track: ls::Track {
                    number: Some(2),
                    total: Some(3),
                },
                genre: vec![s!("Trip-Hop"), s!("Hip-Hop")],
            },
            ls::Entry {
                file_name: s!("None.mp3"),
                file_size: 4,
                title: vec![],
                title_sort_order: None,
                artist: vec![],
                artist_sort_order: None,
                album: vec![],
                album_sort_order: None,
                year: None,
                track: ls::Track {
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
                    "file_name": "Some.mp3",
                    "file_size": 8080,
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
                    "file_name": "None.mp3",
                    "file_size": 4
                }
            ])
        )
    }

    #[test]
    fn verify_args() {
        Args::command().debug_assert()
    }
}
