import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

const paths = [
  ...tablePages.map(page => page.path),
  ...dataTablePages.map(page => page.path),
  "/admin/profile", "/admin/settings", "/admin/users/create", "/admin/users/manage",
  "/admin/roles/create", "/admin/roles/manage", "/admin/metrics", "/admin/version"
];

async function interfaceTextViolations(page) {
  return page.evaluate(() => {
    const violations = [];
    const snakeCase = /^[a-z0-9]+(?:_[a-z0-9]+)*$/;
    const dataSelector = "script, style, textarea, pre, dd, .crud-record-heading h2, .table-cell-preview";
    const check = (element, text, source) => {
      const value = text.trim();
      if (value && /[a-z]/i.test(value) && (!snakeCase.test(value) || getComputedStyle(element).textTransform !== "none")) {
        violations.push({ source, value, element: element.tagName });
      }
    };
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    let node;
    while ((node = walker.nextNode())) {
      const element = node.parentElement;
      if (!element || element.closest(dataSelector)) continue;
      if (element.closest("tbody") && !element.closest(".table-actions, button, a, label")) continue;
      if (!element.checkVisibility()) continue;
      check(element, node.textContent, "text");
    }
    document.querySelectorAll("[aria-label], [title], [placeholder]").forEach(element => {
      if (!element.checkVisibility()) return;
      ["aria-label", "title", "placeholder"].forEach(attribute => {
        if (element.hasAttribute(attribute)) check(element, element.getAttribute(attribute), attribute);
      });
    });
    return violations;
  });
}

async function assertInterfaceText(page) {
  expect(await interfaceTextViolations(page)).toEqual([]);
}

test("test_interface_labels_are_snake_case_across_all_admin_pages", async ({ page }) => {
  test.setTimeout(120_000);
  await page.goto("/admin/sign_in");
  await assertInterfaceText(page);
  await signInInitialAdministrator(page);
  for (const path of paths) {
    await page.goto(path);
    if ([...tablePages, ...dataTablePages].some(item => item.path === path)) {
      await expect(page.locator("table")).toBeVisible();
    } else if (["/admin/profile", "/admin/settings"].includes(path)) {
      await expect(page.getByLabel(path === "/admin/profile" ? "current_password" : "site_name", { exact: true })).toBeVisible();
    }
    await assertInterfaceText(page);
    if (path === "/admin/profile") {
      await expect(page.locator("dd").nth(1)).toHaveText("Initial Administrator");
    }
    if (path === "/admin/settings" || path === "/admin/sessions") {
      const trigger = path === "/admin/settings" ? "reset_to_template_defaults" : "revoke_session";
      await page.getByRole("button", { name: trigger, exact: true }).first().click();
      await expect(page.getByRole("dialog")).toBeVisible();
      await assertInterfaceText(page);
      await page.keyboard.press("Escape");
    }
    const filter = page.locator(".table-column-filter button").first();
    if (await filter.count()) {
      await filter.click();
      await assertInterfaceText(page);
      await page.keyboard.press("Escape");
    }
  }
  await signOutIfAuthenticated(page);
});

test("test_mobile_snake_case_labels_wrap_without_changing_account_data", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.setViewportSize({ width: 320, height: 844 });
  for (const path of ["/admin/users", "/admin/user_roles", "/admin/profile", "/admin/settings", "/admin/users/create", "/admin/roles/manage"]) {
    await page.goto(path);
    if (["/admin/profile", "/admin/settings"].includes(path)) {
      await expect(page.getByLabel(path === "/admin/profile" ? "current_password" : "site_name", { exact: true })).toBeVisible();
    }
    if (["/admin/users", "/admin/user_roles"].includes(path)) {
      await expect(page.locator("table")).toBeVisible();
      const labels = await page.locator("td[data-label]").evaluateAll(elements => elements.map(element => ({ text: element.dataset.label, transform: getComputedStyle(element, "::before").textTransform })));
      labels.forEach(label => {
        expect(label.text).toMatch(/^[a-z0-9]+(?:_[a-z0-9]+)*$/);
        expect(label.transform).toBe("none");
      });
    }
    await assertInterfaceText(page);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth)).toBeLessThanOrEqual(1);
    if (path === "/admin/profile") {
      await expect(page.locator("dd").nth(1)).toHaveText("Initial Administrator");
    }
  }
  await page.setViewportSize({ width: 1280, height: 720 });
  await signOutIfAuthenticated(page);
});

test("test_loading_state_has_snake_case_accessible_labels", async ({ page }) => {
  await signInInitialAdministrator(page);
  let release;
  const gate = new Promise(resolve => { release = resolve; });
  await page.route("**/v1/admin/users**", async route => {
    await gate;
    await route.continue();
  });
  try {
    const response = await page.goto("/admin/users");
    const html = await response.text();
    expect(html).not.toContain('aria-label="Loading"');
    expect(html).toContain('aria-label="loading"');
    await expect(page.locator(".loading-state")).toBeVisible();
    await expect(page.locator(".loading-state")).toHaveAttribute("aria-label", "loading");
    await assertInterfaceText(page);
  } finally {
    release();
    await page.unrouteAll({ behavior: "wait" });
  }
  await expect(page.locator("table")).toBeVisible();
  await signOutIfAuthenticated(page);
});

test("test_label_audit_rejects_non_snake_case_text_and_accessible_labels", async ({ page }) => {
  await page.setContent(`
    <button>Save changes</button>
    <button aria-label="Close dialog" title="View details">close</button>
    <textarea placeholder="Type here">Database Text Must Stay Unchanged</textarea>
    <table><tbody><tr>
      <td>Database Text Must Stay Unchanged</td>
      <td><button>Delete user</button></td>
    </tr></tbody></table>
    <span style="text-transform: uppercase">save_changes</span>
    <span hidden>Hidden text</span>
  `);
  expect(await interfaceTextViolations(page)).toEqual([
    { source: "text", value: "Save changes", element: "BUTTON" },
    { source: "text", value: "Delete user", element: "BUTTON" },
    { source: "text", value: "save_changes", element: "SPAN" },
    { source: "aria-label", value: "Close dialog", element: "BUTTON" },
    { source: "title", value: "View details", element: "BUTTON" },
    { source: "placeholder", value: "Type here", element: "TEXTAREA" }
  ]);
});

test("test_label_audit_preserves_database_text_and_checks_preview_controls", async ({ page }) => {
  const value = "Mixed Case / Unicode: \u041f\u0440\u0438\u0432\u0435\u0442 \u4e16\u754c & <text>";
  await page.setContent(`
    <button>save_changes</button>
    <label>display_name<input value="Mixed Case"></label>
    <textarea placeholder="organization_contacts"></textarea>
    <dl><dt>display_name</dt><dd></dd></dl>
    <pre></pre>
    <table><tbody><tr>
      <td class="stored-value"></td>
      <td><button class="table-cell-preview" title="view_full_value"></button></td>
      <td><button>delete_user</button></td>
    </tr></tbody></table>
  `);
  const data = page.locator("textarea, dd, pre, .stored-value, .table-cell-preview");
  await data.evaluateAll((elements, value) => elements.forEach(element => { element.textContent = value; }), value);
  await assertInterfaceText(page);
  expect(await data.allTextContents()).toEqual(Array(5).fill(value));
  await page.locator(".table-cell-preview").evaluate(element => { element.title = "View full value"; });
  expect(await interfaceTextViolations(page)).toEqual([
    { source: "title", value: "View full value", element: "BUTTON" }
  ]);
  expect(await data.allTextContents()).toEqual(Array(5).fill(value));
});
