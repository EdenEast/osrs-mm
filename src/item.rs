use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// An in-game item. This doesn't include price data, just static data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub examine: String,
    pub members: bool,
    #[serde(rename = "lowalch")]
    pub low_alch: Option<usize>,
    #[serde(rename = "highalch")]
    pub high_alch: Option<usize>,
    pub limit: Option<usize>,
    pub value: usize,
}

/// Current price data for an in-game item
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPrice {
    pub high: Option<usize>,
    pub high_time: Option<usize>,
    pub low: Option<usize>,
    pub low_time: Option<usize>,
}

/// Response for the `/latest` endpoint of the price API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemPriceResponse {
    pub data: HashMap<usize, ItemPrice>,
}

/// An item's core data paired with its current price data. Price data will be
/// `None` if it hasn't been traded recently.
#[derive(Clone, Debug)]
pub struct ItemWithPrice<'a> {
    pub item: &'a Item,
    pub price: Option<&'a ItemPrice>,
}

impl<'a> ItemWithPrice<'a> {
    /// Get the average of the recent high and low prices.
    pub fn avg(&self) -> usize {
        let price = self.price.unwrap();
        match (price.high, price.low) {
            (Some(high), Some(low)) => (high + low) / 2,
            (Some(value), None) | (None, Some(value)) => value,
            (None, None) => unreachable!(),
        }
    }

    pub fn high(&self) -> usize {
        self.price.unwrap().high.unwrap()
    }

    pub fn low(&self) -> usize {
        self.price.unwrap().low.unwrap()
    }
}
