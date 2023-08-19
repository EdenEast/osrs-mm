use std::vec;

use crate::cache::Cache;

mod bolts;
mod clay;
mod decanting;
mod leather;
mod planks;

pub type Report<'a> = Vec<ReportEntry<'a>>;

#[derive(Debug)]
pub struct ReportEntry<'a> {
    pub name: &'a str,
    pub gross: usize,
    pub cost: usize,
    pub limit: usize,
    pub profit: isize,
    pub volume: usize,
}

impl<'a> ReportEntry<'a> {
    pub fn new(name: &'a str, gross: usize, cost: usize, limit: usize, volume: usize) -> Self {
        let profit = gross as isize - cost as isize;
        // TODO: weight methods that dont have high volume lower even if the profit high
        // let rank = (profit / limit as isize) * volume as isize;

        Self {
            profit,
            name,
            gross,
            cost,
            limit,
            volume,
        }
    }
}

impl<'a> Ord for ReportEntry<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.profit.cmp(&other.profit)
    }
}

impl<'a> Eq for ReportEntry<'a> {}

impl<'a> PartialOrd for ReportEntry<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.profit.partial_cmp(&other.profit)
    }
}

impl<'a> PartialEq for ReportEntry<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.profit.eq(&other.profit)
    }
}

pub trait Maker {
    fn run(cache: &Cache) -> Report;
}

pub fn report(cache: &Cache) -> Report {
    vec![
        bolts::Bolts::run(cache),
        clay::Clay::run(cache),
        decanting::Decanting::run(cache),
        leather::Leather::run(cache),
        planks::Plank::run(cache),
    ]
    .into_iter()
    .flatten()
    .collect()
}
