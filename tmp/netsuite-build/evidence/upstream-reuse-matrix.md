# Upstream Reuse Matrix — Oracle SuiteCloud Agent Skills
<!-- Generated: 2026-06-09 | Source: live evidence from GitHub + Oracle docs -->

## Source Repository

- **Repo:** https://github.com/oracle/netsuite-suitecloud-sdk
- **Skills path:** `packages/agent-skills/`
- **License:** Universal Permissive License (UPL), Version 1.0
- **Copyright:** © 2019, 2023 Oracle and/or its affiliates
- **Official Oracle docs:** https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_4123813814.html
- **Announcement date:** April 28, 2026 (SuiteConnect San Francisco)
- **NetSuite release alignment:** 2026.1 (4 new skills); initial 3 launched March 27, 2026

## License Summary

UPL-1.0 is highly permissive (BSD/MIT class):

- **Attribution required:** Yes — copyright notice `"Copyright (c) 2019, 2023 Oracle and/or its affiliates"` must be retained in all copies or substantial portions.
- **Modification permitted:** Yes, including derivative works and commercial use.
- **Distribution permitted:** Yes, with attribution.
- **Copyleft:** None. No share-alike obligation.
- **Warranty:** Disclaimed. Users assume all risk.
- **Bottom line:** ADAPTED_WRAPPER and DEPENDENCY patterns are both clean under UPL-1.0, provided the copyright header is preserved in any redistributed or adapted file.

---

## Upstream Skills Verified (8 of 8 — all confirmed live as of 2026-06-09)

> The 7 skills explicitly requested are covered in the table below. One additional skill (`netsuite-finance-analyst`) was discovered in the same directory and is included.

---

| Upstream Skill | Official URL | License | What it covers | Maps to which of our agents | Reuse Type | Added value if adapted |
|---|---|---|---|---|---|---|
| **netsuite-ai-connector-instructions** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-ai-connector-instructions | UPL-1.0 | Guardrails for AI-to-NetSuite sessions via the NetSuite AI Service Connector MCP. Covers tool-selection decision tree (Reports → Saved Searches → Record Ops → Custom SuiteQL), SuiteQL safety checklist (ROWNUM limits, NVL wrapping, metadata pre-verification), output formatting standards (currency abbreviations, hyperlinked transactions), multi-subsidiary/multi-currency scoping, GL accounting domain knowledge, and security SafeWords principles. | `netsuite-ai-connector-mcp-agent` (primary); secondarily informs `netsuite-web-services-integration-agent` for safe query patterns | **DEPENDENCY** — load as-is alongside the MCP agent | Our MCP agent adds Vanguard harness routing, tool-call logging, and retry logic not present upstream. The upstream skill governs *what* the agent does with NetSuite; our wrapper governs *how* the agent is invoked and monitored. |
| **netsuite-owasp-secure-coding** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-owasp-secure-coding | UPL-1.0 | OWASP Top 10 (2021) mapped to SuiteScript 2.1 and JavaScript. 48 catalogued pitfalls (OSCP-001 through OSCP-048) with BAD/GOOD examples and severity ratings. Covers injection (SuiteQL parameterization, LDAP escaping), output encoding for 5 HTML contexts, CSP construction, file upload/download pipelines, API and RESTlet hardening, CSRF, DOM XSS, postMessage origin validation, AI prompt-injection mitigations, and a mandatory security review checklist. Auto-activates on security-relevant keywords. | `netsuite-suitescript-secure-code-review-agent` (primary); also relevant to `netsuite-suitecloud-developer-agent` and `netsuite-application-developer-agent` | **ADAPTED_WRAPPER** — extend with Vanguard-specific gate thresholds | Add: (1) mapping OSCP pitfall IDs to Vanguard severity taxonomy, (2) block/warn/allow decision gates for CI pipeline integration, (3) reporting format for audit evidence artifacts. |
| **netsuite-sdf-project-documentation** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-sdf-project-documentation | UPL-1.0 | Generates enterprise documentation from SuiteScript and SDF object XML files. Outputs: README.md, ARCHITECTURE.md, API.md, CHANGELOG.md. Produces Mermaid + ASCII diagrams of component relationships and data flows. Documents deployment details, script configurations, URLs, triggers, role permissions, integration points, and troubleshooting tables. Includes post-deployment automation hooks for continuous documentation. Redacts secrets/PII from generated docs. | `netsuite-sdf-devops-release-agent` (primary); also feeds `netsuite-suitecloud-developer-agent` | **ADAPTED_WRAPPER** | Add: (1) integration with our catalog metadata schema so generated docs auto-populate agent manifest fields, (2) CI gate that blocks release if ARCHITECTURE.md is missing or stale, (3) alignment to our docs/_data/catalog.yml Liquid variable conventions. |
| **netsuite-sdf-roles-and-permissions** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-sdf-roles-and-permissions | UPL-1.0 | Authoritative resolution of NetSuite permission configurations in SDF. Contains a validated catalog of 684 permission codes (ADMI_, LIST_, REGT_, REPO_, TRAN_ prefixes and custom record permissions). Covers least-privilege role design, integration role patterns, script run-as configurations, `customrole` XML validation, `permkey`/`permlevel` assignment, and use-case inference for common scenarios (sales orders, invoices, customers). Warns against Administrator role use for scripts. | `netsuite-identity-access-role-permission-agent` (primary); `netsuite-sdf-devops-release-agent` (permission validation during release) | **DEPENDENCY** — reference the upstream catalog directly | Our identity agent adds cross-agent RBAC context (who in the Vanguard harness can invoke which NetSuite operations), SSO/SAML mapping for NetSuite roles, and zero-trust attestation logging not present upstream. |
| **netsuite-suitescript-records-reference** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-suitescript-records-reference | UPL-1.0 | Authoritative field lookup across 272 NetSuite record types. Per field: internal ID, data type (text/currency/date/etc.), required status, `submitFields()` compatibility, search filter and column availability, custom field support, client/server scriptability flags. Sourced from Oracle's official SuiteScript Records Browser. | `netsuite-application-developer-agent` (primary); `netsuite-suitecloud-developer-agent`; `netsuite-suitescript-secure-code-review-agent` (field type validation during review); `netsuite-saved-searches-workbook-agent` (column/filter lookup) | **DEPENDENCY** — load unchanged as a reference context | Our agents add: dynamic field discovery for custom fields not in the static catalog, cross-reference to saved search column IDs, and workspace-aware caching to avoid repeated large context loads. |
| **netsuite-suitescript-upgrade** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-suitescript-upgrade | UPL-1.0 | SuiteScript 1.0 / 2.0 / 2.x → 2.1 migration assistant. 4 modes: Analyze / Convert / Explain / Validate. 125+ API function mappings across 26 modules, 34 object class conversions with 331 method mappings, 13 unmapped APIs with documented workarounds, 10+ script type entry-point changes, 16 breaking-change categories, 15 conversion patterns with paired before/after code samples. Migration complexity scoring via 7-factor matrix. Emits pure SS2.1 without shims. | `netsuite-suitecloud-developer-agent` (primary migration workflows); `netsuite-application-developer-agent` (code modernization during feature work); `netsuite-sdf-devops-release-agent` (pre-release upgrade gate) | **ADAPTED_WRAPPER** | Add: (1) integration with our SDF devops release gate so unconverted 1.0 code blocks deployment, (2) complexity-score thresholds that trigger human-review escalation, (3) output format aligned to our CHANGELOG.md conventions via `netsuite-sdf-project-documentation`. |
| **netsuite-uif-spa-reference** | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-uif-spa-reference | UPL-1.0 | API reference for `@uif-js/core` and `@uif-js/component`. Covers: foundation classes (Date, ArrayDataSource, Ajax, Router), React-style hooks (useState, useEffect, useContext, useCallback, useMemo, useRef), state management (Store/Reducer/dispatch/selector), 277 SystemIcon + 43 RecordIcon constants, 101 KeyCode constants, event bus, and 20+ UI components (DataGrid, StackPanel, GridPanel, TabPanel, Form components, Modal, Tooltip, FilterPanel, etc.) with prop specs, constructor signatures, and real-world pitfall warnings. | `netsuite-application-developer-agent` (UIF SPA feature work); `netsuite-suitecloud-developer-agent` (Suitelet/SPA scaffolding) | **DEPENDENCY** — load as-is as a companion reference | Our agents add: Vanguard component composition patterns, design system token mapping, and accessibility audit integration (WCAG 2.1 AA gates) layered on top of the UIF API reference. |
| **netsuite-finance-analyst** *(additional — not in the 7 requested but verified in repo)* | https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills/netsuite-finance-analyst | UPL-1.0 | Director of Financial Analysis-grade skill for NetSuite financial data interpretation. Covers: financial statement analysis, variance review, month/quarter/year-end close sequencing, bank/AR/AP/intercompany reconciliations, budget-vs-actual, journal entry review, cash/liquidity/covenant reporting, SOX audit-readiness, and board/CFO narrative generation. Read-only by default; requires explicit authorization before mutations. Materiality thresholds calibrated to company scale (mid-market defaults; adjustable). | `netsuite-bi-reporting-agent` (financial narratives + executive dashboards); `netsuite-saved-searches-workbook-agent` (period-close data extraction); no direct match for a dedicated finance agent in the current board — **recommend adding one** | **ADAPTED_WRAPPER** | Add: (1) integration with our BI reporting agent's dashboard-builder output format, (2) multi-subsidiary consolidation scope controls aligned to our identity/permission agent's subsidiary mapping, (3) SOX evidence artifact generation in a format compatible with compliance audit trails. |

---

## Skills Requested But NOT Found Upstream

All 7 explicitly requested skills were confirmed present. No gaps.

| Requested Skill ID | Status |
|---|---|
| netsuite-ai-connector-instructions | FOUND — verified above |
| netsuite-owasp-secure-coding | FOUND — verified above |
| netsuite-sdf-project-documentation | FOUND — verified above |
| netsuite-sdf-roles-and-permissions | FOUND — verified above |
| netsuite-suitescript-records-reference | FOUND — verified above |
| netsuite-suitescript-upgrade | FOUND — verified above |
| netsuite-uif-spa-reference | FOUND — verified above |

---

## Reuse Type Legend

| Code | Meaning |
|---|---|
| **REFERENCE** | Cite upstream in our docs/metadata; no code dependency. |
| **DEPENDENCY** | Load upstream SKILL.md content directly at runtime; no local copy maintained. |
| **ADAPTED_WRAPPER** | Fork or wrap upstream; add Vanguard-specific logic. Must track upstream for drift. |
| **NO_ACTION** | Upstream skill overlaps nothing in our board. |

### Summary by reuse type

| Reuse Type | Count | Skills |
|---|---|---|
| DEPENDENCY | 3 | netsuite-ai-connector-instructions, netsuite-sdf-roles-and-permissions, netsuite-suitescript-records-reference, netsuite-uif-spa-reference |
| ADAPTED_WRAPPER | 4 | netsuite-owasp-secure-coding, netsuite-sdf-project-documentation, netsuite-suitescript-upgrade, netsuite-finance-analyst |
| REFERENCE | 0 | — |
| NO_ACTION | 0 | — |

*(Note: netsuite-uif-spa-reference counted in DEPENDENCY above; total unique skills = 8)*

---

## Coverage Gaps in Our Agent Board

Skills that map to our agents but have **no upstream equivalent** — these must be built from scratch:

| Our Agent | Upstream Coverage | Gap |
|---|---|---|
| `netsuite-web-services-integration-agent` | Partial (ai-connector-instructions covers SuiteQL safety only) | REST/SOAP/TBA auth patterns, REST API versioning, integration record management — no upstream skill exists |
| `netsuite-suiteflow-automation-agent` | None | SuiteFlow workflow designer, action/condition scripting, approval routing — not covered upstream |
| `netsuite-bi-reporting-agent` | Partial (finance-analyst covers narrative; no BI/pivot/chart upstream skill) | Dashboard builder, pivot table design, KPI charting — no upstream skill |
| `netsuite-saved-searches-workbook-agent` | Partial (records-reference covers field IDs; no saved-search query builder skill) | Saved search criteria syntax, workbook formulas, scheduled report delivery — no upstream skill |

---

## Sync Strategy: Preventing Adapted Wrappers from Drifting

### Problem
Four skills use ADAPTED_WRAPPER: our local copy extends upstream content. Oracle pushes updates to the `master` branch without a formal semver tag for individual skills. If we silently diverge, security pitfall counts, API mappings, and permission catalogs in our wrappers become stale.

### Recommended Approach

**1. Pin to upstream via `git subtree` or sparse checkout (preferred)**

Rather than copying SKILL.md files, pull the upstream `packages/agent-skills/` subtree into our repo at a known commit SHA:

```bash
git subtree add \
  --prefix vendor/oracle-agent-skills \
  https://github.com/oracle/netsuite-suitecloud-sdk.git master \
  --squash
```

Update on a schedule:
```bash
git subtree pull \
  --prefix vendor/oracle-agent-skills \
  https://github.com/oracle/netsuite-suitecloud-sdk.git master \
  --squash
```

Our adapted wrapper files then `include` or `extend` from `vendor/oracle-agent-skills/<skill>/SKILL.md` rather than duplicating content.

**2. Automated drift detection (CI gate)**

Add a weekly GitHub Actions workflow that:
- Fetches the latest SHA of `packages/agent-skills/` from `oracle/netsuite-suitecloud-sdk`
- Compares against the pinned SHA in `vendor/oracle-agent-skills/`
- Opens a draft PR with a diff summary if upstream has changed
- Labels the PR `upstream-sync` and assigns it to the NetSuite agent board maintainer

**3. Changelog discipline in wrapper files**

Each ADAPTED_WRAPPER skill file must carry a `<!-- upstream-sha: <sha> -->` comment at the top. The CI gate checks that this SHA matches the vendored subtree SHA. Any mismatch fails the validate gate.

**4. Attribution block (UPL-1.0 requirement)**

Every adapted file must include this header verbatim:

```
Portions of this file are derived from oracle/netsuite-suitecloud-sdk
(packages/agent-skills/<skill-name>/SKILL.md), licensed under the
Universal Permissive License (UPL), Version 1.0.
Copyright (c) 2019, 2023 Oracle and/or its affiliates.
Full license text: https://oss.oracle.com/licenses/upl
```

**5. Release alignment window**

Oracle ships NetSuite updates on a ~quarterly cadence (2026.1, 2026.2, etc.). Align our sync review to coincide with NetSuite release notes publication (typically mid-January, mid-April, mid-July, mid-October). The oracle/netsuite-suitecloud-sdk release history confirms new agent-skills batches ship with NetSuite point releases.

---

## Evidence Sources

- https://github.com/oracle/netsuite-suitecloud-sdk/tree/master/packages/agent-skills
- https://github.com/oracle/netsuite-suitecloud-sdk (license)
- https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_4123813814.html
- https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_0331043722.html
- https://www.randgroup.com/insights/oracle-netsuite/suitecloud/netsuite-suitecloud-agent-skills-what-they-are-and-how-they-help-developers/
- https://knowledgehubmedia.com/netsuite-suiteconnect-2026-new-ai-coding-agent-skills-for-suitecloud-developers/
- https://www.infoworld.com/article/4164873/oracle-netsuite-announces-ai-coding-skills-for-suitecloud-developers.html
- Individual SKILL.md files fetched directly from raw.githubusercontent.com (2026-06-09)
