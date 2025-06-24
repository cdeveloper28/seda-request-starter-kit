use anyhow::Result;
use seda_sdk_rs::{elog, http_fetch, log, Process};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ApiResponseItem {
    symbol: String,
    name: String,
    price: f64,
    changesPercentage: f64,
}

#[derive(Deserialize)]
struct ApiResponse(Vec<ApiResponseItem>);

#[derive(Serialize, Debug)]
struct TrimmedData {
    symbol: String,
    name: String,
    price: f64,
    changesPercentage: f64,
}

pub fn execution_phase() -> Result<()> {
    let url = "https://financialmodelingprep.com/api/v3/quote/SNDL?apikey=ssCANdhuM3VJqXfNL39Ne0nvDqkIhDtk";

    let response = http_fetch(url, None);

    if !response.is_ok() {
        elog!("API request failed: {}", response.status);
        Process::error("API request failed".as_bytes());
        return Ok(());
    }

    let parsed: Vec<ApiResponseItem> = serde_json::from_slice(&response.bytes)?;
    if parsed.is_empty() {
        Process::error("No data returned".as_bytes());
        return Ok(());
    }

    let selected = &parsed[0];
    let trimmed = TrimmedData {
        symbol: selected.symbol.clone(),
        name: selected.name.clone(),
        price: selected.price,
        changesPercentage: selected.changesPercentage,
    };

    let output = serde_json::to_vec(&trimmed)?;
    log!("Fetched stock data: {:?}", trimmed);
    Process::success(&output);

    Ok(())
}
