# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide the following as raw pasted text (these are public files; no credentials required):
- The full content of `ads.txt` from the publisher domain root (e.g., `example.com/ads.txt`)
- The full content of `app-ads.txt` if the publisher has mobile app inventory
- The relevant excerpt or full content of the exchange's `sellers.json` endpoint response — at minimum the entries corresponding to the publisher's account IDs
- The list of exchanges and resellers the publisher has active relationships with, to identify stale or missing entries
- SupplyChain Object node declarations if the user has access to bid-stream samples or SSP configuration exports

If only ads.txt is provided without sellers.json, note that RESELLER-to-sellers.json consistency findings are inference only.

### Step 2 — ads.txt structural audit

Parse each line of the ads.txt file. Each valid entry has the format:
```
<exchange domain>, <publisher account ID>, <relationship>, <certification authority ID>
```
Where `<relationship>` is `DIRECT` or `RESELLER`.

Check for:
- Malformed lines (missing fields, incorrect field count, invalid relationship value).
- Duplicate entries (same exchange domain + account ID combination appearing more than once).
- Entries using IP addresses instead of domain names (not permitted by the spec).
- Entries missing the certification authority ID (optional per spec, but flag absence as informational).

```text
# MALFORMED — missing account ID field
openx.com, RESELLER

# DUPLICATE — same entry appears twice
appnexus.com, 12345, DIRECT, f08c47fec0942fa0
appnexus.com, 12345, DIRECT, f08c47fec0942fa0
```

### Step 3 — RESELLER-to-sellers.json consistency audit

For every RESELLER entry in ads.txt, cross-reference with the corresponding exchange's sellers.json:
- Does the account ID appear in the exchange's sellers.json?
- If present, does the `seller_type` match the expected value (PUBLISHER, INTERMEDIARY, or BOTH)?
- Is the seller domain in sellers.json consistent with the publisher's domain?

```text
# HIGH — RESELLER entry with no sellers.json disclosure
ads.txt: rubicon.com, 98765, RESELLER, 0bfd66d529a55807
sellers.json (rubicon.com): account ID 98765 — not found
→ unauthorized intermediary; buyer cannot verify the resale relationship

# CORRECT — RESELLER disclosed in sellers.json
ads.txt: rubicon.com, 11111, RESELLER, 0bfd66d529a55807
sellers.json: { "seller_id": "11111", "name": "Example Publisher", "seller_type": "PUBLISHER", "domain": "example.com" }
```

Each RESELLER entry absent from sellers.json is a separate HIGH finding with the exchange domain and account ID as the finding identifier.

### Step 4 — DIRECT entry confidentiality conflict audit

For every DIRECT entry in ads.txt, cross-reference with sellers.json:
- Is the account ID present in sellers.json?
- Does the entry carry `is_confidential: 1`?

A DIRECT entry by definition declares that the exchange sells the publisher's inventory with no intermediary. A `is_confidential: 1` flag in sellers.json means the exchange is hiding the seller's identity from buyers. This is contradictory and constitutes a domain-spoofing risk vector.

```text
# HIGH — DIRECT entry resolves as confidential in sellers.json
ads.txt: exchange.com, 55555, DIRECT, abc123
sellers.json: { "seller_id": "55555", "is_confidential": 1 }
→ DIRECT relationship cannot be verified; domain-spoofing risk

# CORRECT — DIRECT entry with transparent seller identity
ads.txt: exchange.com, 55555, DIRECT, abc123
sellers.json: { "seller_id": "55555", "name": "Example Publisher", "domain": "example.com", "is_confidential": 0 }
```

### Step 5 — Orphaned account ID audit

Identify ads.txt entries where the account ID does not appear in the exchange's sellers.json at all (neither disclosed nor confidential):
- This may indicate stale entries from a terminated exchange relationship.
- It may also indicate domain spoofing: a bad actor adds a publisher's domain to their own exchange account without authorization.
- Flag as HIGH; request the user confirm whether the exchange relationship is active.

### Step 6 — Absent ads.txt audit for whitelisted domains

If the user provides a list of domains they have whitelisted for programmatic buying:
- For each domain, confirm whether ads.txt is present.
- A whitelisted domain with no ads.txt is categorically IVT-exposed: the domain has not declared any authorized seller, meaning all inventory sourced from that domain bypasses supply-chain controls.
- Flag each absent ads.txt as HIGH with the domain name.

```text
# HIGH — whitelisted domain with no ads.txt
Domain: news-publisher.example.com
ads.txt: not found (HTTP 404 / file absent)
→ all inventory from this domain is unverifiable; IVT-exposed per MRC guidelines
```

### Step 7 — SupplyChain Object completeness audit

If the user provides SupplyChain Object node declarations from bid-stream samples or SSP configuration:
- Verify `complete: 1` is set — a value of 0 means the chain is declared incomplete, which MRC and most DSPs treat as an IVT signal.
- For each intermediate node, verify `asi` (SSP domain), `sid` (account ID at that SSP), and `rid` (request ID, recommended) are present.
- Flag missing required fields in intermediate nodes as MEDIUM.
- Flag `complete: 0` as HIGH when it is set intentionally — it effectively declares the supply chain is unverifiable.

### Step 8 — Stale declaration assessment

Without a dated changelog, stale detection is inference:
- If the user discloses that exchange relationships have changed in the past twelve months, flag stale declaration as MEDIUM.
- Note that stale RESELLER entries may retain account IDs from terminated relationships that could be reused by unauthorized parties.

### Step 9 — app-ads.txt coverage gap

If the publisher has mobile app inventory:
- Confirm app-ads.txt is present at the developer domain (as declared in the app store listing).
- Absence is MEDIUM — mobile app inventory without app-ads.txt is outside IAB Tech Lab supply-chain protection.

### Step 10 — Produce the output

Format findings using the Output format section below.

---

## Output format

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<ads.txt provided | sellers.json provided | app-ads.txt provided | documentation-based | inference from absent file>

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

- This is a static review. ads.txt, app-ads.txt, and sellers.json are public files. Never request DSP credentials, exchange account tokens, bid-stream logs, or revenue reports containing publisher financial data.
- An unauthorized RESELLER entry may represent a legitimate old relationship that was not removed when the exchange contract ended, or it may represent an active unauthorized intermediary taking arbitrage margin. The distinction requires the publisher to confirm the exchange relationship status — do not assume malicious intent, but flag the opacity clearly.
- Domain spoofing via DIRECT-to-confidential conflicts is a known fraud vector documented in MRC Invalid Traffic Detection guidelines. Surface the risk explicitly without overstating certainty about active fraud.
- When evidence is partial (e.g., ads.txt provided but no sellers.json), scope findings to inference and state the assumption explicitly.
- Do not recommend removing a RESELLER entry without first confirming whether it represents a legitimate revenue path that can be replaced with a DIRECT relationship or a disclosed intermediary — revenue loss from removing legitimate paths is a real operational risk.
