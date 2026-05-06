#!/usr/bin/env node
/**
 * Tests that export-marketplace-agents.mjs emits harness-specific
 * skill-skip notices for Cursor and Kiro (no native skill primitive)
 * and does NOT create any skill directories for those platforms.
 *
 * RED → GREEN driven by the SKIP_SKILLS_PLATFORM_NOTICES map added to
 * the exporter. See docs/cross-harness-skills.md §§ Cursor, Kiro.
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  ".."
);
const exporter = path.join(repoRoot, "scripts", "export-marketplace-agents.mjs");

/** Stable agent fixture present in every checkout. */
const TEST_AGENT = "aws-agentcore-agent";

function runExport(platform, tmpRepo) {
  const r = spawnSync(
    process.execPath,
    [exporter, "--platform", platform, "--agents", TEST_AGENT, "--repo", tmpRepo],
    { encoding: "utf8" }
  );
  return { stdout: r.stdout ?? "", stderr: r.stderr ?? "", exitCode: r.status ?? 0 };
}

function withTmpDir(fn) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-"));
  try {
    fn(dir);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

// ── Cursor ────────────────────────────────────────────────────────────────────

withTmpDir((tmpRepo) => {
  const { stderr } = runExport("cursor", tmpRepo);

  // Must contain cursor-specific language referencing Project Rules
  assert.ok(
    stderr.includes("Project Rules"),
    `[cursor] Expected stderr to mention "Project Rules". Got:\n${stderr}`
  );

  // No .cursor/skills/ or .cursor/rules/ skill directory should be created
  const cursorSkillsDir = path.join(tmpRepo, ".cursor", "skills");
  const cursorRulesDir = path.join(tmpRepo, ".cursor", "rules");
  assert.ok(
    !fs.existsSync(cursorSkillsDir),
    `[cursor] Unexpected .cursor/skills/ directory was created`
  );
  assert.ok(
    !fs.existsSync(cursorRulesDir),
    `[cursor] Unexpected .cursor/rules/ directory was created`
  );

  console.log("PASS  cursor: harness-specific notice + no skill directory created");
});

// ── Kiro (all aliases) ────────────────────────────────────────────────────────

for (const platform of ["kiro", "kiro-ide", "kiro-cli"]) {
  withTmpDir((tmpRepo) => {
    const { stderr } = runExport(platform, tmpRepo);

    // Must contain kiro-specific language referencing Steering
    assert.ok(
      stderr.includes("Steering"),
      `[${platform}] Expected stderr to mention "Steering". Got:\n${stderr}`
    );

    // No .kiro/steering/ skill directory should be created
    const kiroSteeringDir = path.join(tmpRepo, ".kiro", "steering");
    assert.ok(
      !fs.existsSync(kiroSteeringDir),
      `[${platform}] Unexpected .kiro/steering/ directory was created`
    );

    console.log(`PASS  ${platform}: harness-specific notice + no skill directory created`);
  });
}
