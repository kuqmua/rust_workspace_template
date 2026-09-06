import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

test.beforeEach(async ({ page }) => {
  await signInInitialAdministrator(page);
});

test.afterEach(async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await signOutIfAuthenticated(page);
});

async function expectTableFillsPage(page) {
  await expect.poll(async () => page.locator(".table-scroll").evaluate(element => {
    const footer = element.parentElement.lastElementChild;
    const main = element.closest("main");
    return {
      bottomGap: Math.abs(footer.getBoundingClientRect().bottom - window.innerHeight),
      tableGap: Math.abs(footer.getBoundingClientRect().top - element.getBoundingClientRect().bottom),
      pageOverflow: main.scrollHeight - main.clientHeight
    };
  })).toEqual({ bottomGap: 0, tableGap: 0, pageOverflow: 0 });
}

[...tablePages, ...dataTablePages].forEach(({ name, path }) => {
  test(`test_${name}_table_fills_page_and_scrolls_only_rows`, async ({ page }) => {
    await page.goto(path);
    const table = page.locator(".table-scroll");
    await expect(table.locator("table")).toBeVisible();
    const viewports = [
      { width: 1920, height: 1080 }, { width: 1280, height: 720 },
      { width: 768, height: 720 }, { width: 390, height: 844 },
      { width: 320, height: 568 }
    ];
    await viewports.reduce(async (previous, viewport) => {
      await previous;
      await page.setViewportSize(viewport);
      await [0, 1, 100].reduce(async (previous, count) => {
        await previous;
        await table.evaluate((element, count) => {
          const body = element.querySelector("tbody");
          const header = element.querySelector("thead tr");
          const row = body.querySelector("tr")?.cloneNode(false) || document.createElement("tr");
          const cells = Array.from(header.children, heading => {
            const cell = document.createElement("td");
            cell.dataset.label = heading.textContent;
            cell.textContent = heading.textContent;
            return cell;
          });
          row.replaceChildren(...cells);
          body.replaceChildren(...Array.from({ length: count }, () => row.cloneNode(true)));
          element.scrollTop = 0;
        }, count);
        await expectTableFillsPage(page);
        const before = await table.evaluate(element => ({
          headerTop: element.querySelector("thead").getBoundingClientRect().top,
          footerTop: element.parentElement.lastElementChild.getBoundingClientRect().top,
          footerHeight: element.parentElement.lastElementChild.getBoundingClientRect().height,
          rowTop: element.querySelector("tbody tr")?.getBoundingClientRect().top,
          overflow: element.scrollHeight - element.clientHeight
        }));
        if (count === 100) {
          expect(before.overflow).toBeGreaterThan(0);
          expect(before.footerTop + before.footerHeight).toBe(page.viewportSize().height);
          await table.evaluate(element => { element.scrollTop = element.scrollHeight; });
          await expect.poll(async () => table.evaluate(element => element.scrollTop)).toBeGreaterThan(0);
          const after = await table.evaluate(element => ({
            headerTop: element.querySelector("thead").getBoundingClientRect().top,
            footerTop: element.parentElement.lastElementChild.getBoundingClientRect().top,
            rowTop: element.querySelector("tbody tr").getBoundingClientRect().top
          }));
          expect(after.headerTop).toBe(before.headerTop);
          expect(after.footerTop).toBe(before.footerTop);
          expect(after.rowTop).toBeLessThan(before.rowTop);
          await expectTableFillsPage(page);
        } else {
          expect(before.overflow).toBe(0);
        }
      }, Promise.resolve());
    }, Promise.resolve());
  });
});

test("test_table_reserves_space_for_interface_below_pagination", async ({ page }) => {
  await page.goto("/admin/users");
  await expect(page.locator(".table-pagination")).toBeVisible();
  await page.locator(".table-pagination").evaluate(element => {
    const footer = document.createElement("footer");
    footer.dataset.testid = "table-footer-fixture";
    footer.style.height = "48px";
    footer.textContent = "Additional interface";
    element.after(footer);
  });
  const footer = await page.getByTestId("table-footer-fixture").boundingBox();
  const pagination = await page.locator(".table-pagination").boundingBox();
  expect(footer.y + footer.height).toBe(page.viewportSize().height);
  expect(pagination.y + pagination.height).toBe(footer.y);
});
