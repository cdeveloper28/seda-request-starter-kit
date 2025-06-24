import { postAndAwaitDataRequest, buildSigningConfig, Signer } from "@seda-protocol/dev-tools";

async function fetchBTCPrice(): Promise<number> {
  const res = await fetch("https://api.coindesk.com/v1/bpi/currentprice/BTC.json");
  const data = await res.json();
  return parseFloat(data.bpi.USD.rate.replace(",", ""));
}

async function main() {
  const signer = await Signer.fromPartial(buildSigningConfig({}));

  const btcPrice = await fetchBTCPrice();

  const result = await postAndAwaitDataRequest(signer, {
    oracleProgramId: process.env.ORACLE_PROGRAM_ID!,
    drInputs: Buffer.from(`btc-usd:${btcPrice}`),
    tallyInputs: Buffer.alloc(0),
    consensusOptions: { method: "none" },
    memo: Buffer.from(new Date().toISOString()),
  }, {});

  console.log("✅ DR sent:", result.dataRequestId);
}

main().catch(console.error);
