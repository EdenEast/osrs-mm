use super::{Maker, ReportEntry};

pub struct Decanting;

/// Potion 3 id, Potion 4 id
#[rustfmt::skip]
const POTION_IDS: &[(usize, usize)] = &[
    (3034,  3032),  (2454,  2452),
    (121,   2428),  (22464, 22461),
    (22452, 22449), (9741,  9739),
    (6472,  6470),  (133,   2432),
    (24638, 24635), (24626, 24623),
    (23748, 23745), (23736, 23733),
    (23700, 23697), (23688, 23685),
    (23724, 23721), (23712, 23709),
    (3010,  3008),  (151,   2438),
    (10000, 9998),  (3042,  3040),
    (139,   2434),  (169,   2444),
    (127,   2430),  (12627, 12625),
    (115,   113),   (21981, 21978),
    (12697, 12695),
];

impl Maker for Decanting {
    fn run(cache: &crate::cache::Cache) -> super::Report {
        let mut report = Vec::new();

        for (id_3, id_4) in POTION_IDS {
            let item_3 = cache.get(*id_3);
            let item_4 = cache.get(*id_4);

            let limit = item_3.item.limit.unwrap();
            let decant_total = (limit * 3) / 4;
            let cost = item_3.low() * limit;
            let gross = item_4.high() * decant_total;

            // Only care about potions that will have some movement on the ge
            if item_4.volume > 100_000 {
                report.push(ReportEntry::new(
                    item_3.item.name.as_str(),
                    gross,
                    cost,
                    limit,
                    item_4.volume,
                ));
            }
        }

        report
    }
}
