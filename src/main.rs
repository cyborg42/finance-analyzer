use std::{
    collections::BTreeMap,
    time::{Duration, UNIX_EPOCH},
};
use ta::DataItem;
use time::{
    macros::{datetime, offset, time},
    OffsetDateTime,
};
use tokio_test;
use finance_analyzer::{
    position::{Portfolio, Position, Symbols},
    quote::MultiChart,
    utils::{sec_time, ToFile},
};
use yahoo_finance_api as yahoo;

fn main() {
    let multi_chart = MultiChart::load("data/chart.json").unwrap();
    let start_time = datetime!(2022-08-20 00:00:00)
        .assume_offset(offset!(+0))
        .unix_timestamp() as u64;
    let data: BTreeMap<_, _> = multi_chart
        .data
        .into_iter()
        .filter(|(time, _)| *time > start_time)
        .collect();
    let symbols = Symbols::load("config/symbols.toml").unwrap();
    let mut portfolio = Portfolio::new(symbols, 10000.);
    let mut portfolio_keep = portfolio.clone();
    let balance_interval = Duration::from_secs(3600 * 24);
    let mut last_time = OffsetDateTime::from(UNIX_EPOCH);
    let mut set = false;
    for (time, quotes) in data.iter() {
        for (symbol, quote) in quotes.iter() {
            portfolio_keep
                .positions
                .entry(symbol.clone())
                .and_modify(|position| {
                    position.last_price = quote.close;
                });
        }
        if portfolio_keep.positions.values().any(|p| p.last_price <= 0.) {
            continue;
        }
        if set {
            continue;
        }
        for p in portfolio_keep.positions.values_mut() {
            let invest = portfolio_keep.cash * p.weight;
            p.shares = invest / p.last_price;
        }
        portfolio_keep.cash = 0.;
        set = true;
    }

    let mut max_value: f64 = 0.;
    let mut max_drawdown: f64 = 0.;

    for (time, quotes) in data.iter() {
        let time = sec_time(*time);
        for (symbol, quote) in quotes.iter() {
            portfolio
                .positions
                .entry(symbol.clone())
                .and_modify(|position| {
                    position.last_price = quote.close;
                });
        }

        if time - last_time < balance_interval {
            continue;
        }
        last_time = time;

        let total_value = portfolio.cash
            + portfolio
                .positions
                .iter()
                .map(|(_, position)| position.shares * position.last_price)
                .sum::<f64>();
        max_value = max_value.max(total_value);
        max_drawdown = max_drawdown.max((max_value - total_value) / max_value);
        let mut sum_invset = 0.;
        for p in portfolio.positions.values_mut() {
            if p.last_price <= 0. {
                continue;
            }
            let invest = total_value * p.weight;
            p.shares = invest / p.last_price;
            sum_invset += invest;
        }
        portfolio.cash = total_value - sum_invset;
        println!("{}: \t{}\n{:#?}", time, total_value, portfolio);
    }
    println!("Max drawdown: {}", max_drawdown);

    let keep_volume = portfolio_keep
        .positions
        .values()
        .map(|p| p.shares * p.last_price)
        .sum::<f64>();
    println!("Keep volume: {}, {:#?}", keep_volume, portfolio_keep);
}

#[test]
fn time() {
    let timestamp = 1661434200;
    let time: OffsetDateTime = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(timestamp));
    println!("{}", time);
}
