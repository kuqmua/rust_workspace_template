import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test("test_generated_read_replaces_single_record_reads", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    const document = await (await page.request.get("/openapi.json")).json();
    const resources = ["users", "roles", "permissions", "system_settings", "user_roles", "role_permissions"];
    await resources.reduce(async (previous, resource) => {
      await previous;
      const path = `/${resource}/read`;
      const example = await page.request.get(`${path}_payload_example`);
      expect(example.status(), path).toBe(200);
      const payload = {
        ...await example.json(),
        where_many: null,
        select: [{ id: null }],
        pagination: { limit: 1, offset: 0 },
      };
      const listed = await page.request.post(path, { data: payload });
      expect(listed.status(), path).toBe(200);
      const rows = (await listed.json()).Desirable;
      expect(rows, resource).toHaveLength(1);
      const identifier = rows[0].id.value;
      await [identifier, -1].reduce(async (pending, value) => {
        await pending;
        const response = await page.request.post(path, {
          data: {
            ...payload,
            where_many: { id: { operator: "And", values: [{ Eq: { operator: "And", values: value } }] } },
          },
        });
        expect(response.status(), `${resource}: ${value}`).toBe(200);
        expect((await response.json()).Desirable).toEqual(value === identifier ? rows : []);
      }, Promise.resolve());
      expect(document.paths[path].post, path).toBeDefined();
      await ["read_one", "read_one_payload_example", "read_many", "read_many_payload_example"].reduce(async (pending, operation) => {
        await pending;
        expect(document.paths[`/${resource}/${operation}`]).toBeUndefined();
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
