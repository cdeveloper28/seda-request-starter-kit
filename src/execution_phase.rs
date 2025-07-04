use anyhow::Result;
use seda_sdk_rs::{elog, http_fetch, log, Process};
use serde::Deserialize;

#[derive(Deserialize)]
struct QuoteApiResponse {
    q: String, // quote
    a: String, // author
}

pub fn execution_phase() -> Result<()> {
    let url = "https://zenquotes.io/api/random";

    let response = http_fetch(url, None);
    if !response.is_ok() {
        elog!("❌ Failed to fetch quote: {}", response.status);
        Process::error("Quote API request failed".as_bytes());
        return Ok(());
    }

    let data: Vec<QuoteApiResponse> = match serde_json::from_slice(&response.bytes) {
        Ok(parsed) => parsed,
        Err(e) => {
            elog!("❌ Failed to parse API response: {}", e);
            Process::error("Invalid quote API response".as_bytes());
            return Ok(());
        }
    };

    if data.is_empty() {
        Process::error("Empty quote API response".as_bytes());
        return Ok(());
    }

    let quote = format!("{} — {}", data[0].q, data[0].a);
    log!("📜 Quote Oracle: {}", quote);

    Process::success(quote.as_bytes()); // Send on-chain
    Ok(())
}
