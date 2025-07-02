use anyhow::Result;
use seda_sdk_rs::{elog, http_fetch, log, Process};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct StockScreenerItem {
    symbol: String,
    companyName: String,
    price: f64,
}

#[derive(Serialize, Debug)]
struct TrimmedData {
    symbol: String,
    name: String,
    price: f64,
    changesPercentage: f64, // always 0.0 for now
}

pub fn execution_phase() -> Result<()> {
    let url = "https://financialmodelingprep.com/api/v3/stock-screener?priceLowerThan=5&volumeMoreThan=100000&limit=10&apikey=ssCANdhuM3VJqXfNL39Ne0nvDqkIhDtk";

    let res = http_fetch(url, None);
    if !res.is_ok() {
        elog!("❌ Pennywise API request failed: {}", res.status);
        Process::error("Pennywise API request failed".as_bytes());
        return Ok(());
    }

    let parsed: Vec<StockScreenerItem> = match serde_json::from_slice(&res.bytes) {
        Ok(data) => data,
        Err(e) => {
            elog!("❌ Pennywise Failed to parse API response: {}", e);
            Process::error("Pennywise Failed to parse API response".as_bytes());
            return Ok(());
        }
    };

    let trimmed: Vec<TrimmedData> = parsed
        .into_iter()
        .map(|stock| TrimmedData {
            symbol: stock.symbol,
            name: stock.companyName,
            price: stock.price,
            changesPercentage: 0.0,
        })
        .collect();

    if trimmed.is_empty() {
        Process::error("No penny stocks found".as_bytes());
        return Ok(());
    }

    log!("📦 Pennywise Top 10 Penny Stocks:");
    for stock in &trimmed {
        log!(
            "{} ({}): ${:.2} | {:.2}% 📈",
            stock.name,
            stock.symbol,
            stock.price,
            stock.changesPercentage
        );
    }
 log!("🌐 view on pennywise247.vercel.app");

 use std::fs;

let json = serde_json::to_string_pretty(&trimmed)?;
fs::write("/mnt/data/cron-output.txt", &json)?;
Process::success(json.as_bytes());


    let json = serde_json::to_vec(&trimmed)?;
    Process::success(&json); // ✅ sends only the trimmed list to SEDA

    Ok(())
}
