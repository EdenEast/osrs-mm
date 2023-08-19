use crate::id::{
    ID_ASTRAL_RUNE, ID_BLACK_DRAGONHIDE, ID_BLACK_DRAGON_LEATHER, ID_NATURE_RUNE,
    ID_RED_DRAGONHIDE, ID_RED_DRAGON_LEATHER,
};

use super::{Maker, ReportEntry};

pub struct Leather;

impl Maker for Leather {
    fn run(cache: &crate::cache::Cache) -> super::Report {
        let mut report = Vec::new();

        let astral = cache.get(ID_ASTRAL_RUNE);
        let nature = cache.get(ID_NATURE_RUNE);
        let rune_cast_cost = (astral.avg() * 2) + nature.avg();
        let limit = 3000;
        let ncasts = limit / 5;
        let cast_cost = rune_cast_cost * ncasts;

        let mut variant = |hide, leather| {
            let hide = cache.get(hide);
            let leather = cache.get(leather);
            let cost = (hide.low() * limit) + cast_cost;
            let gross = leather.high() * limit;
            report.push(ReportEntry::new(
                leather.item.name.as_str(),
                gross,
                cost,
                limit,
                leather.volume,
            ));
        };

        variant(ID_BLACK_DRAGONHIDE, ID_BLACK_DRAGON_LEATHER);
        variant(ID_RED_DRAGONHIDE, ID_RED_DRAGON_LEATHER);

        report
    }
}
