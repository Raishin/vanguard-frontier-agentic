# Budget Methodology

Use this reference when the user has no existing performance budget, or an existing budget is missing a device/network class, a compression form, or a percentile — establish or correct it before ranking anything against it.

## What people get wrong

The common bad assumption is:

> "Keep the bundle small" is a budget.

It is not. A budget with no number is a slogan. A number with no device/network class and no percentile is a coin flip about whose experience it describes. Per web.dev's performance-budget guidance, a usable budget names:

1. **the metric being bounded** — total JS/CSS transfer bytes, per-route transfer bytes, or a timing metric (TTI/INP) the byte budget is a proxy for,
2. **the compression form** — gzip or brotli, matching what production actually serves; raw/uncompressed bytes overstate the number users pay for, and mixing forms across comparisons invalidates the comparison,
3. **the device/network class** — a mid-tier mobile CPU on a throttled "good 4G" profile is a meaningfully different budget than an unthrottled desktop; web.dev's budget guidance frames budgets in terms of a target device class precisely because bundle-parse/execution cost scales with CPU, not just network time,
4. **the percentile** — p75 is the conventional anchor (it is also the percentile Core Web Vitals field assessment uses), not the best-case or median session.

## Establishing a budget when none exists

1. Ask (or infer from existing CI config, `lighthouserc`, `bundlesize.config.json`, or a `size-limit` entry) whether a budget already exists in any form. Do not assume none exists just because this review wasn't asked to check.
2. Anchor the budget to a route classification, not a single global number:
   - **critical path / shared entry** — loaded on every route; tightest budget, since it blocks first render everywhere.
   - **secondary route** — route-specific bundle loaded via code splitting; budget can be looser since it does not block other routes.
   - **vendor chunk** — third-party dependency weight; budget separately from application code so a dependency bump is caught distinctly from an application-code regression.
3. State the number in gzipped (or brotli, matching production) bytes, tied to p75 on the project's target device/network class. If the project has no stated target device/network class, default to a mid-tier mobile device on a throttled "good 4G" profile per web.dev's stated default budget context, and say explicitly that this default was assumed, not confirmed with the user.
4. Tie the byte budget back to a timing outcome (TTI or INP) where possible — a byte number with no timing justification is arbitrary. State the reasoning explicitly (e.g., "X KB of parse/execute cost on the target device class corresponds to roughly Y ms of main-thread blocking, which risks the INP budget").

## Correcting an existing but underspecified budget

If a budget exists but is missing a device/network class or a percentile, do not silently adopt it as sufficient. State the gap explicitly as a finding, propose the missing dimension using the defaults above, and note that the corrected budget needs sign-off before being treated as CI-enforceable — see [CI enforcement and verification](ci-enforcement-and-verification.md).

## Non-negotiables

- Do not rank an analyzer report against a budget that has no stated compression form — a byte number with unstated compression is not comparable to anything.
- Do not accept "under 250 KB" with no percentile or device class as a complete budget; treat it as a starting number that needs the missing dimensions filled in before enforcement.
- Do not invent a budget number without grounding it in either the project's existing target (stated by the user, present in CI config) or web.dev's documented default budget context — state which one was used.
