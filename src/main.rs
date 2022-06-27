use clap::{clap_derive::ArgEnum, CommandFactory, Parser};
use serde_json::{json, Value};
use std::error::Error;
use tabled::Table;

mod ls;

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn error(err: impl Error) -> ! {
    Args::command()
        .error(clap::ErrorKind::Io, capitalize_first_letter(&err.to_string()))
        .exit();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Format {
    Table,
    JSON,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum SortBy {
    FileName,
    FileSize,
    Title,
    Artist,
    Album,
    Year,
    Track,
    Genre,
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

    /// Sort by WORD
    #[clap(long = "sort", short = 's')]
    #[clap(value_name = "WORD")]
    #[clap(arg_enum)]
    #[clap(default_value = "file-name")]
    #[clap(value_parser)]
    sort_by: SortBy,
}

fn to_table(res: &Vec<ls::Entry>) -> String {
    if res.is_empty() {
        Default::default()
    } else {
        Table::new(res)
            .with(tabled::Style::blank())
            .with(tabled::Modify::new(tabled::object::Segment::all()).with(tabled::Alignment::left()))
            .to_string()
    }
}

fn to_json(res: &Vec<ls::Entry>) -> Value {
    serde_json::to_value(res).unwrap_or_else(|err| error(err))
}

fn main() {
    let mut args = Args::parse();
    if args.file.len() == 0 {
        args.file = vec!["".to_string()];
    }

    let results: Vec<_> = args
        .file
        .iter()
        .map(|f| {
            let mut v = ls::list(f).unwrap_or_else(|err| error(err));
            v.entries.sort_unstable_by(|a, b| match args.sort_by {
                SortBy::FileName => a.file_name.cmp(&b.file_name),
                SortBy::FileSize => a.file_size.cmp(&b.file_size),
                SortBy::Title => a.title.cmp(&b.title),
                SortBy::Artist => a.artist.cmp(&b.artist),
                SortBy::Album => a.album.cmp(&b.album),
                SortBy::Year => a.year.cmp(&b.year),
                SortBy::Track => a.track.cmp(&b.track),
                SortBy::Genre => a.genre.cmp(&b.genre),
            });
            v
        })
        .collect();

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
        let (files, dirs): (Vec<_>, Vec<_>) = results.into_iter().partition(|f| f.file_type == ls::FileType::File);
        if !files.is_empty() {
            match args.format {
                Format::Table => tables.push(to_table(&files.into_iter().map(|f| f.entries).flatten().collect())),
                Format::JSON => values.push(to_json(&files.into_iter().map(|f| f.entries).flatten().collect())),
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
