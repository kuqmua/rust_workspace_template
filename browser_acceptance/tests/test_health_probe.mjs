import assert from "node:assert/strict";
import test from "node:test";
import { fetchHealthText } from "../../frontend_admin/static/health_probe.js";

function healthProbeReaderFixture(chunks, failure) {
  const calls = { read: 0, cancel: 0, release: 0 };
  const reader = {
    async read() {
      calls.read += 1;
      if (failure) throw failure;
      const value = chunks[calls.read - 1];
      return value ? { done: false, value } : { done: true };
    },
    async cancel() { calls.cancel += 1; },
    releaseLock() { calls.release += 1; }
  };
  return { calls, reader };
}

function mockHealthResponse(context, reader, status = 200) {
  return context.mock.method(globalThis, "fetch", async () => ({
    status,
    body: reader ? { getReader: () => reader } : null
  }));
}

test("test_health_probe_preserves_status_and_no_store_for_absent_body", async context => {
  const fetch = mockHealthResponse(context, null, 503);
  assert.equal(await fetchHealthText("/health/ready/read"), "503 ");
  assert.deepEqual(fetch.mock.calls[0].arguments, ["/health/ready/read", { cache: "no-store" }]);
});

test("test_health_probe_decodes_split_utf8_and_releases_reader", async context => {
  const fixture = healthProbeReaderFixture([Uint8Array.of(0xc3), Uint8Array.of(0xa9)]);
  mockHealthResponse(context, fixture.reader);
  assert.equal(await fetchHealthText("/health/read"), "200 \u00e9");
  assert.deepEqual(fixture.calls, { read: 3, cancel: 1, release: 1 });
});

test("test_health_probe_accepts_exact_byte_limit", async context => {
  const fixture = healthProbeReaderFixture([new Uint8Array(8192).fill(120)]);
  mockHealthResponse(context, fixture.reader);
  assert.equal(await fetchHealthText("/health/read"), `200 ${"x".repeat(8192)}`);
  assert.deepEqual(fixture.calls, { read: 2, cancel: 1, release: 1 });
});

test("test_health_probe_rejects_accumulated_byte_overflow_and_cleans_up", async context => {
  const fixture = healthProbeReaderFixture([new Uint8Array(8192).fill(120), Uint8Array.of(120)]);
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), { message: "health_response_too_large" });
  assert.deepEqual(fixture.calls, { read: 2, cancel: 1, release: 1 });
});

test("test_health_probe_rejects_incomplete_utf8_and_cleans_up", async context => {
  const fixture = healthProbeReaderFixture([Uint8Array.of(0xc3)]);
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), TypeError);
  assert.deepEqual(fixture.calls, { read: 2, cancel: 1, release: 1 });
});

test("test_health_probe_preserves_read_failure_and_cleans_up", async context => {
  const failure = new Error("read_failed");
  const fixture = healthProbeReaderFixture([], failure);
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), error => error === failure);
  assert.deepEqual(fixture.calls, { read: 1, cancel: 1, release: 1 });
});

test("test_health_probe_releases_lock_after_cancellation_failure", async context => {
  const failure = new Error("cancel_failed");
  const fixture = healthProbeReaderFixture([]);
  context.mock.method(fixture.reader, "cancel", async () => {
    fixture.calls.cancel += 1;
    throw failure;
  });
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), error => error === failure);
  assert.deepEqual(fixture.calls, { read: 1, cancel: 1, release: 1 });
});

test("test_health_probe_preserves_fetch_failure", async context => {
  const failure = new Error("fetch_failed");
  context.mock.method(globalThis, "fetch", async () => { throw failure; });
  await assert.rejects(fetchHealthText("/health/read"), error => error === failure);
});

test("test_health_probe_cleanup_failures_preserve_error_precedence", async context => {
  const readFailure = new Error();
  const cancelFailure = new Error();
  const releaseFailure = new Error();
  await [
    [readFailure, cancelFailure, null],
    [readFailure, null, releaseFailure],
    [readFailure, cancelFailure, releaseFailure],
    [null, cancelFailure, releaseFailure],
    [null, null, releaseFailure]
  ].reduce(async (previous, [read, cancel, release], index) => {
    await previous;
    await context.test(`test_cleanup_failure_case_${index}`, async context => {
      const fixture = healthProbeReaderFixture([], read);
      context.mock.method(fixture.reader, "cancel", async () => {
        fixture.calls.cancel += 1;
        if (cancel) throw cancel;
      });
      context.mock.method(fixture.reader, "releaseLock", () => {
        fixture.calls.release += 1;
        if (release) throw release;
      });
      mockHealthResponse(context, fixture.reader);
      await assert.rejects(fetchHealthText("/health/read"), error => error === (release ?? cancel ?? read));
      assert.deepEqual(fixture.calls, { read: 1, cancel: 1, release: 1 });
    });
  }, Promise.resolve());
});

test("test_health_probe_rejects_malformed_utf8_during_read_and_cleans_up", async context => {
  const fixture = healthProbeReaderFixture([Uint8Array.of(0xc3, 0x28), Uint8Array.of(0x61)]);
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), TypeError);
  assert.deepEqual(fixture.calls, { read: 1, cancel: 1, release: 1 });
});

test("test_health_probe_counts_multibyte_text_and_ignores_empty_chunks_at_exact_limit", async context => {
  const text = "\u00e9".repeat(4096);
  const bytes = new TextEncoder().encode(text);
  assert.equal(bytes.byteLength, 8192);
  const fixture = healthProbeReaderFixture([
    new Uint8Array(),
    bytes.subarray(0, 1),
    bytes.subarray(1),
    new Uint8Array()
  ]);
  mockHealthResponse(context, fixture.reader);
  assert.equal(await fetchHealthText("/health/read"), `200 ${text}`);
  assert.deepEqual(fixture.calls, { read: 5, cancel: 1, release: 1 });
});

test("test_health_probe_rejects_multibyte_overflow_before_reading_more_chunks", async context => {
  const encoder = new TextEncoder();
  const fixture = healthProbeReaderFixture([
    encoder.encode("\u00e9".repeat(4096)),
    encoder.encode("\u00e9"),
    Uint8Array.of(0x61)
  ]);
  mockHealthResponse(context, fixture.reader);
  await assert.rejects(fetchHealthText("/health/read"), { message: "health_response_too_large" });
  assert.deepEqual(fixture.calls, { read: 2, cancel: 1, release: 1 });
});
