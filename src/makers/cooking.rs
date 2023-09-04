use super::{Maker, Report, ReportEntry};

pub struct Cooking;

const IDS: &[(usize, usize)] = &[
    // Lobster
    (377, 379),
    // Swordfish
    (371, 373),
    // monkfish
    (7944, 7946),
    // shark
    (383, 385),
    // karambwan
    (3142, 3144),
    // mantaray
    (389, 391),
];

impl Maker for Cooking {
    fn run(cache: &crate::cache::Cache) -> Report {
        let mut report = Vec::new();

        let items = IDS
            .iter()
            .map(|(id_uncooked, id_cooked)| (cache.get(*id_uncooked), cache.get(*id_cooked)))
            .collect::<Vec<_>>();

        for (uncooked, cooked) in items {
            let limit = uncooked.item.limit.unwrap();
            let cost = uncooked.low() * limit;
            let gross = cooked.high() * limit;
            report.push(ReportEntry::new(
                &format!("Cooking {}", cooked.item.name),
                gross,
                cost,
                limit,
                cooked.volume,
            ));
        }

        report
    }
}
