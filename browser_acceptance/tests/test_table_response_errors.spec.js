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
