import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  reporter: process.env.CI ? [["line"], ["html", { open: "never" }]] : "line",
  use: {
    baseURL: "http://127.0.0.1:18080",
    screenshot: "only-on-failure",
    trace: "retain-on-failure",
    video: "retain-on-failure"
  },
  projects: [
    {
      name: "chromium-wide",
      use: { ...devices["Desktop Chrome"] }
    }
  ],
  webServer: {
    command: "./run-server.sh",
    cwd: import.meta.dirname,
    url: "http://127.0.0.1:18080/health/live",
    reuseExistingServer: !process.env.CI,
    timeout: 180_000
  }
});
