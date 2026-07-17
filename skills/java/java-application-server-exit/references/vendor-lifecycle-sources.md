# Vendor Lifecycle Sources for Application-Server and JDK Exit Decisions

> Fail-closed on dates. No end-of-support, end-of-life, premier/extended-support, or license-boundary date is reproduced from memory anywhere in this reference. This file records *which primary source* governs each platform and *how to read it*; it does not assert a current date. Before citing a lifecycle date to a user, open the cited vendor page and read the current value with a read-on date. If you cannot verify it, mark it `unknown (needs vendor page)` and require the user to supply or confirm it. A stale date here produces a confidently-wrong multi-year, multi-million-dollar exit recommendation — the exact failure this reference exists to prevent.

## Why this decision matters

A replatform-vs-renew call turns partly on "how much runway does this platform have before support or license cost changes." That runway is vendor-specific and program-specific: Oracle WebLogic, IBM WebSphere, and Red Hat JBoss EAP each publish their own lifecycle policy, each policy has multiple tiers (e.g. premier vs. extended/sustaining support, or a rolling supported-versions window), and the tiers carry different cost and risk implications. Treating "the app server is old" as sufficient evidence is not defensible in front of a board; the tier and its date, read from the vendor's own page on the day of review, is.

## Primary sources (open these; do not trust memory)

| Platform | Authoritative page |
|---|---|
| Oracle WebLogic Server | `https://www.oracle.com/middleware/weblogic/` (product/lifecycle entry point) together with Oracle's Lifetime Support Policy documentation for the Fusion Middleware / WebLogic product line |
| Oracle JDK / Oracle Java SE | `https://www.oracle.com/java/technologies/java-se-support-roadmap.html` — the JDK lifecycle finding is owned technically by `java-jdk-lifecycle-and-upgrade-agent`; this agent consumes that agent's cited output rather than re-deriving it, but if a lifecycle claim reaches this agent directly, verify it against this same page |
| IBM WebSphere Application Server (traditional and Liberty) | `https://www.ibm.com/support/pages/lifecycle` — IBM's software lifecycle lookup tool; search the specific product and version |
| Red Hat JBoss EAP (and predecessor JBoss AS) | `https://access.redhat.com/support/policy/updates/jboss_notes` — Red Hat's JBoss product update and support policy notes |
| Jakarta EE namespace / specification versions | `https://jakarta.ee/about/faq/` — governs which `javax.*` vs `jakarta.*` namespace and which Jakarta EE major version a given application-server release implements; relevant when the jakarta-namespace-debt specialist finding needs a namespace-to-platform-version cross-reference |
| Cross-vendor overview (secondary, corroborating only) | `https://endoflife.date/` — useful as a lead for which product page to open next; never cite it as the authority, and never in place of the vendor page above |

## How to read a lifecycle boundary

1. **Support tier, not just "supported/unsupported."** Oracle, IBM, and Red Hat each use multi-tier models (for example: full/premier support, then extended or sustaining support at higher cost or reduced scope, then end of support). The tier the estate is currently in — not just whether a date has passed — determines both risk and near-term cost trajectory.
2. **Version-specific, not product-family-wide.** "WebLogic" or "WebSphere" is not a single lifecycle; a specific major/minor version is. Confirm the version in scope (from the specialist container-readiness or inventory findings) before opening the vendor page.
3. **License and support are separate questions.** A platform can be inside its support window while its license/subscription cost tier has changed, or vice versa. This reference governs the *support/lifecycle* question; the *dollar* question is always a user-supplied figure per `decision-model-and-cost-inputs.md` — never inferred from the lifecycle tier.

## Boundary table (structure — fill from the cited page at review time)

| Component | Platform | Version | Current support tier | Tier-change or EOL date | Source page | Read on |
|---|---|---|---|---|---|---|
| _(e.g. claims-portal)_ | _(e.g. WebLogic)_ | _(e.g. 12.2.1.4)_ | _(verify)_ | _(verify — do not guess)_ | _(vendor URL)_ | _(YYYY-MM-DD)_ |

If a cell cannot be verified against the primary source during the review, write `unknown (needs vendor page)` — never a remembered value, and never a value inferred from a different version of the same product.

## Known uncertainty

- Vendor lifecycle and support policies are revised, and vendors occasionally extend or shorten a tier for a specific version. Treat this file's structure as durable; treat any date as a snapshot requiring re-verification.
- Oracle's support-tier terminology and the specific policy documents it points to have changed across "Lifetime Support Policy" revisions; confirm you are reading the current version of the policy document for the specific product line (Fusion Middleware vs. Java SE are governed separately).
- IBM's lifecycle lookup tool covers many products under one search interface; confirm the exact product name and edition (WebSphere Application Server traditional vs. Liberty are different lifecycles).
- Red Hat's JBoss EAP policy notes page format and specific supported-version windows change between EAP major releases; confirm you are reading the section for the version in scope.

## Refresh protocol

- **Refresh owner:** the Java board maintainer (`github: Raishin`) or a named delegate.
- **Cadence:** re-verify any boundary cited in an active review against its primary source at the time of that review — this reference does not cache dates. Re-read this file's source-page list at least quarterly for URL drift (vendors restructure support pages).
- **On refresh:** update `last_verified` in the skill's `metadata.json` when the source-page list itself changes; record the read-on date in any table cell touched during a review.
- This engine never consults the wall clock — behavior changes only when a committed reference changes or a user supplies a verified date. Do not encode "as of today" logic.

## Escalation conditions

- A JDK-specific lifecycle question arises in isolation (no application-server platform decision attached) → hand it to `java-jdk-lifecycle-and-upgrade-agent`; that agent owns the JDK technical support-boundary finding, this agent consumes its output.
- A required lifecycle date cannot be verified against any primary source → return `unknown` and ask the user to supply the vendor page and read-on date; do not proceed to an "exit now" or "safe to renew" conclusion on a guessed date.
- The lifecycle tier is verified but its dollar impact (renewal quote, extended-support surcharge, migration labor cost) is unknown → that is a cost input, not a lifecycle fact; see `decision-model-and-cost-inputs.md` and do not estimate it here.
