use super::{Maker, Report, ReportEntry};
use crate::id::*;

pub struct Bolts;

impl Maker for Bolts {
    fn run(cache: &crate::cache::Cache) -> Report {
        let mut report = Vec::new();

        let dbolt_unf = cache.get(ID_DRAGON_BOLT_UNF);
        let diamond_tips = cache.get(ID_DIAMOND_BOLT_TIPS);
        let diamond_dragon_bolt_e = cache.get(ID_DIAMOND_DRAGON_BOLT_E);
        let law = cache.get(ID_LAW_RUNE);
        let cosmic = cache.get(ID_COSMIC_RUNE);

        {
            // Diamond dragon bolt (e)
            // Enchant cast is 2 law, 1 cosmic and 5 earth (use staff)
            // One case is 10 bolts
            let limit = diamond_tips.item.limit.unwrap();
            let cast_cost = ((law.avg() * 2) + cosmic.avg()) * (limit / 10);
            let material_cost = (dbolt_unf.low() + diamond_tips.high()) * limit;
            let cost = material_cost + cast_cost;

            let gross = diamond_dragon_bolt_e.high() * limit;
            report.push(ReportEntry::new(
                diamond_dragon_bolt_e.item.name.as_str(),
                gross,
                cost,
                limit,
                diamond_dragon_bolt_e.volume,
            ));
        }

        let ruby_dragon_bolt_e = cache.get(ID_RUBY_DRAGON_BOLT_E);
        let ruby_tips = cache.get(ID_RUBY_BOLT_TIPS);
        let blood = cache.get(ID_BLOOD_RUNE);

        {
            // Ruby dragon bolt (e)
            // Enchant cast is 1 blood and 1 cosmic and 5 fire (use staff)
            // one cast is 10 bolts
            let limit = ruby_tips.item.limit.unwrap();
            let cast_cost = (blood.avg() + cosmic.avg()) * (limit / 10);
            let material_cost = (dbolt_unf.low() + ruby_tips.high()) * limit;
            let cost = material_cost + cast_cost;

            let gross = ruby_dragon_bolt_e.high() * limit;
            report.push(ReportEntry::new(
                ruby_dragon_bolt_e.item.name.as_str(),
                gross,
                cost,
                limit,
                ruby_dragon_bolt_e.volume,
            ));
        }
        report
    }
}
