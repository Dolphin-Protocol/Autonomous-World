import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";

export default defineConfig({
  base: "/Autonomous-World/",
  plugins: [wasm()],
});
