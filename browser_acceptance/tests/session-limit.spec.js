import { expect, test } from "@playwright/test";
import {
  adminOrigin,
  bootstrapAdministrator,
  signIn
} from "./support/admin.js";

test.describe.configure({ mode: "serial" });
test.skip(
  process.env.BROWSER_ACCEPTANCE_SESSION_LIMIT !== "2",
  "session-limit acceptance requires an isolated server configured with limit two"
);

test("the third sign-in evicts only the oldest session when the limit is two", async ({
  browser
}) => {
  const firstContext = await browser.newContext({ baseURL: adminOrigin });
  const firstPage = await firstContext.newPage();
  await bootstrapAdministrator(firstPage);

  const secondContext = await browser.newContext({ baseURL: adminOrigin });
  const secondPage = await secondContext.newPage();
  await signIn(secondPage);
  await expect(secondPage).toHaveURL(/\/admin\/users$/);

  const thirdContext = await browser.newContext({ baseURL: adminOrigin });
  const thirdPage = await thirdContext.newPage();
  await signIn(thirdPage);
  await expect(thirdPage).toHaveURL(/\/admin\/users$/);

  expect(
    (await firstPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(401);
  expect(
    (await secondPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(200);
  expect(
    (await thirdPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(200);

  await firstContext.close();
  await secondContext.close();
  await thirdContext.close();
});
