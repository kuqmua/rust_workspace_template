import { expect, test } from "@playwright/test";
import { adminHeaders, adminOrigin, changedAdminPassword, initialAdminPassword } from "./support/admin.js";
import { readUsers, usersReadPage } from "./support/users.js";

test("test_users_read_replaces_legacy_list_with_search_filters_roles_and_total", async ({ context }) => {
  const request = context.request;
  const login = "administrator";
  let signIn = await request.post("/auth/sign_in", {
    data: { login, password: changedAdminPassword },
    headers: { Origin: adminOrigin }
  });
  if (signIn.status() === 401) {
    signIn = await request.post("/auth/sign_in", {
      data: { login, password: initialAdminPassword },
      headers: { Origin: adminOrigin }
    });
    expect(signIn.status()).toBe(200);
    const password = await request.post("/auth/password", {
      data: { current_password: initialAdminPassword, new_password: changedAdminPassword },
      headers: await adminHeaders(context)
    });
    expect(password.status()).toBe(204);
    signIn = await request.post("/auth/sign_in", {
      data: { login, password: changedAdminPassword },
      headers: { Origin: adminOrigin }
    });
  }
  expect(signIn.status()).toBe(200);

  const response = await readUsers(request, "search=Initial&limit=1");
  expect(response.status()).toBe(200);
  const page = await usersReadPage(response);
  expect(page.total).toBe(1);
  expect(page.items).toHaveLength(1);
  expect(page.items[0].login).toBe(login);
  expect(page.items[0].display_name).toBe("Initial Administrator");
  expect(page.items[0].role_ids.length).toBeGreaterThan(0);
  expect(page.roles.length).toBeGreaterThan(0);

  const empty = await readUsers(request, "search=Initial&limit=1&offset=1");
  expect(empty.status()).toBe(200);
  expect(await usersReadPage(empty)).toMatchObject({ items: [], total: 1 });

  for (const isBanned of [false, true]) {
    const filtered = await request.post("/users/read", {
      data: {
        search: "Initial",
        select: [{ login: null }],
        where_many: { is_banned: { operator: "And", values: [{ Eq: { operator: "And", values: isBanned } }] } },
        order_by: { column: { login: null }, order: "ascending" },
        pagination: { limit: 1, offset: 0 }
      }
    });
    expect(filtered.status()).toBe(200);
    const selected = await usersReadPage(filtered);
    expect(selected.total).toBe(isBanned ? 0 : 1);
    expect(selected.items).toHaveLength(isBanned ? 0 : 1);
    if (!isBanned) {
      expect(selected.items[0]).toEqual({ login, role_ids: page.items[0].role_ids });
    }
  }
  expect((await request.get("/users")).status()).toBe(404);
});
