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
  "offset=0&offset=1",
  "search=one&search=two",
  "sort=login&sort=created_at",
  "direction=ascending&direction=descending",
  "filter_field=id&filter_field=login&filter_operation=eq&filter_value=1",
  "filter_field=id&filter_operation=eq&filter_operation=in&filter_value=1",
  "filter_field=id&filter_operation=eq&filter_value=1&filter_value=2",
  "filter_field=id&filter_operation=between&filter_value=1&filter_end=2&filter_end=3"
].forEach(query => {
  test(`test_csr_query_rejects_${query}`, async ({ page }) => {
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

test("test_csr_rules_rejects_unknown_filter_field", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const readRequests = [];
  page.on("request", request => {
    if (request.method() === "POST" && new URL(request.url()).pathname === "/rules/read") {
      readRequests.push(request);
    }
  });
  await page.goto("/admin/rules?filter_field=unknown&filter_operation=eq&filter_value=1");
  await expect(page.getByText("the_table_query_is_invalid", { exact: true })).toBeVisible();
  expect(readRequests).toHaveLength(0);
});

test("test_csr_system_settings_rejects_unknown_filter_field", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const readRequests = [];
  page.on("request", request => {
    if (request.method() === "POST" && new URL(request.url()).pathname === "/system_settings/read") {
      readRequests.push(request);
    }
  });
  await page.goto("/admin/system_settings?filter_field=unknown&filter_operation=eq&filter_value=1");
  await expect(page.getByText("the_table_query_is_invalid", { exact: true })).toBeVisible();
  expect(readRequests).toHaveLength(0);
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

test("test_roles_column_filter_preserves_table_query", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/roles?search=admin&sort=id&direction=descending");
  const filterForm = page.locator(".table-filter-form").first();
  await expect(filterForm).toBeAttached();
  const values = await filterForm.evaluate(form => Object.fromEntries(new FormData(form)));
  expect(values).toMatchObject({ search: "admin", sort: "id", direction: "descending" });
});

test("test_roles_clear_filter_preserves_table_query", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/roles?search=admin&sort=id&direction=descending&limit=1&filter_field=id&filter_operation=eq&filter_value=1");
  const clear = page.locator(".table-filter-clear").first();
  await expect(clear).toBeAttached();
  const values = await clear.evaluate(element => {
    const form = element.closest("form");
    return form
      ? Object.fromEntries(new FormData(form))
      : Object.fromEntries(new URL(element.href).searchParams);
  });
  expect(values).toMatchObject({ search: "admin", sort: "id", direction: "descending", limit: "1" });
  expect(values).not.toHaveProperty("filter_field");
  await clear.evaluate(element => element.click());
  await expect.poll(() => Object.fromEntries(new URL(page.url()).searchParams)).toEqual({
    search: "admin",
    sort: "id",
    direction: "descending",
    limit: "1"
  });
});

test("test_sessions_page_forwards_pagination_and_filter_query", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const request = page.waitForRequest(candidate =>
    candidate.method() === "GET" && new URL(candidate.url()).pathname === "/auth/sessions/read"
  );
  await page.goto("/admin/sessions?limit=1&offset=1&filter_field=current&filter_operation=eq&filter_value=true");
  const searchParams = new URL((await request).url()).searchParams;
  expect(Object.fromEntries(searchParams)).toMatchObject({
    limit: "1",
    offset: "1",
    filter_field: "current",
    filter_operation: "eq",
    filter_value: "true"
  });
});
