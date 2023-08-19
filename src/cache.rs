use std::{collections::HashMap, path::Path};

use eyre::Result;
use serde::de::DeserializeOwned;

use crate::item::{DailyVolumeResponse, Item, ItemPrice, ItemPriceResponse, ItemWithPrice};

const ITEM_MAP_CACHE_FILEPATH: &str = "cache/item_map.json";
const PRICES_CACHE_FILEPATH: &str = "cache/prices.json";
const VOLUMES_CACHE_FILEPATH: &str = "cache/volumes.json";

type ItemMap = HashMap<usize, Item>;
type ItemPriceMap = HashMap<usize, ItemPrice>;
type VolumeMap = HashMap<usize, usize>;

pub struct Cache {
    pub items: ItemMap,
    pub prices: ItemPriceMap,
    pub volumes: VolumeMap,
}

impl Cache {
    pub fn new(force: bool) -> Result<Self> {
        Ok(Self {
            items: load_item_map()?,
            prices: load_price_map(force)?,
            volumes: load_volume_map(force)?,
        })
    }

    pub fn get(&self, id: usize) -> ItemWithPrice {
        let item = self
            .items
            .get(&id)
            .expect("I hardcode all ids and know them in advance");

        let price = self.prices.get(&id);
        let volume = self.volumes.get(&id).map(|f| *f).unwrap_or_default();
        ItemWithPrice {
            item,
            price,
            volume,
        }
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

fn load_price_map(force: bool) -> Result<ItemPriceMap> {
    if force {
        let prices = prices_from_url()?;
        write_prices_to_cache(&prices)?;
        Ok(prices)
    } else {
        Ok(match prices_from_cache() {
            Some(prices) => prices,
            None => {
                let prices = prices_from_url()?;
                write_prices_to_cache(&prices)?;
                prices
            }
        })
    }
}

fn volumes_from_url() -> Result<VolumeMap> {
    let response: DailyVolumeResponse = get("https://prices.runescape.wiki/api/v1/osrs/volumes")?;
    Ok(response.data)
}

fn volumes_from_cache() -> Option<VolumeMap> {
    let path = Path::new(VOLUMES_CACHE_FILEPATH);
    if !path.exists() {
        return None;
    }

    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn write_volumes_to_cache(volumes: &VolumeMap) -> Result<()> {
    let contents = serde_json::to_string_pretty(&volumes)?;
    let path = Path::new(VOLUMES_CACHE_FILEPATH);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, contents)?;
    Ok(())
}

fn load_volume_map(force: bool) -> Result<VolumeMap> {
    if force {
        let volumes = volumes_from_url()?;
        write_volumes_to_cache(&volumes)?;
        Ok(volumes)
    } else {
        Ok(match volumes_from_cache() {
            Some(volumes) => volumes,
            None => {
                let volumes = volumes_from_url()?;
                write_volumes_to_cache(&volumes)?;
                volumes
            }
        })
    }
}
