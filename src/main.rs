mod ls;

use main_error::MainResult;
use std::env;
use tabled::{object::Segment, Alignment, Modify, Style, Table};

fn main() -> MainResult {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "" };
    let results = ls::list(dir)?;
    if !results.is_empty() {
        let table = Table::new(results)
            .with(Style::blank())
            .with(Modify::new(Segment::all()).with(Alignment::left()));
        print!("{}", table.to_string());
    }

    Ok(())
}
