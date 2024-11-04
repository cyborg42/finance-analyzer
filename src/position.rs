use std::{
    any,
    collections::{BTreeMap, HashMap},
};

use serde::{Deserialize, Serialize};

use crate::utils::ToFile;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symbols(pub BTreeMap<String, f64>);

impl ToFile for Symbols {
    fn save(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let ret: Self = toml::from_str(&data)?;
        Ok(ret)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub shares: f64,
    pub weight: f64,
    pub last_price: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: HashMap<String, Position>,
}

impl Portfolio {
    pub fn new(symbols: Symbols, cash: f64) -> Self {
        let sum_weight = symbols.0.values().sum::<f64>();
        let positions = symbols
            .0
            .iter()
            .map(|(symbol, weight)| {
                let shares = 0.;
                let last_price = 0.;
                (
                    symbol.clone(),
                    Position {
                        shares,
                        weight: *weight / sum_weight,
                        last_price,
                    },
                )
            })
            .collect();
        Self { cash, positions }
    }
}
