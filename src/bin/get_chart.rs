use finance_analyzer::{position::Symbols, quote::MultiChart, utils::ToFile};
use yahoo_finance_api as yahoo;

const SYMBOLS: &str = "config/symbols.toml";
const CHART: &str = "data/chart.json";
const INTERVAL: &str = "1d";
const RANGE: &str = "5y";

async fn get_chart() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let mut multi_chart = MultiChart::new(INTERVAL.to_string());
    let symbols = Symbols::load(SYMBOLS).unwrap();
    for (symbol, _) in symbols.0 {
        println!("Getting chart for {}", symbol);
        let response = provider
            .get_quote_range(&symbol, INTERVAL, RANGE)
            .await
            .unwrap();
        let quotes = response.quotes().unwrap();
        multi_chart.insert(symbol.to_string(), quotes);
    }
    multi_chart.save(CHART).unwrap();
}

fn main() {
    tokio_test::block_on(get_chart());
}
