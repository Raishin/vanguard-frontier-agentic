---
name: ai-generated-frontend-code-review
description: Apply an elevated review pass to AI or LLM-generated frontend code changes, checking specifically for hallucinated framework APIs, slopsquatted or non-existent dependency names, unsanitized dynamic-HTML sinks, and missing accessibility semantics before the diff is merged.
allowed-tools: Read Grep Glob WebFetch WebSearch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: ai
---

# AI-Generated Frontend Code Review

## Purpose

AI-generated frontend code fails in ways human-authored code rarely does at the same volume: it invokes framework APIs that read as idiomatic but do not exist for the project's installed version, it proposes installing packages whose names sound plausible but are not on the real registry (a slopsquat vector an attacker can pre-register), and it reproduces visually-correct interactive markup that is semantically empty because visual training signal does not capture keyboard/ARIA behavior. A diff that compiles and looks right is not evidence any of this was checked. This skill exists to apply a specifically elevated, adversarial bar to generated diffs — treating "the code compiles" and "this looks like normal code" as exactly zero evidence of correctness or safety — rather than folding AI-origin code into a generic review pass where these failure modes go unchecked.

## When to use

Use this skill when the user asks to:

- review a pull request or diff known or suspected to be AI/LLM-generated before merge,
- verify that framework or library APIs used in generated code actually exist and behave as claimed for the project's installed version,
- check newly introduced dependency names in a lockfile/manifest diff for registry legitimacy before install is approved,
- audit generated interactive components (menus, modals, tabs, forms, comboboxes) for missing accessibility semantics.

Do not use this skill for:

- a general code-quality or architecture review with no AI-origin signal and no hallucination/slopsquat/sink/a11y concern — use the relevant framework-specific review skill instead,
- confirming a dependency is not just spelled correctly but is also free of known CVEs — that is a separate supply-chain vulnerability scan, not a name-legitimacy check,
- live penetration testing of a confirmed unsanitized sink — this is a static-review skill; escalate confirmed exploitable sinks to a live security review process.

## Context7 Documentation Protocol

- Resolve each in-scope framework/library's Context7 ID with `resolve-library-id` before accepting or rejecting any API call in the diff as real (React: `/reactjs/react.dev` or `/websites/react_dev_reference`; use the equivalent resolved ID for other frameworks in scope). Read the project's actual manifest (`package.json` and lockfile) first to pin the installed major version — an API that exists in one major version and not another is exactly the kind of claim generated code gets wrong.
- Query Context7 for the specific hook, component prop, lifecycle method, or utility function named in the diff. If Context7 returns no matching API for that name/signature, treat the call as `unverified — possible hallucination`, not as a minor style note; do not assume the API exists because it "sounds like something React/Vue/Angular would have."
- For dependency-registry legitimacy, Context7 is not authoritative — it indexes documentation, not registry existence. Ground every new-dependency check in a live registry lookup (npm registry, JSR, or the ecosystem's canonical registry) via `WebFetch`/`WebSearch`, and label the result `registry-verified` or `registry-lookup-failed / could not verify`, never `documentation-based`.
- For accessibility semantics, Context7 does not reliably index the W3C ARIA Authoring Practices Guide as a queryable library; ground every ARIA pattern claim directly in the `official_docs` URL for the APG in this skill's `metadata.json` and label it `documentation-based`.
- If Context7 is unavailable for a framework in scope, fall back to the framework's official docs URL and label every API claim `documentation-based, unverified against current release` rather than answering from training-data memory alone.

## Lean operating rules

- Treat "this compiles" and "this looks like normal, idiomatic code" as zero evidence of correctness. Every non-trivial framework/library API call newly introduced in the diff must be checked against Context7 or the framework's official docs for the project's actual installed version before it is accepted as real.
- Check every newly introduced package name in a lockfile or manifest diff against the real public registry before approving the change. A name that "sounds right" and does not resolve on the registry, or resolves to an unrelated/typosquat-shaped package, is a slopsquat risk to flag — not a typo to silently correct and move past.
- Flag every `dangerouslySetInnerHTML`, `innerHTML` assignment, `v-html`, `document.write`, or equivalent dynamic-HTML sink newly introduced in the diff. For each, trace whether the value can carry attacker- or user-controlled data and verify a sanitizer is actually applied on that path — a sink with no traced taint source is a lower-severity pattern-only note, not a confirmed finding, and the distinction must be stated explicitly.
- Check every new interactive component (menu, modal, tabs, combobox, disclosure, tooltip) against the relevant W3C ARIA Authoring Practices Guide pattern for required role, keyboard operability, and focus management. Do not assume visual/DOM-structure correctness implies semantic correctness — AI-generated markup routinely nails the visual layout while omitting `role`, `aria-*` state, or keyboard handlers entirely.
- Do not lower the review bar because the change is "just AI-generated boilerplate" or because the diff is small. Apply the same or higher scrutiny than human-authored code, precisely because these four failure modes are systematically more likely in generated output than in it.
- Load `references/hallucinated-api-verification.md` only when verifying a specific framework/library API claim in the diff.
- Load `references/slopsquat-dependency-check.md` only when the diff introduces a new package dependency.
- Load `references/generated-a11y-gap-audit.md` only when the diff introduces or modifies an interactive component.

## References

Load these only when needed:

- [Hallucinated API verification](references/hallucinated-api-verification.md) — use to check whether a framework/library API used in generated code actually exists and behaves as claimed for the project's installed version, and how to classify the result when it does not.
- [Slopsquat dependency check](references/slopsquat-dependency-check.md) — use whenever the diff introduces a new package dependency, to verify registry legitimacy and typosquat/slopsquat risk before install is approved.
- [Generated accessibility gap audit](references/generated-a11y-gap-audit.md) — use when reviewing new or modified interactive components generated by AI, to check for missing keyboard operability and ARIA semantics against the APG.

## Response minimum

Return, at minimum:

- a per-finding verdict (`hallucinated-api` / `slopsquat-risk` / `unsanitized-sink` / `a11y-gap` / `clean`) tied to a specific file and line,
- the Context7 or official-doc citation backing each API-existence claim, or an explicit `unverified — possible hallucination` label when none is found,
- the registry-verification result for every newly introduced dependency (`registry-verified` with resolved package/publisher, or `registry-lookup-failed / could not verify`),
- a prioritized fix list ordered by security/correctness severity (unsanitized sinks and hallucinated APIs before a11y gaps before style),
- an explicit statement that the review bar applied was equal to or higher than the standard human-authored-code bar, plus what remains to be manually confirmed (e.g., live exploitability of a traced sink requires a controlled security review, not static review).
