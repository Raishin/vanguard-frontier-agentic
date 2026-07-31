#!/usr/bin/env node
/**
 * Optional E2E check for the real Codex marketplace-add command.
 *
 * This test is intentionally opt-in because it runs the installed `codex` CLI
 * and may hit the network when CODEX_PLUGIN_MARKETPLACE_SOURCE is a GitHub
 * shorthand. It uses an isolated CODEX_HOME and never writes to ~/.codex.
 *
 * What it proves:
 *   - `codex plugin marketplace add <source>` exits successfully.
 *   - Codex tracks the marketplace in the isolated CODEX_HOME/config.toml.
 *   - Codex materializes the marketplace source under CODEX_HOME/.tmp/marketplaces.
 *   - The materialized marketplace contains the repo's Codex marketplace and plugin manifests.
 *
 * What it does NOT prove:
 *   - It does not prove a plugin was installed into CODEX_HOME/plugins/cache/... .
 *     OpenAI docs describe that as plugin installation through a marketplace;
 *     the current CLI command under test is marketplace-add, not plugin-install.
 *
 * Run:
 *   RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 node tests/test-codex-plugin-marketplace-install.test.mjs
 *
 * Optional override:
 *   CODEX_PLUGIN_MARKETPLACE_SOURCE=VincentChuWaiChow/vanguard-frontier-agentic@main \
 *   RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 node tests/test-codex-plugin-marketplace-install.test.mjs
 *
 * Strict cache assertion, expected to fail for marketplace-add-only on the
 * current Codex CLI unless a separate plugin install path populates cache:
 *   EXPECT_CODEX_PLUGIN_CACHE=1 RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 \
 *   node tests/test-codex-plugin-marketplace-install.test.mjs
 */

import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const enabled = process.env.RUN_CODEX_PLUGIN_MARKETPLACE_E2E === "1";
if (!enabled) {
  console.log("SKIP codex marketplace E2E; set RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 to run it");
  process.exit(0);
}

const source = process.env.CODEX_PLUGIN_MARKETPLACE_SOURCE || "VincentChuWaiChow/vanguard-frontier-agentic";
const marketplaceName = process.env.CODEX_PLUGIN_MARKETPLACE_NAME || "vanguard-frontier-agentic";
const expectPluginCache = process.env.EXPECT_CODEX_PLUGIN_CACHE === "1";
const codexHome = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-codex-home-"));

let failures = 0;
const ok = (msg) => console.log(`OK   ${msg}`);
const fail = (msg) => {
  console.log(`FAIL ${msg}`);
  failures += 1;
};

function exists(rel) {
  return fs.existsSync(path.join(codexHome, rel));
}

try {
  const result = spawnSync(
    "codex",
    ["plugin", "marketplace", "add", source],
    {
      encoding: "utf8",
      env: {
        ...process.env,
        CODEX_HOME: codexHome,
      },
      timeout: 120000,
    },
  );

  if (result.error?.code === "ENOENT") {
    console.log("SKIP codex marketplace E2E; `codex` executable not found on PATH");
    process.exit(0);
  }
  if (result.signal === "SIGTERM") {
    fail("codex marketplace add timed out after 120s");
  }
  if (result.status === 0) {
    ok(`codex plugin marketplace add ${source} exits 0`);
  } else {
    fail(`codex marketplace add exited ${result.status}; stderr=${(result.stderr || "").slice(0, 1000)}`);
  }

  const configPath = path.join(codexHome, "config.toml");
  const config = fs.existsSync(configPath) ? fs.readFileSync(configPath, "utf8") : "";
  if (config.includes(`[marketplaces.${marketplaceName}]`)) {
    ok(`config.toml tracks marketplace ${marketplaceName}`);
  } else {
    fail(`config.toml missing [marketplaces.${marketplaceName}]`);
  }

  const installedRoot = path.join(codexHome, ".tmp", "marketplaces", marketplaceName);
  if (fs.existsSync(installedRoot)) {
    ok(`marketplace source materialized at ${installedRoot}`);
  } else {
    fail(`marketplace source missing at ${installedRoot}`);
  }

  const requiredFiles = [
    `.tmp/marketplaces/${marketplaceName}/.agents/plugins/marketplace.json`,
    `.tmp/marketplaces/${marketplaceName}/plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json`,
    `.tmp/marketplaces/${marketplaceName}/plugins/cross-platform-agent-template/.codex-plugin/plugin.json`,
  ];
  for (const rel of requiredFiles) {
    if (exists(rel)) ok(`${rel} exists`);
    else fail(`${rel} missing`);
  }

  const cacheRoot = path.join(codexHome, "plugins", "cache", marketplaceName);
  if (fs.existsSync(cacheRoot)) {
    ok(`plugin cache exists at ${cacheRoot}`);
  } else if (expectPluginCache) {
    fail(`plugin cache missing at ${cacheRoot}`);
  } else {
    console.log(`INFO plugin cache not created by marketplace-add alone: ${cacheRoot}`);
  }
} finally {
  if (process.env.KEEP_CODEX_MARKETPLACE_E2E_HOME !== "1") {
    fs.rmSync(codexHome, { recursive: true, force: true });
  } else {
    console.log(`INFO kept isolated CODEX_HOME at ${codexHome}`);
  }
}

if (failures > 0) {
  console.error(`\n${failures} check(s) failed`);
  process.exit(1);
}
console.log("\nOK: codex marketplace add E2E checks passed");
