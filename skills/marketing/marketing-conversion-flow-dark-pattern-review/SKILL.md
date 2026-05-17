---
name: marketing-conversion-flow-dark-pattern-review
description: Use this skill when reviewing marketing conversion flow specifications — subscription sign-up, upsell interstitial, free-trial enrollment, and cancellation path — for dark-pattern practices that invalidate consent or constitute unfair or deceptive acts under FTC Section 5 and state privacy laws. Trigger when a user provides a UX flow specification including step-by-step page descriptions, annotated wireframes, CTA labels, pre-checked options, visual weight of accept vs decline paths, countdown timer specs, or cancellation flow step counts. Scope is limited to marketing conversion flows; consent banner review is handled by a separate skill.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Marketing Conversion Flow Dark-Pattern Review

## Purpose
This skill reviews marketing conversion flow specifications — subscription sign-up, upsell interstitials, free-trial enrollment, and cancellation paths — for dark-pattern practices that invalidate consent or constitute unfair or deceptive acts under FTC Section 5, the FTC Negative Option Rule (ROSCA), the CPRA statutory dark-pattern definition (§ 1798.140(l)), and EU AI Act Article 5(1)(b). Dark patterns in conversion flows are a distinct and high-priority regulatory surface: pre-checked auto-renew boxes, asymmetric cancel vs. subscribe step counts, artificial countdown timers, and visually suppressed decline paths have drawn FTC enforcement, FTC rules with click-to-cancel mandates, and CPRA enforcement advisories. This skill works from a sanitized UX flow specification or annotated wireframe only. It does not review consent banners — that is the domain of `marketing-consent-data-collection-review`.

## Lean operating rules
- Treat a free-trial or subscription enrollment flow that pre-checks "auto-renew at full price" (or any material recurring-charge term) as HIGH — pre-checked consent for a recurring financial commitment is prohibited under the FTC Negative Option Rule and invalidates consent under CPRA § 1798.140(l).
- Treat any cancellation path that requires more steps than the enrollment path, or that interposes save-offers between the cancel intent and the cancel confirmation without a direct-cancel alternative, as HIGH — the FTC Negative Option Rule and ROSCA require cancellation to be at least as easy as enrollment.
- Treat an artificial countdown timer applied to an offer with no real deadline as HIGH — it creates false urgency, a deceptive act under FTC Act Section 5.
- Treat visual suppression of the decline path (smaller font, lower contrast, grey-out, positioning below the fold, or absence of a visible "no thanks" option) as HIGH when paired with a visually dominant accept CTA — asymmetric visual weight subverts user autonomy under CPRA § 1798.140(l) and constitutes a deceptive format under FTC Section 5.
- Treat upsell interstitials that make the "continue without upgrade" option absent, invisible, or materially harder to reach than the upgrade CTA as HIGH — the absence of a clear decline path on a mandatory interstitial eliminates meaningful consent.
- Treat a subscription sign-up flow in which material price, renewal date, and cancellation method are not disclosed clearly and conspicuously before billing information is collected as HIGH — ROSCA requires pre-billing disclosure of all material terms.
- Flag "confirm-shaming" CTA copy (e.g. "No thanks, I don't want to save money") as MEDIUM — it applies social pressure but may not alone constitute an unfair act; combined with visual suppression it escalates.
- Flag any save-offer sequence on a cancellation path that does not offer a direct cancel option at each step as MEDIUM — save offers are permissible but must not be the only route.
- Flag countdown timers whose real deadline is authenticated by server state (session-scoped) as LOW — distinguish from artificial timers which are HIGH.
- Do not recommend removing a conversion step without naming the revenue or data-collection impact and an FTC-compliant alternative.
- Label every finding with evidence basis: flow specification provided, wireframe provided, documentation-based, or inference from missing element.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Pre-checked consent assessment (recurring-charge terms, auto-renew)
- Cancellation path symmetry assessment (step count vs. enrollment path)
- Countdown timer authenticity assessment
- Visual weight and decline-path accessibility assessment
- Upsell interstitial consent assessment
- Material-term pre-billing disclosure assessment
- Confirm-shaming CTA assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
