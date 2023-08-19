use cache::Cache;
use clap::{arg, Parser};

mod cache;
mod id;
mod item;
mod makers;

fn pretty_print_int(i: isize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}

#[derive(Debug, Parser)]
struct Cli {
    /// query latest prices regardless of cache
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let cache = Cache::new(cli.force)?;
    let mut report = makers::report(&cache);
    report.sort();
    report.reverse();

    let pp = pretty_print_int;
    for r in report {
        println!(
            "{:>30}: {:>12}, [g: {:>12}, c: {:>12}, v: {:>12}, l: {:>7}]",
            r.name,
            pp(r.profit),
            pp(r.gross as isize),
            pp(r.cost as isize),
            pp(r.volume as isize),
            pp(r.limit as isize),
        );
    }

    Ok(())
}
