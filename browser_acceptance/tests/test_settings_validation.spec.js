import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_invalid_settings_value_reports_validation_failure", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/settings");
  await page.getByLabel("default_route").fill("/admin/not_a_page");
  let updateRequests = 0;
  page.on("request", request => {
    if (request.method() === "PATCH" && new URL(request.url()).pathname === "/system_settings/update") {
      updateRequests += 1;
    }
  });
  await page.getByRole("button", { name: "save_settings" }).click();
  await expect(page.getByText("check_settings_values", { exact: true })).toBeVisible();
  expect(updateRequests).toBe(0);
  await expect(page.getByLabel("default_route")).toHaveValue("/admin/not_a_page");
});
