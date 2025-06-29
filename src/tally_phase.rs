use anyhow::Result;
use seda_sdk_rs::{elog, get_reveals, log, Process};

pub fn tally_phase() -> Result<()> {
    let reveals = get_reveals()?;
    if reveals.is_empty() {
        Process::error("No reveals found".as_bytes());
        return Ok(());
    }

    // We could do validation here, but for now return the first successful reveal
    let body = &reveals[0].body.reveal;
    log!("Top 10 penny stocks by pennywise247.vercel.app");
    Process::success(body);

    Ok(())
}
