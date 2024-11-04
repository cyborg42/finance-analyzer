use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use yahoo_finance_api::Quote;

use ta::{errors::TaError, DataItem};

use crate::utils::ToFile;

pub trait ToDataItem {
    fn to_data_item(&self) -> DataItem {
        self.to_data_item_checked().unwrap()
    }
    fn to_data_item_checked(&self) -> Result<DataItem, TaError>;
}

impl ToDataItem for Quote {
    fn to_data_item_checked(&self) -> Result<DataItem, TaError> {
        DataItem::builder()
            .open(self.open)
            .high(self.high)
            .low(self.low)
            .close(self.close)
            .volume(self.volume as f64)
            .build()
    }
}

type TimestampSec = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiChart {
    pub interval: String,
    pub data: BTreeMap<TimestampSec, BTreeMap<String, Quote>>,
}

impl MultiChart {
    pub fn new(interval: String) -> Self {
        Self {
            interval,
            data: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, symbol: String, quotes: Vec<Quote>) {
        for q in quotes {
            let time = q.timestamp;
            let entry = self.data.entry(time).or_insert_with(BTreeMap::new);
            entry.insert(symbol.clone(), q);
        }
    }
}

impl ToFile for MultiChart {}

#[test]
fn test_multi_chart() {
    let mut multi_chart = MultiChart::new("1d".to_string());
    let quotes = vec![
        Quote {
            timestamp: 1,
            open: 1.,
            high: 2.,
            low: 0.,
            close: 1.,
            volume: 100,
            adjclose: 1.,
        },
        Quote {
            timestamp: 2,
            open: 2.,
            high: 3.,
            low: 1.,
            close: 2.,
            volume: 200,
            adjclose: 2.,
        },
    ];
    multi_chart.insert("AAPL".to_string(), quotes);
    let path = "data_test.json";
    multi_chart.save(path).unwrap();
    let loaded = MultiChart::load(path).unwrap();
    assert_eq!(multi_chart, loaded);
}
