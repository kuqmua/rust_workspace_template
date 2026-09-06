import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { navigationAdminPaths } from "./support/pages.js";

async function expectSharedFontFamily(page) {
  const fonts = await page.evaluate(() => {
    const family = getComputedStyle(document.documentElement).fontFamily;
    const samples = [document.body, ...document.body.querySelectorAll("*")].flatMap(element => {
      const hasText = Array.from(element.childNodes).some(node =>
        node.nodeType === Node.TEXT_NODE && node.textContent.trim().length > 0
      );
      const control = element.matches("input, textarea, select, button");
      const samples = hasText || control ? [{
        element: element.tagName,
        family: getComputedStyle(element).fontFamily
      }] : [];
      ["::before", "::after", "::marker", "::placeholder", "::file-selector-button"].forEach(pseudo => {
        const style = getComputedStyle(element, pseudo);
        const generated = style.content && !["none", "normal"].includes(style.content);
        if (generated || (pseudo === "::marker" && getComputedStyle(element).display === "list-item") ||
            (pseudo === "::placeholder" && element.hasAttribute("placeholder")) ||
            (pseudo === "::file-selector-button" && element.matches('input[type="file"]'))) {
          samples.push({ element: `${element.tagName}${pseudo}`, family: style.fontFamily });
        }
      });
      return samples;
    });
    return { family, samples };
  });
  expect(fonts.family).not.toBe("");
  expect(fonts.samples.length).toBeGreaterThan(0);
  expect(fonts.samples.filter(sample => sample.family !== fonts.family), page.url()).toEqual([]);
}

test("test_sign_in_uses_one_font_family", async ({ page }) => {
  await page.goto("/admin/sign_in");
  await expect(page.getByRole("button", { name: "sign_in", exact: true })).toBeVisible();
  await expectSharedFontFamily(page);
});

test("test_code_and_native_controls_use_the_interface_font", async ({ page }) => {
  await page.goto("/admin/sign_in");
  await expect(page.getByRole("button", { name: "sign_in", exact: true })).toBeVisible();
  await page.evaluate(() => {
    const section = document.createElement("section");
    section.innerHTML = '<style>.font-fixture::before, .font-fixture::after {' +
      'content: "Generated text"; font-family: monospace !important; }</style>' +
      '<pre>Preformatted text</pre><code class="font-mono">Code text</code>' +
      '<kbd>Keyboard shortcut</kbd><samp>Program output</samp>' +
      '<span style="font-family: monospace">Explicit legacy font</span>' +
      '<input placeholder="Input placeholder"><textarea placeholder="Textarea placeholder"></textarea>' +
      '<input type="file"><select><option>Option text</option></select>' +
      '<ul><li>List text</li></ul><button>Fixture button</button>' +
      '<span class="font-fixture">Generated content fixture</span>';
    document.body.append(section);
  });
  await expectSharedFontFamily(page);
});

test.describe("administrator typography", () => {
  test.beforeEach(async ({ page }) => {
    await signInInitialAdministrator(page);
  });

  test.afterEach(async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    await signOutIfAuthenticated(page);
  });

  const paths = [
    ...navigationAdminPaths.filter(path => path !== "/admin/swagger_ui"),
    "/admin/users/create", "/admin/users/manage",
    "/admin/roles/create", "/admin/roles/manage"
  ];

  paths.forEach(path => {
    test(`test_shared_font_on_${path.slice(1).replaceAll("/", "_")}`, async ({ page }) => {
      await page.goto(path);
      await expect(page.locator("main")).toBeVisible();
      await expect(page.getByText("loading", { exact: true })).toHaveCount(0);
      await expect(page.locator("main")).not.toBeEmpty();
      await expect(page.getByRole("alert")).toHaveCount(0);
      await expectSharedFontFamily(page);
      await page.setViewportSize({ width: 390, height: 844 });
      await page.getByText("navigation", { exact: true }).click();
      await expect(page.locator("header nav")).toBeVisible();
      await expectSharedFontFamily(page);
    });
  });

  test("test_dialog_and_filter_popover_use_the_interface_font", async ({ page }) => {
    await page.goto("/admin/settings");
    await page.getByRole("button", { name: "reset_to_template_defaults", exact: true }).click();
    const confirmation = page.getByRole("dialog", { name: "reset_settings" });
    await expect(confirmation).toBeVisible();
    await expectSharedFontFamily(page);
    await confirmation.getByRole("button", { name: "cancel", exact: true }).click();
    await page.goto("/admin/role_permissions");
    await page.getByRole("button", { name: "filter_role_id", exact: true }).click();
    const filter = page.getByRole("dialog", { name: "filter_role_id", exact: true });
    await expect(filter).toBeVisible();
    await expectSharedFontFamily(page);
    await filter.getByRole("button", { name: "close", exact: true }).click();
  });
});
