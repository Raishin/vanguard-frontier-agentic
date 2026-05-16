"use strict";

const SESSION_URL_RE = /https?:\/\/claude\.ai\/code\/session_\S+/g;

function cleanBody(body) {
  if (!body) return body;
  return body
    .replace(SESSION_URL_RE, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim() || undefined;
}

module.exports = {
  branches: ["master"],
  plugins: [
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "conventionalcommits",
        releaseRules: [
          { type: "security", release: "patch" },
          { type: "perf", release: "patch" },
          { type: "refactor", release: "patch" },
          { type: "build", release: "patch" },
          { type: "revert", release: "patch" },
          { scope: "no-release", release: false },
        ],
      },
    ],
    [
      "@semantic-release/release-notes-generator",
      {
        preset: "conventionalcommits",
        presetConfig: {
          types: [
            { type: "feat", section: "✨ Features" },
            { type: "fix", section: "🐛 Bug Fixes" },
            { type: "security", section: "🔒 Security" },
            { type: "perf", section: "⚡ Performance" },
            { type: "refactor", section: "♻️ Refactor" },
            { type: "docs", section: "📚 Documentation" },
            { type: "build", section: "📦 Build" },
            { type: "revert", section: "⏪ Reverts" },
            { type: "test", hidden: true },
            { type: "ci", hidden: true },
            { type: "chore", hidden: true },
            { type: "style", hidden: true },
          ],
        },
        writerOpts: {
          headerPartial:
            "## 🛡️ v{{version}} — *Provenance, Policy, Portability* &mdash; {{date}}\n\n" +
            "> _Multi-cloud agent marketplace · `AWS` · `Azure` · `OCI` · `Terraform`_\n" +
            ">\n" +
            "> Built for operators on the cloud frontier — least privilege, live evidence, safe rollback paths.\n\n",
          commitPartial:
            "*{{#if scope}} **{{scope}}:**{{~/if}} {{subject}}{{#if shortHash}} ({{shortHash}}){{/if}}\n" +
            "{{#if body}}\n{{body}}\n{{/if}}\n",
          // conventional-changelog-writer v8 freezes the commit object,
          // so mutating fields directly throws "Cannot modify immutable
          // object". Return a shallow copy with the cleaned body instead.
          transform(commit) {
            return { ...commit, body: cleanBody(commit.body) };
          },
        },
      },
    ],
    ["@semantic-release/changelog", { changelogFile: "CHANGELOG.md" }],
    [
      "@semantic-release/exec",
      {
        // Runs AFTER changelog writes and BEFORE npm packs the tarball.
        // Synchronizes plugin manifest versions and the cross-asset
        // integrity manifest to the bumped package.json so the released
        // tarball is internally consistent. See scripts/release-prepare.mjs.
        prepareCmd: "node scripts/release-prepare.mjs ${nextRelease.version}",
      },
    ],
    "@semantic-release/npm",
    [
      "@semantic-release/github",
      {
        successComment: false,
        failComment: false,
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: [
          "CHANGELOG.md",
          "package.json",
          // Synchronized by scripts/release-prepare.mjs during prepare.
          // package.json is the single source of truth; all versioned files
          // below are derived from it and committed together in the release
          // commit so the next diff stays clean and the attested tarball
          // matches the committed tree.
          ".claude-plugin/plugin.json",
          ".cursor-plugin/plugin.json",
          "plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json",
          ".github/plugin/marketplace.json",
          "SECURITY.md",
          "catalog/asset-integrity.json",
        ],
        message: "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}",
      },
    ],
  ],
};
