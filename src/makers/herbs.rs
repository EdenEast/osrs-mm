use super::{Maker, ReportEntry};

pub struct Herbs;

// (Grimy, herb, seed, unf potion)
const IDS: &[(usize, usize, usize, usize)] = &[
    // Grimy torstol, Torstol
    (219, 269, 5304, 111),
    // Grimy kwuarm, Kwuarm
    (213, 263, 213, 105),
    // Grimy lantadyme, Lantadyme
    (2485, 2481, 5302, 2483),
    // Grimy ranarr weed, Ranarr weed
    (207, 257, 5295, 99),
    // Grimy guam leaf, Guam leaf
    (199, 249, 5291, 91),
    // Grimy avantoe, Avantoe
    (211, 261, 5298, 103),
    // Grimy toadflax, Toadflax
    (3049, 2998, 5296, 3002),
    // Grimy marrentill, Marrentill
    (201, 251, 5292, 93),
    // Grimy cadantine, Cadantine
    (215, 265, 5301, 107),
    // Grimy harralander, Harralander
    (205, 255, 5294, 97),
    // Grimy snapdragon, Snapdragon
    (3051, 3000, 5300, 3004),
    // Grimy dwarf weed, Dwarf weed
    (217, 267, 5303, 109),
    // Grimy irit leaf, Irit leaf
    (209, 259, 5297, 101),
    // Grimy tarromin, Tarromin
    (203, 253, 5293, 95),
];

impl Maker for Herbs {
    fn run(cache: &crate::cache::Cache) -> super::Report {
        let mut report = Vec::new();

        let items = IDS
            .iter()
            .map(|(id_grimy, id_herb, id_seed, id_unf)| {
                let grimy = cache.get(*id_grimy);
                let herb = cache.get(*id_herb);
                let seed = cache.get(*id_seed);
                let unf = cache.get(*id_unf);

                (grimy, herb, seed, unf)
            })
            .collect::<Vec<_>>();

        for (grimy, herb, _seed, unf) in items {
            // cleaning herbs
            {
                let limit = grimy.item.limit.unwrap();
                let cost = grimy.low() * limit;
                let gross = herb.high() * limit;
                let profit = gross as isize - cost as isize;
                if profit >= 1_000_000 && cost < 20_000_000 {
                    report.push(ReportEntry::new(
                        &format!("Cleaning {}", grimy.item.name),
                        gross,
                        cost,
                        limit,
                        herb.volume,
                    ));
                }
            }

            // Unfinished potion
            {
                let limit = unf.item.limit.unwrap();
                let cost = herb.low() * limit;
                let gross = unf.high() * limit;
                report.push(ReportEntry::new(
                    unf.item.name.as_str(),
                    gross,
                    cost,
                    limit,
                    herb.volume,
                ));
            }
        }

        report
    }
}
