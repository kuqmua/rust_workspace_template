# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: admin.spec.js >> admin login
- Location: admin.spec.js:2:1

# Error details

```
Error: expect(page).toHaveURL(expected) failed

Expected pattern: /\/admin\/version$/
Received string:  "http://127.0.0.1:8080/admin/sign-in"
Timeout: 5000ms

Call log:
  - Expect "toHaveURL" with timeout 5000ms
    14 × unexpected value "http://127.0.0.1:8080/admin/sign-in"

```

```yaml
- main:
  - heading "Sign in" [level=1]
  - textbox "Login": admin
  - textbox "Password": Admin-0ba13190872ed598f4d5dbcd
  - button "Sign in"
  - paragraph: server returned HTTP 401
```

# Test source

```ts
  1  | const { test, expect } = require('/usr/lib/node_modules/@playwright/test');
  2  | test('admin login', async ({ page }) => {
  3  |   page.on('console', (message) => console.log('CONSOLE', message.type(), message.text()));
  4  |   page.on('pageerror', (error) => console.log('PAGEERROR', error.stack));
  5  |   page.on('request', (request) => {
  6  |     if (request.url().includes('/auth/sign-in')) console.log('REQUEST', request.postData());
  7  |   });
  8  |   page.on('response', async (response) => {
  9  |     if (response.url().includes('/api/')) console.log('API', response.status(), response.url());
  10 |   });
  11 |   await page.goto('http://127.0.0.1:8080/admin/sign-in');
  12 |   await page.getByPlaceholder('Login').fill('admin');
  13 |   await page.getByPlaceholder('Password').fill('Admin-0ba13190872ed598f4d5dbcd');
  14 |   await page.getByRole('button', { name: 'Sign in' }).click();
  15 |   await page.waitForTimeout(2000);
  16 |   console.log('URL', page.url());
  17 |   console.log('COOKIES', (await page.context().cookies()).map((cookie) => cookie.name));
  18 |   console.log('BODY', await page.locator('body').innerText());
> 19 |   await expect(page).toHaveURL(/\/admin\/version$/);
     |                      ^ Error: expect(page).toHaveURL(expected) failed
  20 | });
  21 | 
```