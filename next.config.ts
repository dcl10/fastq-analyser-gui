import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Tauri ships a static frontend (no Node server), so produce a static
  // export that `tauri.conf.json`'s frontendDist can point at.
  output: "export",
  // Don't let `next dev`/`next build` inject its own AGENTS.md block — this
  // repo keeps a hand-written AGENTS.md identical to CLAUDE.md.
  agentRules: false,
};

export default nextConfig;
