import { serve } from "bun";
import "dotenv/config";

// 🔁 Auto-trigger DR when Railway Cron hits us
if (process.env.CRON === "true") {
  console.log("⏰ Railway Cron Trigger Detected — Posting Oracle Request...");
  await import("./scripts/post-dr.js");
  process.exit(0);
}

// 🛰️ Serve API for /latest
serve({
  port: 3000,
  async fetch(req) {
    if (req.url.endsWith("/latest")) {
      const ORACLE_PROGRAM_ID = process.env.ORACLE_PROGRAM_ID!;
      const res = await fetch(`https://testnet.api.seda.xyz/oracle-programs/${ORACLE_PROGRAM_ID}/latest-result`);
      const data = await res.json();
      const decoded = data.result?.decoded;

      if (!decoded) {
        return new Response(JSON.stringify({ error: "No data yet" }), { status: 404 });
      }

      return new Response(JSON.stringify(decoded, null, 2), {
        headers: { "Content-Type": "application/json" },
      });
    }

    return new Response("✅ Penny Stock Oracle is running.");
  },
});
