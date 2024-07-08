import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  base: "/gpi-v2/",
  plugins: [react({
    fastRefresh: process.env.NODE_ENV !== 'test'
  })],
  css: {
    modules: {
      localsConvention: "camelCaseOnly",
    },
  },
});
