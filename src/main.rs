use std::{collections::HashMap, path::Path, time::Duration};

use eyre::Result;
use item::{Item, ItemPrice, ItemPriceResponse, ItemWithPrice};
use serde::de::DeserializeOwned;

mod item;

const ITEM_MAP_CACHE_FILEPATH: &str = "item_map.json";
const PRICES_CACHE_FILEPATH: &str = "prices.json";

type ItemMap = HashMap<usize, Item>;
type ItemPriceMap = HashMap<usize, ItemPrice>;

fn get<T: DeserializeOwned>(url: &str) -> eyre::Result<T> {
    let agent = ureq::AgentBuilder::new()
        .user_agent("osrs-mm/v0.0.1")
        .build();

    let response = agent.get(url).call()?;
    let body = response.into_string()?;
    let json = serde_json::from_str(&body)?;
    Ok(json)
}

fn item_map_from_url() -> eyre::Result<ItemMap> {
    Ok(
        get::<Vec<Item>>("https://prices.runescape.wiki/api/v1/osrs/mapping")?
            .into_iter()
            .map(|i| (i.id, i))
            .collect(),
    )
}

fn item_from_cache() -> Option<ItemMap> {
    let path = Path::new(ITEM_MAP_CACHE_FILEPATH);
    if !path.exists() {
        return None;
    }

    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn write_to_cache(items: &ItemMap) -> Result<()> {
    let contents = serde_json::to_string_pretty(&items)?;
    std::fs::write(ITEM_MAP_CACHE_FILEPATH, contents)?;
    Ok(())
}

fn load_item_map() -> Result<ItemMap> {
    Ok(match item_from_cache() {
        Some(items) => items,
        None => {
            let items = item_map_from_url()?;
            write_to_cache(&items)?;
            items
        }
    })
}

fn prices_from_url() -> Result<ItemPriceMap> {
    let response: ItemPriceResponse = get("https://prices.runescape.wiki/api/v1/osrs/latest")?;
    Ok(response.data)
}

fn prices_from_cache() -> Option<ItemPriceMap> {
    let path = Path::new(PRICES_CACHE_FILEPATH);
    if !path.exists() {
        return None;
    }

    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn write_prices_to_cache(prices: &ItemPriceMap) -> Result<()> {
    let contents = serde_json::to_string_pretty(&prices)?;
    std::fs::write(PRICES_CACHE_FILEPATH, contents)?;
    Ok(())
}

fn load_price_map() -> Result<ItemPriceMap> {
    Ok(match prices_from_cache() {
        Some(prices) => prices,
        None => {
            let prices = prices_from_url()?;
            write_prices_to_cache(&prices)?;
            prices
        }
    })
}

const ID_CLAY: usize = 434;
const ID_SOFT_CLAY: usize = 434;
const ID_ASTRAL_RUNE: usize = 9075;
const ID_DRAGON_BOLT_UNF: usize = 21930;
const ID_DIAMOND_DRAGON_BOLT_E: usize = 21946;
const ID_LAW_RUNE: usize = 563;
const ID_BOOD_RUNE: usize = 565;
const ID_COSMIC_RUNE: usize = 564;
const ID_DIAMOND_BOLT_TIPS: usize = 9192;
const ID_RUBY_BOLT_TIPS: usize = 9191;
const ID_RUBY_DRAGON_BOLT_E: usize = 21944;

struct Cache {
    pub items: ItemMap,
    pub prices: ItemPriceMap,
}

impl Cache {
    pub fn new() -> Result<Self> {
        Ok(Self {
            items: load_item_map()?,
            prices: load_price_map()?,
        })
    }

    fn get(&self, id: usize) -> ItemWithPrice {
        let item = self
            .items
            .get(&id)
            .expect("I hardcode all ids and know them in advance");

        let price = self.prices.get(&id);
        ItemWithPrice { item, price }
    }
}

fn pretty_print_int(i: isize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}

fn print_result(name: &str, gross: usize, cost: usize) {
    let profit = gross as isize - cost as isize;
    let pp = pretty_print_int;
    println!(
        "{:>30}: {:>12}, [g: {:>12}, c: {:>12}]",
        name,
        pp(profit),
        pp(gross as isize),
        pp(cost as isize)
    );
}

struct ReportEntry {
    name: String,
    gross: usize,
    cost: usize,
    limit: usize,
    time: Option<Duration>,
}

fn main() -> eyre::Result<()> {
    let cache = Cache::new()?;

    // Clay mm
    let clay = cache.get(ID_CLAY);
    let soft_clay = cache.get(ID_SOFT_CLAY);
    let astral = cache.get(ID_ASTRAL_RUNE);

    let limit = clay.item.limit.unwrap();
    let casts = limit / 27 + 1;
    let cast_cost = casts * astral.avg();
    let cost = limit * clay.low() + cast_cost;
    let gross = soft_clay.high() * limit;
    print_result("clay", gross, cost);

    let dbunf = cache.get(ID_DRAGON_BOLT_UNF);
    let dtips = cache.get(ID_DIAMOND_BOLT_TIPS);
    let ddbe = cache.get(ID_DIAMOND_DRAGON_BOLT_E);
    let cosmic = cache.get(ID_COSMIC_RUNE);
    let law = cache.get(ID_LAW_RUNE);

    let limit = dtips.item.limit.unwrap();
    let cast_cost = ((law.avg() * 2) + cosmic.avg()) * (limit / 10);
    let mat_cost = (dbunf.low() + dtips.high()) * limit;
    let cost = mat_cost + cast_cost;
    let gross = ddbe.high() * limit;
    print_result(&ddbe.item.name, gross, cost);

    let rdbe = cache.get(ID_RUBY_DRAGON_BOLT_E);
    let rtips = cache.get(ID_RUBY_BOLT_TIPS);
    let blood = cache.get(ID_BOOD_RUNE);
    let cast_cost = (blood.avg() + cosmic.avg()) * (limit / 10);
    let mat_cost = (dbunf.low() + rtips.high()) * limit;
    let cost = mat_cost + cast_cost;
    let gross = rdbe.high() * limit;
    print_result(&rdbe.item.name, gross, cost);

    Ok(())
}
