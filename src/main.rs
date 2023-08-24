use cache::Cache;
use clap::{arg, Parser};
use num_format::{Locale, ToFormattedString};

mod cache;
mod id;
mod item;
mod makers;

pub fn fmt_int<T: ToFormattedString>(num: &T) -> String {
    let locale = Locale::en;
    num.to_formatted_string(&locale)
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

    println!(
        "{:>30} | {:>12} | {:>12} | {:>12} | {:>12} | {:>7}",
        "name", "profit", "gross", "cost", "volume", "limit"
    );

    for r in report {
        if r.profit.is_negative() || r.profit < 1_000_000 {
            continue;
        }

        println!(
            "{:>30} | {:>12} | {:>12} | {:>12} | {:>12} | {:>7}",
            r.name,
            fmt_int(&r.profit),
            fmt_int(&r.gross),
            fmt_int(&r.cost),
            fmt_int(&r.volume),
            fmt_int(&r.limit),
        );
    }

    Ok(())
}
