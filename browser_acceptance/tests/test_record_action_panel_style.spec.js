import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_roles_action_panel_matches_users_on_desktop_and_mobile", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const width of [1440, 390]) {
    await page.setViewportSize({ width, height: 900 });
    const panels = [];
    for (const resource of ["users", "roles"]) {
      await page.goto(`/admin/${resource}`);
      const panel = page.locator('tbody tr td[data-label="actions"] .table-actions').first();
      await expect(panel.getByRole("link", { name: "read", exact: true })).toBeVisible();
      await expect(panel.getByRole("link", { name: "update", exact: true })).toBeVisible();
      panels.push(await panel.evaluate(element => {
        const properties = (element, names) => {
          const style = getComputedStyle(element);
          return Object.fromEntries(names.map(name => [name, style.getPropertyValue(name)]));
        };
        return {
          panel: properties(element, ["display", "gap", "padding", "align-items", "flex-wrap", "min-width"]),
          buttons: Array.from(element.querySelectorAll(":scope > a"), button => ({
            label: button.getAttribute("aria-label"),
            style: properties(button, ["width", "height", "min-width", "min-height", "padding", "border", "border-radius", "background-color", "color", "font-size"]),
            icon: properties(button.querySelector("svg"), ["width", "height", "fill"])
          }))
        };
      }));
    }
    expect(panels[1]).toEqual(panels[0]);
    expect(panels[0].panel.gap).toBe("0px");
    expect(panels[0].buttons.map(button => button.label)).toEqual(["read", "update"]);
  }
});
