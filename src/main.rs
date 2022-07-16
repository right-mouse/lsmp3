#![warn(missing_docs)]

//! List MP3s with title, artist, album, year, track and genre metadata.
//!
//! Works similar to `ls`, but ignores all files that are not MP3s with valid ID3 tags. Various options are provided for
//! sorting. In addition to a human readable table format, JSON output is also supported.

use clap::{clap_derive::ArgEnum, CommandFactory, Parser};
use serde_json::{json, Value};
use std::{error::Error, io::Write};
use tabled::Table;

pub mod ls;

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
    #[clap(multiple = true)]
    #[clap(value_parser)]
    file: Vec<String>,

    /// Output format to use
    #[clap(long = "format", short = 'f')]
    #[clap(value_name = "WORD")]
    #[clap(arg_enum)]
    #[clap(default_value = "table")]
    #[clap(value_parser)]
    format: Format,

    /// Sort by WORD (can be set multiple times)
    #[clap(long = "sort", short = 's')]
    #[clap(value_name = "WORD")]
    #[clap(arg_enum)]
    #[clap(multiple = true)]
    #[clap(number_of_values = 1)]
    #[clap(default_value = "file-name")]
    #[clap(value_parser)]
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
    }
}

#[inline]
fn to_json(res: &[ls::Entry]) -> Value {
    serde_json::to_value(res).unwrap_or_else(|err| error(err))
}

fn main() {
    let args = Args::parse();

    let results = ls::list(&args.file, &ls::ListOptions { sort_by: &args.sort_by }).unwrap_or_else(|err| error(err));
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
            f.sort_unstable_by(|a, b| ls::cmp_entry(a, b, &args.sort_by));
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
