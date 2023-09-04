use std::vec;

use crate::cache::Cache;

mod bolts;
mod clay;
mod cooking;
mod decanting;
mod herbs;
mod leather;
mod planks;

pub type Report = Vec<ReportEntry>;

#[derive(Debug)]
pub struct ReportEntry {
    pub name: String,
    pub gross: usize,
    pub cost: usize,
    pub limit: usize,
    pub profit: isize,
    pub volume: usize,
    // pub rank: f32,
}

impl ReportEntry {
    pub fn new(name: &str, gross: usize, cost: usize, limit: usize, volume: usize) -> Self {
        let profit = gross as isize - cost as isize;
        // TODO: weight methods that dont have high volume lower even if the profit high
        // let rank = ((limit as f32 / volume as f32) * 1.0) * profit as f32; //  * 1.0;

        Self {
            profit,
            name: name.to_string(),
            gross,
            cost,
            limit,
            volume,
            // rank,
        }
    }
}

impl Ord for ReportEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.profit.cmp(&other.profit)
    }
}

impl Eq for ReportEntry {}

impl PartialOrd for ReportEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.profit.partial_cmp(&other.profit)
    }
}

impl PartialEq for ReportEntry {
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
        cooking::Cooking::run(cache),
        decanting::Decanting::run(cache),
        herbs::Herbs::run(cache),
        leather::Leather::run(cache),
        planks::Plank::run(cache),
    ]
    .into_iter()
    .flatten()
    .collect()
}
