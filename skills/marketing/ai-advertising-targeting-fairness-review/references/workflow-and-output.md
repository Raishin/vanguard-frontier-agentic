# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports (replace real values with placeholders; no real user PII, no ad-account credentials, no live audience membership data):
- Ad platform audience definition export (Meta Ads Manager audience spec, Google Ads targeting layer export, DSP deal config)
- Declared AI features enabled per campaign (e.g., Advantage+ Audience, broad match, Performance Max, Target CPA, automated bidding strategy)
- Campaign vertical and ad category (housing, credit, employment, insurance, or other)
- Seed-list demographics summary if a lookalike audience is in scope (aggregate only — no individual-level data)
- Interest segment names or IDs included in the targeting stack
- Platform Special Ad Category or equivalent fairness-restriction declaration, if any

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Campaign vertical classification

Classify the campaign into a fairness-risk tier before inspecting AI features:

- **Tier 1 — Special category** (highest risk): housing/rental, mortgage/credit, employment/hiring, insurance underwriting or pricing. FHA, ECOA, and analogous EU AI Act provisions impose the strictest obligations.
- **Tier 2 — Sensitive adjacent**: health products, financial services (non-credit), legal services, political advertising. Protected-class proxies and automated decisions warrant careful scrutiny.
- **Tier 3 — General commercial**: e-commerce, SaaS, entertainment. Standard fairness hygiene applies but special-category rules do not.

Any Tier 1 campaign with AI-driven audience expansion enabled is HIGH by classification — proceed to Step 4 immediately.

### Step 3 — AI feature inventory

Enumerate every declared AI feature active on the campaign:

```text
# Example inventory table
| Feature                  | Platform | Campaign     | Opt-out available? |
|--------------------------|----------|--------------|-------------------|
| Advantage+ Audience      | Meta     | Housing_Q2   | Partial           |
| Target CPA bidding       | Google   | Credit_Lead  | Yes               |
| Broad match keywords     | Google   | Credit_Lead  | Yes               |
| Lookalike expansion L1   | Meta     | Housing_Q2   | No                |
```

For each feature, note: whether it expands beyond declared audience, what optimization signal it uses, and whether a fairness constraint or protected-category exclusion is declared.

### Step 4 — Protected-class proxy segment audit

Inspect interest and behavioral segments for protected-class proxy risk:

```text
# HIGH — health-condition proxy on insurance campaign
Interest segment: "Diabetes management apps" → infers health condition → protected under ADA, ECOA

# HIGH — national-origin proxy via language and cultural affinity targeting
Interest segment: "Spanish-language content" + "Latin music" → national origin proxy on housing campaign

# MEDIUM — general health interest segment on non-healthcare campaign
Interest segment: "Fitness & wellness" → weaker proxy; flag for review but lower confidence
```

Flag segments that reliably infer race, sex, age, national origin, familial status, disability, or religion — even when those characteristics are not named explicitly.

### Step 5 — Algorithmic disparate-impact assessment

Assess whether automated bidding or audience expansion propagates historical bias:

```text
# HIGH — lookalike seeded from historical converters, no demographic audit
Seed list: "past_mortgage_applicants_2019_2023"
Lookalike: L1% similarity expansion
Risk: If historical applicants skew by race or national origin, the lookalike inherits that skew.
Mitigation: Demographic representativeness audit of seed list required.

# HIGH — Target CPA on credit-offer campaign, conversion event = "application_submitted"
Risk: CPA optimization deprioritizes delivery to audiences with lower historical application rates,
      which may correlate with protected-class membership.
```

### Step 6 — Platform fairness-declaration check

For Meta campaigns: confirm whether a Special Ad Category (Housing, Employment, Credit) is declared. Absence on a Tier 1 campaign is HIGH — it circumvents mandatory targeting restrictions.

For Google: confirm whether Limited Ad Serving policies are acknowledged and whether sensitive-category restrictions are applied.

For DSPs: confirm whether deal-level fairness constraints (e.g., no health-condition targeting, no age exclusions) are documented.

### Step 7 — Geographic redlining check

Inspect geofencing and location exclusions for patterns that trace protected-class neighborhood boundaries:

```text
# HIGH — exclusion zone matches historic redlining district boundaries
Excluded ZIP codes: [10031, 10037, 10039] on NYC housing campaign
These ZIPs are majority-minority neighborhoods; exclusion on a housing campaign = FHA §3604 risk.
```

Compare exclusion zones against publicly available fair-lending geography if the artifact suggests geographic selectivity.

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<audience spec provided | AI feature declaration provided | documentation-based | inference>

## Campaign tier
<Tier 1 special-category | Tier 2 sensitive adjacent | Tier 3 general commercial>

## AI feature inventory
<table of features, platform, campaign, opt-out status>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security and scope notes

- This is a static review. Never request live campaign credentials, ad-account access tokens, real audience membership lists, or individual-level conversion data.
- A finding here may constitute a fair lending, fair housing, or EU AI Act compliance violation — flag that possibility and route legal determination to qualified counsel and compliance teams. Do not make the legal determination yourself.
- Algorithmic disparate impact is a legal theory that can apply even when no protected characteristic is named — proxy targeting and optimized delivery on skewed seed populations are within scope.
- Hashing or pseudonymizing a seed list does not eliminate the disparate-impact risk from a demographically unrepresentative seed population.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
- Do not recommend disabling AI features without naming the performance impact and a manual targeting alternative.
