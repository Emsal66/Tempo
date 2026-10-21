import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [react()],
  test: {
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    include: ["src/**/*.test.{ts,tsx}"],
    restoreMocks: true,
    // The persistent transform cache does not notice files added to an eager
    // `import.meta.glob`, which would let the i18n source guard skip new files.
    fsModuleCache: false,
  },
});
