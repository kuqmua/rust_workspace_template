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
    await assert.rejects(fetchHealthText("/health/read"), /health_response_too_large/);
    assert.equal(cancelled, true);

    const failure = new Error("network_failure");
    globalThis.fetch = async () => { throw failure; };
    await assert.rejects(fetchHealthText("/health/read"), error => error === failure);

    globalThis.fetch = async () => new Response(
      '{"status":"unavailable"}', { status: 503 },
    );
    assert.equal(await fetchHealthText("/health/read"), '503 {"status":"unavailable"}');
  } finally {
    globalThis.fetch = originalFetch;
  }
});

test("test_health_probe_accepts_exact_byte_limit_and_split_utf8", async () => {
  const originalFetch = globalThis.fetch;
  const calls = [];
  try {
    const chunks = [
      new Uint8Array([...new Uint8Array(8190).fill(120), 0xc3]),
      new Uint8Array([0xa9]),
    ];
    let index = 0;
    const lifecycle = [];
    globalThis.fetch = async (url, options) => {
      calls.push([url, options]);
      return {
        status: 503,
        body: {
          getReader: () => ({
            read: async () => index < chunks.length
              ? { done: false, value: chunks[index++] }
              : { done: true },
            cancel: async () => { lifecycle.push("cancel"); },
            releaseLock: () => { lifecycle.push("release"); },
          }),
        },
      };
    };
    assert.equal(
      await fetchHealthText("/health/read"),
      `503 ${"x".repeat(8190)}${String.fromCodePoint(0xe9)}`,
    );
    assert.deepEqual(lifecycle, ["cancel", "release"]);
    assert.deepEqual(calls, [["/health/read", { cache: "no-store" }]]);
    globalThis.fetch = async () => ({ status: 204, body: null });
    assert.equal(await fetchHealthText("/health/read"), "204 ");
  } finally {
    globalThis.fetch = originalFetch;
  }
});

test("test_health_probe_releases_reader_after_utf8_and_read_failures", async () => {
  const originalFetch = globalThis.fetch;
  try {
    const readFailure = new Error("read_failure");
    await [
      { chunk: new Uint8Array([0xff]), failure: TypeError },
      { chunk: new Uint8Array([0xc3]), failure: TypeError },
      { readFailure, failure: error => error === readFailure },
    ].reduce(async (previous, scenario) => {
      await previous;
      let supplied = false;
      const lifecycle = [];
      globalThis.fetch = async () => ({
        status: 200,
        body: {
          getReader: () => ({
            read: async () => {
              if (scenario.readFailure) throw scenario.readFailure;
              if (supplied) return { done: true };
              supplied = true;
              return { done: false, value: scenario.chunk };
            },
            cancel: async () => { lifecycle.push("cancel"); },
            releaseLock: () => { lifecycle.push("release"); },
          }),
        },
      });
      await assert.rejects(fetchHealthText("/health/read"), scenario.failure);
      assert.deepEqual(lifecycle, ["cancel", "release"]);
    }, Promise.resolve());
  } finally {
    globalThis.fetch = originalFetch;
  }
});

test("test_health_probe_releases_reader_when_cancellation_fails", async () => {
  const originalFetch = globalThis.fetch;
  const failure = new Error("cancellation_failure");
  const lifecycle = [];
  try {
    globalThis.fetch = async () => ({
      status: 200,
      body: {
        getReader: () => ({
          read: async () => ({ done: true }),
          cancel: async () => { lifecycle.push("cancel"); throw failure; },
          releaseLock: () => { lifecycle.push("release"); },
        }),
      },
    });
    await assert.rejects(fetchHealthText("/health/read"), error => error === failure);
    assert.deepEqual(lifecycle, ["cancel", "release"]);
  } finally {
    globalThis.fetch = originalFetch;
  }
});
