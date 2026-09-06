import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

test.beforeEach(async ({ page }) => {
  await signInInitialAdministrator(page);
});

test.afterEach(async ({ page }) => {
  const dialog = page.locator(".table-cell-dialog[open]");
  if (await dialog.count()) await page.keyboard.press("Escape");
  await page.setViewportSize({ width: 1280, height: 720 });
  await signOutIfAuthenticated(page);
});

[...tablePages, ...dataTablePages].forEach(({ name, path }) => {
  test(`test_${name}_values_have_a_full_content_viewer`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator(".table-scroll table")).toBeVisible();
    const values = page.locator('tbody td:not([data-label="actions"])');
    const previews = values.locator(".table-cell-preview");
    await expect(previews).toHaveCount(await values.count());
    await expect(page.locator('td[data-label="actions"] .table-cell-preview')).toHaveCount(0);
    if (await previews.count()) {
      const preview = previews.first();
      const value = await preview.textContent();
      await preview.click();
      const dialog = page.getByRole("dialog");
      await expect(dialog.locator("pre")).toHaveText(value);
      await dialog.getByRole("button", { name: "close", exact: true }).click();
      await expect(page.locator(".table-cell-dialog")).toHaveCount(0);
      await expect(preview).toBeFocused();
    }
  });
});

test("test_long_production_value_is_ellipsized_and_keyboard_accessible", async ({ page }) => {
  await page.goto("/admin/roles");
  const preview = page.locator('td[data-label="permissions"] .table-cell-preview').first();
  await expect(preview).toBeVisible();
  const value = await preview.textContent();
  expect(value.length).toBeGreaterThan(24);
  expect(await preview.evaluate(element => element.scrollWidth > element.clientWidth)).toBe(true);
  await expect(preview).toHaveCSS("text-overflow", "ellipsis");
  await preview.focus();
  await page.keyboard.press("Enter");
  const dialog = page.getByRole("dialog", { name: "permissions", exact: true });
  await expect(dialog.locator("pre")).toHaveText(value);
  await expect(dialog.getByRole("button", { name: "close", exact: true })).toBeFocused();
  await page.keyboard.press("Escape");
  await expect(dialog).toHaveCount(0);
  await expect(preview).toBeFocused();
  await page.keyboard.press("Space");
  await expect(dialog).toBeVisible();
});

test("test_mobile_viewer_preserves_multiline_unicode_and_displays_markup_as_text", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await page.goto("/admin/users");
  const preview = page.locator('td[data-label="display_name"] .table-cell-preview').first();
  const value = ('Unicode \u{1F600} e\u0301\n<img src="invalid" onerror="window.previewInjected=true">\n').repeat(400) + 'End of value';
  await preview.evaluate((element, value) => { element.textContent = value; }, value);
  const rowHeight = await page.locator("tbody td").first().evaluate(element => element.getBoundingClientRect().height);
  expect(rowHeight).toBe(36);
  expect(await preview.evaluate(element => element.scrollWidth > element.clientWidth)).toBe(true);
  await preview.click();
  const dialog = page.getByRole("dialog", { name: "display_name", exact: true });
  const content = dialog.locator("pre");
  expect(await content.textContent()).toBe(value);
  await expect(dialog.locator("img")).toHaveCount(0);
  expect(await page.evaluate(() => window.previewInjected)).toBeUndefined();
  expect(await dialog.evaluate(element => element.scrollWidth - element.clientWidth)).toBe(0);
  const box = await dialog.boundingBox();
  expect(box.y).toBeGreaterThanOrEqual(0);
  expect(box.y + box.height).toBeLessThanOrEqual(568);
  expect(await content.evaluate(element => element.scrollHeight > element.clientHeight)).toBe(true);
  await content.selectText();
  expect(await page.evaluate(() => getSelection().toString())).toBe(value);
  await content.evaluate(element => { element.scrollTop = element.scrollHeight; });
  expect(await content.evaluate(element => element.scrollTop)).toBeGreaterThan(0);
  await dialog.getByRole("button", { name: "close", exact: true }).click();
  await expect(preview).toBeFocused();
  await expect(page.locator(".table-cell-dialog")).toHaveCount(0);
});

test("test_empty_values_remain_accessible_and_do_not_reuse_previous_content", async ({ page }) => {
  await page.goto("/admin/users");
  const preview = page.locator('td[data-label="display_name"] .table-cell-preview').first();
  await preview.click();
  await page.keyboard.press("Escape");
  await preview.evaluate(element => { element.textContent = ""; });
  await expect(preview).toBeVisible();
  await preview.click();
  const dialog = page.getByRole("dialog", { name: "display_name", exact: true });
  expect(await dialog.locator("pre").textContent()).toBe("");
  await dialog.getByRole("button", { name: "close", exact: true }).click();
  await expect(preview).toBeFocused();
});
