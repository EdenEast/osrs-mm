use crate::id::{ID_ASTRAL_RUNE, ID_CLAY, ID_SOFT_CLAY};

use super::{Maker, ReportEntry};

pub struct Clay;

impl Maker for Clay {
    fn run(cache: &crate::cache::Cache) -> super::Report {
        let mut report = Vec::new();

        let clay = cache.get(ID_CLAY);
        let soft_clay = cache.get(ID_SOFT_CLAY);
        let astral = cache.get(ID_ASTRAL_RUNE);

        let limit = clay.item.limit.unwrap();
        let cast_cost = astral.avg() * ((limit / 27) + 1);
        let cost = (clay.low() * limit) + cast_cost;

        let gross = soft_clay.high() * limit;
        report.push(ReportEntry::new(
            soft_clay.item.name.as_str(),
            gross,
            cost,
            limit,
            soft_clay.volume,
        ));

        report
    }
}
