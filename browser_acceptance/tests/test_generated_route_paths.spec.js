import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

async function readFromTablePage(page, resource) {
  const path = `/${resource}/read`;
  const request = page.waitForRequest(value => new URL(value.url()).pathname === path && value.method() === "POST");
  await page.goto(`/admin/${resource}`);
  const payload = (await request).postDataJSON();
  const response = await page.request.post(path, { data: payload });
  expect(response.status(), path).toBe(200);
  expect(await response.json(), path).toBeTruthy();
  return path;
}

test("test_generated_read_replaces_single_record_reads", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    await ["users", "roles", "rules", "system_settings"].reduce(async (previous, resource) => {
      await previous;
      await readFromTablePage(page, resource);
      await ["read_one", "read_one_payload_example", "read_many", "read_many_payload_example"].reduce(async (pending, operation) => {
        await pending;
        const removed = await page.request.fetch(`/${resource}/${operation}`, {
          method: operation.endsWith("payload_example") ? "GET" : "POST",
          maxRedirects: 0,
        });
        expect(removed.status()).not.toBe(200);
      }, Promise.resolve());
      const legacy = await page.request.get(`/admin_${resource}/read_payload_example`, { maxRedirects: 0 });
      expect(legacy.status()).not.toBe(200);
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});

for (const resource of ["access_sessions", "role_rules"]) {
  test(`test_${resource}_read_uses_post_only`, async ({ page }) => {
    await signInInitialAdministrator(page);
    try {
      const path = await readFromTablePage(page, resource);
      expect((await page.request.get(path, { maxRedirects: 0 })).status()).not.toBe(200);
    } finally {
      await signOutIfAuthenticated(page);
    }
  });
}
