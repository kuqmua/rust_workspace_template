import assert from "node:assert/strict";
import test from "node:test";
import { adminHeaders, adminOrigin, cookieValue, observeBrowserErrors } from "./support/admin.js";
import { readUsers, usersReadPage } from "./support/users.js";

test("test_cookie_lookup_preserves_first_match_and_missing_values", () => {
  const cookies = [
    { name: "other", value: "unrelated" },
    { name: "admin_csrf_token", value: "first" },
    { name: "admin_csrf_token", value: "second" }
  ];
  assert.equal(cookieValue(cookies, "admin_csrf_token"), "first");
  assert.equal(cookieValue(cookies, "missing"), undefined);
  assert.equal(cookieValue([], "admin_csrf_token"), undefined);
});

test("test_admin_headers_preserve_origin_and_require_csrf_cookie", async () => {
  assert.deepEqual(await adminHeaders({ cookies: async () => [
    { name: "other", value: "unrelated" },
    { name: "admin_csrf_token", value: "fixture" }
  ] }), { Origin: adminOrigin, "X-CSRF-Token": "fixture" });
  await assert.rejects(adminHeaders({ cookies: async () => [] }), /toBeTruthy/);
  await assert.rejects(adminHeaders({ cookies: async () => [
    { name: "admin_csrf_token", value: "" }
  ] }), /toBeTruthy/);
});

test("test_users_query_helper_preserves_defaults_aliases_and_explicit_controls", async () => {
  const cases = [
    { query: undefined, search: null, column: "id", order: "ascending", limit: 20, offset: 0 },
    { query: "search=alpha+operator&sort=status&direction=descending&limit=40&offset=5",
      search: "alpha operator", column: "is_banned", order: "descending", limit: 40, offset: 5 },
    { query: "sort=login&limit=1&offset=9", search: null, column: "login", order: "ascending", limit: 1, offset: 9 },
    { query: "search=&sort=&direction=&limit=&offset=", search: "", column: "id", order: "ascending", limit: 0, offset: 0 }
  ];
  await cases.reduce(async (previous, expected) => {
    await previous;
    const result = await readUsers({ post: async (path, options) => ({ path, ...options }) }, expected.query);
    assert.deepEqual(result, {
      path: "/users/read",
      data: {
        search: expected.search,
        where_many: null,
        select: [{ id: null }, { login: null }, { display_name: null }, { is_banned: null }],
        order_by: { column: { [expected.column]: null }, order: expected.order },
        pagination: { limit: expected.limit, offset: expected.offset }
      }
    });
  }, Promise.resolve());
});

test("test_users_response_helper_maps_column_order_and_preserves_page_metadata", async () => {
  const source = {
    columns: ["login", "must_change_password", "id", "is_banned", "display_name"].map(name => ({ name })),
    items: [
      { values: ["alpha", "true", "17", "false", "Alpha Operator"] },
      { values: ["beta", "false", "23", "true", "Beta Operator"] }
    ],
    total: 71,
    pagination: { limit: 2, offset: 5 }
  };
  const result = await usersReadPage({ json: async () => source });
  assert.deepEqual(result, {
    ...source,
    items: [
      { login: "alpha", must_change_password: true, id: 17, is_banned: false, display_name: "Alpha Operator" },
      { login: "beta", must_change_password: false, id: 23, is_banned: true, display_name: "Beta Operator" }
    ]
  });
  assert.equal(result.columns, source.columns);
  assert.deepEqual(source.items[0], { values: ["alpha", "true", "17", "false", "Alpha Operator"] });
  assert.deepEqual(await usersReadPage({ json: async () => ({ columns: [], items: [], total: 0 }) }),
    { columns: [], items: [], total: 0 });
});

test("test_users_response_helper_preserves_json_failure", async () => {
  const error = new Error("fixture JSON failure");
  await assert.rejects(usersReadPage({ json: async () => { throw error; } }), actual => actual === error);
});

test("test_browser_error_observer_filters_console_and_retains_selected_events", () => {
  [false, true].forEach(includeFailedRequests => {
    const handlers = new Map();
    const observed = observeBrowserErrors({ on: (event, callback) => handlers.set(event, callback) }, includeFailedRequests);
    handlers.get("console")({ type: () => "info", text: () => "ignored" });
    handlers.get("console")({ type: () => "error", text: () => "console fixture" });
    handlers.get("pageerror")(new Error("page fixture"));
    assert.equal(handlers.has("requestfailed"), includeFailedRequests);
    if (includeFailedRequests) handlers.get("requestfailed")({ method: () => "GET", url: () => "/missing" });
    assert.deepEqual(observed, {
      consoleErrors: ["console fixture"],
      failedRequests: includeFailedRequests ? ["GET /missing"] : [],
      pageErrors: ["page fixture"]
    });
  });
});
