import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import type { IncomingMessage } from "http";

// Les chemins /dashboard, /users, /logs, /alerts sont À LA FOIS des routes
// React (pages du SPA) et des endpoints API. On ne relaie vers le backend
// QUE les appels fetch (Accept: application/json / */*). Les navigations
// navigateur (Accept: text/html) sont laissées au SPA via `bypass`.
// N.B. : proxy = serveur de dev uniquement ; `vite build` (prod) l'ignore.
const isBrowserNavigation = (req: IncomingMessage) =>
  (req.headers.accept ?? "").includes("text/html");

const backend = (extra: object = {}) => ({
  target: "http://localhost:3000",
  changeOrigin: true,
  bypass: (req: IncomingMessage) =>
    isBrowserNavigation(req) ? req.url : undefined,
  ...extra,
});

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/auth": backend(),
      "/logs": backend(),
      "/alerts": backend(),
      "/dashboard": backend(),
      "/users": backend(),
    },
  },
});
