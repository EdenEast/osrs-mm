use crate::id::{
    ID_ASTRAL_RUNE, ID_MAHOGANY_LOG, ID_MAHOGANY_PLANK, ID_NATURE_RUNE, ID_TEAK_LOG, ID_TEAK_PLANK,
};

use super::{Maker, ReportEntry};

pub struct Plank;

impl Maker for Plank {
    fn run(cache: &crate::cache::Cache) -> super::Report {
        let mut report = Vec::new();

        let astral = cache.get(ID_ASTRAL_RUNE);
        let nature = cache.get(ID_NATURE_RUNE);
        let rune_cast_cost = (astral.avg() * 2) + nature.avg();
        let limit = 3000;

        let mut variant = |plank, log, cast_price| {
            let plank = cache.get(plank);
            let log = cache.get(log);
            let single_cast_cost = rune_cast_cost + log.low() + cast_price;
            let cost = single_cast_cost * limit;

            let gross = plank.high() * limit;
            report.push(ReportEntry::new(
                plank.item.name.as_str(),
                gross,
                cost,
                limit,
                plank.volume,
            ));
        };

        variant(ID_MAHOGANY_PLANK, ID_MAHOGANY_LOG, 1050);
        variant(ID_TEAK_PLANK, ID_TEAK_LOG, 350);

        report
    }
}
