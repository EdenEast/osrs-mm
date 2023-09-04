use super::{Maker, Report, ReportEntry};
use crate::id::*;

pub struct Bolts;

impl Maker for Bolts {
    fn run(cache: &crate::cache::Cache) -> Report {
        let mut report = Vec::new();

        let law = cache.get(ID_LAW_RUNE);
        let cosmic = cache.get(ID_COSMIC_RUNE);
        let blood = cache.get(ID_BLOOD_RUNE);
        let soul = cache.get(ID_SOUL_RUNE);

        let mut variant = |unf, tips, bolts, ench_bolt, ench_rune| {
            let unf = cache.get(unf);
            let tip = cache.get(tips);
            let bolts = cache.get(bolts);
            let ench_bolt = cache.get(ench_bolt);
            let limit = tip.item.limit.unwrap();
            let cast_cost = ench_rune * (limit / 10);
            let mat_cost = (unf.low() + tip.high()) * limit;
            let cost = cast_cost + mat_cost;

            {
                let gross = bolts.high() * limit;
                report.push(ReportEntry::new(
                    bolts.item.name.as_str(),
                    gross,
                    mat_cost,
                    limit,
                    bolts.volume,
                ));
            }
            {
                let gross = ench_bolt.high() * limit;
                report.push(ReportEntry::new(
                    ench_bolt.item.name.as_str(),
                    gross,
                    cost,
                    limit,
                    ench_bolt.volume,
                ));
            }
        };

        // Diamond dragon bolt (e) Enchant cast is 2 law, 1 cosmic and 5 earth (use staff)
        variant(
            ID_DRAGON_BOLT_UNF,
            ID_DIAMOND_BOLT_TIPS,
            ID_DIAMOND_DRAGON_BOLT,
            ID_DIAMOND_DRAGON_BOLT_E,
            (law.avg() * 2) + cosmic.avg(),
        );

        // Ruby dragon bolt (e) Enchant cast is 1 blood and 1 cosmic and 5 fire (use staff)
        variant(
            ID_DRAGON_BOLT_UNF,
            ID_RUBY_BOLT_TIPS,
            ID_RUBY_DRAGON_BOLT,
            ID_RUBY_DRAGON_BOLT_E,
            blood.avg() + cosmic.avg(),
        );

        // Dragonstone dragon bolt (e) Enchant cast is 1 soul and 1 cosmic and 15 earth
        variant(
            ID_DRAGON_BOLT_UNF,
            ID_DRAGONSTONE_BOLT_TIPS,
            ID_DRAGONSTONE_DRAGON_BOLT,
            ID_DRAGONSTONE_DRAGON_BOLT_E,
            soul.avg() + cosmic.avg(),
        );

        // Opal dragon bolt (e) Enachant cast is 1 cosmic rune and 2 air
        variant(
            ID_DRAGON_BOLT_UNF,
            ID_OPAL_BOLT_TIPS,
            ID_OPAL_DRAGON_BOLT,
            ID_OPAL_DRAGON_BOLT_E,
            cosmic.avg(),
        );

        report
    }
}
