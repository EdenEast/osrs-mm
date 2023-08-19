use std::{collections::HashMap, path::Path};

use eyre::Result;
use serde::de::DeserializeOwned;

use crate::item::{Item, ItemPrice, ItemPriceResponse, ItemWithPrice};

const ITEM_MAP_CACHE_FILEPATH: &str = "cache/item_map.json";
const PRICES_CACHE_FILEPATH: &str = "cache/prices.json";

type ItemMap = HashMap<usize, Item>;
type ItemPriceMap = HashMap<usize, ItemPrice>;

pub struct Cache {
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

    pub fn get(&self, id: usize) -> ItemWithPrice {
        let item = self
            .items
            .get(&id)
            .expect("I hardcode all ids and know them in advance");

        let price = self.prices.get(&id);
        ItemWithPrice { item, price }
    }
}

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
    let path = Path::new(ITEM_MAP_CACHE_FILEPATH);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, contents)?;
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
    let path = Path::new(PRICES_CACHE_FILEPATH);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, contents)?;
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
