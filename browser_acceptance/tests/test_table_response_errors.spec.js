import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

[
  ["http_error", route => route.fulfill({ status: 500, body: "x" }), /the_server_returned_status_500_for_/],
  ["invalid_json", route => route.fulfill({ status: 200, contentType: "application/json", body: "x" }), "the_table_response_was_invalid"],
  ["fetch_failure", route => route.abort("failed"), "the_table_request_failed"],
].forEach(([name, intercept, message]) => {
  test(`test_table_${name}_is_visible_without_rows`, async ({ page }) => {
    await signInInitialAdministrator(page);
    await page.route("**/users/read", intercept);
    try {
      await page.goto("/admin/users");
      const content = page.getByRole("main");
      await expect(content.getByRole("alert")).toBeVisible();
      await expect(content.getByRole("alert")).toContainText(message);
      await expect(content.locator("tbody tr")).toHaveCount(0);
    } finally {
      await page.unroute("**/users/read", intercept);
      await signOutIfAuthenticated(page);
    }
  });
});

[
  ["refresh_success_retry_success", 204, 200, null],
  ["refresh_success_retry_unauthorized", 204, 401, "the_server_returned_status_401_for_/users/read"],
  ["refresh_failure_retry_success", 503, 200, null],
  ["refresh_failure_retry_failure", 503, 500, "the_server_returned_status_503_for_/auth/refresh"],
].forEach(([name, refreshStatus, retryStatus, message]) => {
  test(`test_table_${name}_retries_once_and_preserves_error_precedence`, async ({ page }) => {
    await signInInitialAdministrator(page);
    await page.goto("/admin/users");
    await expect(page.getByRole("main").locator("tbody tr").first()).toBeVisible();
    const sequence = [];
    const bodies = [];
    const read = async route => {
      sequence.push("read");
      expect(route.request().method()).toBe("POST");
      expect(route.request().headers()["content-type"]).toBe("application/json");
      bodies.push(route.request().postDataJSON());
      if (bodies.length === 1) {
        await route.fulfill({ status: 401, body: "x" });
      } else if (retryStatus === 200) {
        await route.continue();
      } else {
        await route.fulfill({ status: retryStatus, body: "x" });
      }
    };
    const refresh = async route => {
      sequence.push("refresh");
      expect(route.request().method()).toBe("POST");
      expect(route.request().postDataJSON()).toBeNull();
      await route.fulfill({ status: refreshStatus, body: refreshStatus === 204 ? "" : "x" });
    };
    await page.route("**/users/read", read);
    await page.route("**/auth/refresh", refresh);
    try {
      await page.reload();
      const content = page.getByRole("main");
      if (message === null) {
        await expect(content.locator("tbody tr").first()).toBeVisible();
        await expect(content.getByRole("alert")).toHaveCount(0);
      } else {
        await expect(content.getByRole("alert")).toContainText(message);
        await expect(content.locator("tbody tr")).toHaveCount(0);
      }
      expect(sequence).toEqual(["read", "refresh", "read"]);
      expect(bodies).toHaveLength(2);
      expect(bodies[1]).toEqual(bodies[0]);
    } finally {
      await page.unroute("**/users/read", read);
      await page.unroute("**/auth/refresh", refresh);
      await signOutIfAuthenticated(page);
    }
  });
});

[
  ["http_error", route => route.fulfill({ status: 500, body: "x" }), "the_server_returned_status_500_for_/auth/me/read"],
  ["invalid_json", route => route.fulfill({ status: 200, contentType: "application/json", body: "x" }), "the_table_response_was_invalid"],
  ["fetch_failure", route => route.abort("failed"), "the_table_request_failed"],
].forEach(([name, intercept, message]) => {
  test(`test_initial_administrator_${name}_prevents_table_requests`, async ({ page }) => {
    await signInInitialAdministrator(page);
    const requests = [];
    const observe = request => {
      const path = new URL(request.url()).pathname;
      if (["/auth/me/read", "/users/read", "/auth/refresh"].includes(path)) requests.push(path);
    };
    page.on("request", observe);
    await page.route("**/auth/me/read", intercept);
    try {
      await page.goto("/admin/users");
      const content = page.getByRole("main");
      await expect(content.getByRole("alert")).toContainText(message);
      await expect(content.locator("tbody tr")).toHaveCount(0);
      await expect(page.locator(".loading-state")).toHaveCount(0);
      await expect(page.locator("header a")).toHaveCount(0);
      expect(requests).toEqual(["/auth/me/read"]);
    } finally {
      page.off("request", observe);
      await page.unroute("**/auth/me/read", intercept);
      await signOutIfAuthenticated(page);
    }
  });
});
