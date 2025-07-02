import { readFileSync, existsSync } from 'fs';

Bun.serve({
  port: process.env.PORT || 3000,
  fetch(req) {
    const url = new URL(req.url);

    if (url.pathname === '/latest.json') {
      const filePath = '/mnt/data/cron-output.txt';

      if (!existsSync(filePath)) {
        return new Response(
          JSON.stringify({ error: 'No data available' }),
          { status: 404, headers: { 'Content-Type': 'application/json' } }
        );
      }

      try {
        const file = readFileSync(filePath, 'utf8');
        const json = JSON.parse(file);

        return new Response(
          JSON.stringify(json),
          { status: 200, headers: { 'Content-Type': 'application/json' } }
        );
      } catch (err) {
        console.error('❌ Failed to parse cron-output.txt:', err);
        return new Response(
          JSON.stringify({ error: 'Invalid JSON format in cron-output.txt' }),
          { status: 500, headers: { 'Content-Type': 'application/json' } }
        );
      }
    }

    return new Response('Not found', { status: 404 });
  }
});
