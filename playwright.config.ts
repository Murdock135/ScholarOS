import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "tests/ui",
  workers: 1,
  use: {
    baseURL: "http://127.0.0.1:1420",
    headless: true,
    viewport: { width: 1280, height: 820 },
  },
  webServer: [
    {
      command: "cargo run -p scholar-core --example ui_test_server",
      url: "http://127.0.0.1:4319",
      timeout: 120000,
    },
    {
      command: "npm run dev -- --mode test",
      url: "http://127.0.0.1:1420",
      timeout: 30000,
    },
  ],
  reporter: "list",
});
