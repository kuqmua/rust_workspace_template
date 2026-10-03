import { expect, test } from "@playwright/test";
import {
  signInAdministratorWithPasswordReset,
  signOutIfAuthenticated
} from "./support/admin.js";

test("test_sessions_rows_expose_read_and_icon_revoke_actions", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    await page.goto("/admin/sessions");
    const row = page.locator("tbody tr").first();
    const sessionId = (await row.locator('td[data-label="id"]').innerText()).trim();
    const read = row.getByRole("link", { name: "read", exact: true });
    const readPath = `/admin/access_sessions/${sessionId}/read`;
    await expect(read).toHaveAttribute("href", readPath);
    await read.click();
    await expect(page).toHaveURL(readPath);
    await expect(page.locator('[data-page="access-session-read"] .health-result').first()).toHaveText(sessionId);
    await page.goBack();
    await expect(page).toHaveURL("/admin/sessions");
    await expect(read.locator("svg")).toHaveCount(1);
    const revokeButton = row.getByRole("button", { name: "revoke_session", exact: true });
    await expect(revokeButton.locator("svg")).toHaveCount(1);
    const actionStyles = await Promise.all(
      [read, revokeButton].map((action) =>
        action.evaluate((element) => {
          const style = getComputedStyle(element);
          return {
            backgroundColor: style.backgroundColor,
            borderColor: style.borderColor,
            color: style.color,
            height: style.height,
            padding: style.padding,
            width: style.width
          };
        })
      )
    );
    expect(actionStyles[1]).toEqual(actionStyles[0]);
    await expect(row.locator(".table-actions")).toHaveCSS("flex-wrap", "nowrap");
    await expect(row.locator(".table-actions")).toHaveCSS("gap", "0px");
    await expect(read).toHaveCSS("border-width", "0px");
    await expect(revokeButton).toHaveCSS("border-width", "0px");
    await expect(read).toHaveCSS("box-shadow", "none");
    await expect(revokeButton).toHaveCSS("box-shadow", "none");
    await revokeButton.click();
    await expect(row.getByRole("dialog", { name: "revoke_session" })).toBeVisible();
  } finally {
    await signOutIfAuthenticated(page);
  }
});
