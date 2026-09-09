import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

test("test_all_data_tables_have_empty_actions_column", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    const pages = [...tablePages.filter(value => value.name !== "sessions"), ...dataTablePages];
    await pages.reduce(async (previous, { path }) => {
      await previous;
      await page.goto(path);
      const table = page.locator('section[data-renderer="csr"] table');
      await expect(table.locator("thead th").last()).toHaveText("actions");
      const valid = await table.evaluate(element => {
        const columns = element.querySelectorAll("thead th").length;
        return [...element.querySelectorAll("tbody tr")].every(row => {
          const cell = row.lastElementChild;
          return row.children.length === columns
            && cell.dataset.label === "actions"
            && cell.textContent === ""
            && !cell.querySelector("button, a, input");
        });
      });
      expect(valid, path).toBe(true);
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});
