import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

[
  "limit=0",
  "limit=101",
  "limit=invalid",
  "limit=65536",
  "limit=",
  "limit=1&limit=2",
  "offset=-1",
  "offset=4294967296",
  "offset=invalid",
  "offset=",
  "offset=0&offset=1"
].forEach(query => {
  test(`test_csr_pagination_rejects_${query}`, async ({ page }) => {
    await signInAdministratorWithPasswordReset(page);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await page.addInitScript(query => {
      if (location.pathname === "/admin/users") {
        history.replaceState(null, "", `${location.pathname}?${query}`);
      }
    }, query);
    const readRequests = [];
    page.on("request", request => {
      if (request.method() === "POST" && new URL(request.url()).pathname === "/users/read") {
        readRequests.push(request);
      }
    });
    await page.goto("/admin/users");
    await expect(page.getByText("the_table_query_is_invalid", { exact: true })).toBeVisible();
    expect(readRequests).toHaveLength(0);
  });
});

[
  { query: "", limit: 20, offset: 0 },
  { query: "limit=1&offset=0", limit: 1, offset: 0 },
  { query: "limit=100&offset=42", limit: 100, offset: 42 }
].forEach(({ query, limit, offset }) => {
  test(`test_csr_pagination_preserves_${query || "defaults"}`, async ({ page }) => {
    await signInAdministratorWithPasswordReset(page);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    const readRequest = page.waitForRequest(request =>
      request.method() === "POST" && new URL(request.url()).pathname === "/users/read"
    );
    await page.goto(`/admin/users${query ? `?${query}` : ""}`);
    expect((await readRequest).postDataJSON().pagination).toEqual({ limit, offset });
  });
});
