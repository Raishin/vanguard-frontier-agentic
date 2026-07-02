# Legal Exposure and Severity Model

Use this reference when triaging a finding's litigation risk, prioritizing a remediation backlog under limited engineering capacity, or preparing input for a VPAT / accessibility conformance statement where severity and exposure need to be defensible, not vibes-based.

## What people get wrong

The naive story is:

> "All WCAG failures are equally 'an accessibility bug' — triage by engineering effort, not by criterion."

Wrong for two reasons. First, WCAG Level A is the floor of any recognized conformance claim (ADA Title II/III, Section 508, EN 301 549, AODA — all build on WCAG A/AA as the referenced technical standard); an unresolved Level A failure undermines the entire conformance claim, not just one page. Second, litigation pattern data (publicly tracked ADA web-accessibility lawsuit filings) skews heavily toward a small, repeatable set of criteria — the same handful of failure types recur across the large majority of filed complaints and demand letters. Triaging purely by "how hard is this to fix" ignores which failures are actually driving legal exposure.

## Non-negotiable severity framing

Every finding gets exactly one severity, derived from two independent axes — never blend them into a single fuzzy number:

1. **Conformance Level** (A > AA > AAA) — Level A failures are always at least as severe as an otherwise-similar AA/AAA failure, because A is the mandatory floor.
2. **Litigation-pattern exposure** (elevated / standard) — independent of Level, based on whether the SC is one of the criteria that recurs disproportionately in filed accessibility litigation.

## Elevated-exposure success criteria

Flag findings against these SC as **elevated** regardless of how minor the fix looks, because they are the pattern most frequently cited in demand letters and filed complaints:

| SC | Level | Why it recurs in litigation |
|---|---|---|
| 1.1.1 Non-text Content | A | Missing alt text on functional images (logos-as-links, icon buttons) is trivially detectable by a plaintiff's automated pre-filing scan |
| 1.4.3 Contrast (Minimum) | AA | Mechanically measurable — plaintiffs' own tooling flags it identically to defense tooling, leaving little room to dispute |
| 2.1.1 / 2.1.2 Keyboard / No Keyboard Trap | A | "Could not complete purchase/signup using only a keyboard" is a common, concrete injury narrative |
| 2.4.4 Link Purpose (In Context) | A | "Click here" / "read more" links with no accessible context are common and easy to demonstrate |
| 2.4.7 Focus Visible / 2.4.11 Focus Not Obscured | AA / AA | Directly demonstrable via screen recording of keyboard navigation |
| 3.3.2 Labels or Instructions | A | Unlabeled form fields on checkout/signup flows are a recurring named injury in filings |
| 4.1.2 Name, Role, Value | A | Custom widgets (fake buttons, unlabeled icon controls) with no accessible name/role are the most common root cause cited alongside 2.1.1 |

This list is a documented-pattern heuristic, not a legal opinion — verify current litigation-pattern data and consult qualified counsel before using it to size actual legal risk for a specific organization.

## Severity matrix

| Level | Elevated exposure | Standard exposure |
|---|---|---|
| A | **Critical** — blocks conformance floor + high litigation pattern match | **High** — blocks conformance floor |
| AA | **High** — typical target level + high litigation pattern match | **Medium** — typical target level |
| AAA | **Medium** (only if AAA is an explicit stated target) | **Low** (AAA is rarely a required conformance target — do not imply blocking severity unless the user has stated AAA is their target) |

## Minimal safe triage flow

1. Identify the SC and Level (from `references/wcag22-sc-index.md`).
2. Determine evidence tier — automated/semi-automated/manual (from `references/act-rules-detection-boundary.md`).
3. Check whether the SC appears in the elevated-exposure table above.
4. Assign severity from the matrix.
5. State the evidence tier and severity together in the same finding line — never report severity without also stating how confidently the finding was established.
6. For a remediation backlog, sort Critical > High > Medium > Low, then within each tier prefer the SC that unblocks the largest number of user flows (e.g. a global focus-visible regression outranks a single-page contrast issue even at the same nominal severity).

## When to push back

Push back if the user asks to:

- deprioritize a Level A finding because "it's a small visual thing" — Level A is the conformance floor regardless of perceived visual magnitude,
- treat this severity model as a legal risk quantification or substitute for counsel — it is a documented-pattern triage heuristic for engineering prioritization, not a legal opinion, and must not be presented as one,
- suppress or soften an elevated-exposure finding in a report because it's inconvenient for a release timeline — the exposure flag exists precisely so it cannot be silently dropped from the audit trail.
