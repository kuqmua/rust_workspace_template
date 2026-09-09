import fs from "node:fs";
import test from "node:test";
import assert from "node:assert/strict";

const source = fs.readFileSync(
  new URL("../frontend_admin/static/health_probe.js", import.meta.url),
  "utf8",
);
const { fetchHealthText } = await import(
  "data:text/javascript;base64," + Buffer.from(source).toString("base64")
);

test("test_health_probe_bounds_reads_and_preserves_failures", async () => {
  const originalFetch = globalThis.fetch;
  try {
    let cancelled = false;
    globalThis.fetch = async () => new Response(new ReadableStream({
      pull(controller) {
        controller.enqueue(new Uint8Array(4096));
      },
      cancel() {
        cancelled = true;
      },
    }));
    await assert.rejects(fetchHealthText("/health"), /health_response_too_large/);
    assert.equal(cancelled, true);

    const failure = new Error("network_failure");
    globalThis.fetch = async () => { throw failure; };
    await assert.rejects(fetchHealthText("/health"), error => error === failure);

    globalThis.fetch = async () => new Response(
      '{"status":"unavailable"}', { status: 503 },
    );
    assert.equal(await fetchHealthText("/health"), '503 {"status":"unavailable"}');
  } finally {
    globalThis.fetch = originalFetch;
  }
});
