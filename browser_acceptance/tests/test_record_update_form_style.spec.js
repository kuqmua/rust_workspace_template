import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_role_update_form_matches_user_update_on_desktop_and_mobile", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const width of [1440, 390]) {
    await page.setViewportSize({ width, height: 900 });
    const forms = [];
    for (const resource of ["users", "roles"]) {
      await page.goto(`/admin/${resource}/1/update`);
      const form = page.locator(`form.${resource === "users" ? "user" : "role"}-update-form`);
      await expect(form).toBeVisible();
      await expect(form.getByRole("button", { name: "update", exact: true })).toBeVisible();
      forms.push(await form.evaluate(element => {
        const properties = (element, names) => {
          const style = getComputedStyle(element);
          return Object.fromEntries(names.map(name => [name, style.getPropertyValue(name)]));
        };
        const controls = ["width", "height", "min-height", "padding", "border", "border-radius", "background-color", "color", "font-size"];
        return {
          page: properties(element.closest(".crud-page"), ["display", "gap", "padding", "width"]),
          form: properties(element, ["display", "gap", "padding", "width"]),
          fields: properties(element.querySelector(".crud-form"), ["display", "gap", "padding", "grid-template-columns"]),
          label: properties(element.querySelector('[data-name="Label"]'), ["display", "gap", "padding", "width"]),
          caption: properties(element.querySelector('[data-name="Label"] > span'), controls),
          input: properties(element.querySelector('[data-name="Input"]'), controls),
          button: properties(element.querySelector(".crud-actions > button"), controls)
        };
      }));
      await form.locator('input:not([type="hidden"])').first().fill("Style fixture");
      await expect(form.locator('input:not([type="hidden"])').first()).toHaveValue("Style fixture");
    }
    expect(forms[1]).toEqual(forms[0]);
  }
});
