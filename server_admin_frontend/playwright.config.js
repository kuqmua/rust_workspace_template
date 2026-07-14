const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: './tests',
  outputDir: '.playwright-results',
  fullyParallel: false,
  reporter: 'list',
  use: {
    baseURL: 'http://127.0.0.1:8081',
    headless: true,
  },
  webServer: {
    command: 'node tests/fixture-server.js',
    url: 'http://127.0.0.1:8081/admin/users',
    reuseExistingServer: false,
  },
});
