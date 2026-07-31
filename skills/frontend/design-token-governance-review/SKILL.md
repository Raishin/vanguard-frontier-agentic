---
name: design-token-governance-review
description: Reviews design-token source of truth and build pipelines for hardcoded-value drift and resolved WCAG 1.4.3/1.4.11 contrast compliance across theme variants (light, dark, high-contrast), grounded in the W3C Design Tokens format and current WCAG success criteria.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compliance
---

# Design Token Governance Review

## Purpose

A token system only delivers its promise (consistent, themeable, accessible UI) if it is actually the single source of truth. This skill catches the two most common failure modes: hardcoded values that duplicate an existing token (silent drift) and token pairings that resolve to an inaccessible contrast ratio in some theme variant that nobody re-checked after the token value changed.

## When to use

Use this skill when the user asks to:

- review a design-token source file or build/transform pipeline (Style Dictionary, Tokens Studio, W3C DTCG format),
- audit whether hardcoded colors/spacing values have crept back into components,
- verify WCAG 1.4.3 (text contrast) or 1.4.11 (non-text/UI-component contrast) compliance for token pairings,
- review a proposed rebrand, dark-mode, or high-contrast-mode token change for contrast regressions.

## Context7 Documentation Protocol

Before making any claim about token-file format, `$value`/`$type` semantics, reference resolution, transform behavior, or Storybook a11y tooling, ground the claim in current library docs:

1. Call `mcp__Context7__resolve-library-id` for the library in scope (`Style Dictionary`, `Storybook`) if not already resolved in this session.
2. Call `mcp__Context7__query-docs` with a specific query before asserting version-specific behavior (transform config shape, DTCG `$type` inheritance rules, a11y-addon detection coverage, etc.).
3. Verified library IDs for this skill's scope: Style Dictionary `/style-dictionary/style-dictionary` (DTCG format, `$value`/`$type`, `resolveReferences`/`getReferences`, `typeDtcgDelegate` for `$type` inheritance); Storybook `/storybookjs/storybook` (`@storybook/addon-a11y`, built on axe-core, documented as catching up to ~57% of WCAG issues automatically — never represent an addon-a11y pass as full conformance).
4. If Context7 has no coverage for a claim, fall back to the W3C Design Tokens Community Group spec (`https://tr.designtokens.org/format/`) or WCAG 2.1 normative text, and mark the claim `documentation-based` rather than `verified`.
5. Never invent a transform name, CLI flag, or token property that was not confirmed via Context7 or the W3C spec.

## Lean operating rules

- Always compute the resolved contrast ratio from the actual token values in scope; a token's semantic name (`--color-text-secondary`) does not guarantee its computed contrast is compliant.
- Check every theme variant (light, dark, high-contrast) independently; a token pairing compliant in light mode can fail in dark mode with no code-level warning.
- Distinguish a documented, intentional design-system escape hatch (a one-off spacing value explicitly allowed by governance docs) from undocumented drift; only the latter is a finding.
- Apply WCAG 1.4.3 (4.5:1 normal text / 3:1 large text) to text-on-background token pairings and WCAG 1.4.11 (3:1) to UI-component and graphical-object contrast (borders, focus indicators, icons) — these are different success criteria with different thresholds; do not conflate them.
- In DTCG-format token files, resolve `{group.token}` references and `$type` group-level inheritance before evaluating a value — a token's effective type/value can come from an ancestor group, not the token's own object.
- Require that generated token build output (CSS custom properties, JS token modules) be committed with a reviewable diff against source tokens, not silently regenerated; a build-only change with no source-token diff is itself a governance finding.
- Treat a Storybook `@storybook/addon-a11y` pass as partial automated evidence only (axe-core-class coverage, not full conformance); it does not substitute for manually verifying contrast math on token pairings the user is asking about.
- Load the `visual-regression-storybook-review` skill instead of this one when the question is about pixel-diff baseline approval rather than token source/contrast math.
- This is a static-review skill: read and analyze token source files, build configs, and resolved output; do not run builds, mutate token files, or execute pipeline commands.

## References

Load these only when needed:

- [Token source and pipeline review](references/token-source-and-pipeline-review.md) — use when reviewing DTCG/Style Dictionary token structure, reference resolution, `$type` inheritance, transform/build config, or hunting for hardcoded-value drift against an existing token set.
- [Contrast compliance review](references/contrast-compliance-review.md) — use when computing or verifying resolved contrast ratios for token pairings against WCAG 1.4.3/1.4.11 across theme variants.

## Response minimum

Return, at minimum:

- inventory of hardcoded-value findings with file/line and the matching existing token,
- computed contrast ratio per token pairing reviewed, per theme variant, with WCAG pass/fail against the correct success criterion (1.4.3 vs 1.4.11),
- evidence level (computed from provided token values vs inferred, and Context7-grounded vs documentation-based),
- recommended governance mechanism (lint rule/CI check) to prevent recurrence,
- security caveat if the token pipeline pulls from an external API.
