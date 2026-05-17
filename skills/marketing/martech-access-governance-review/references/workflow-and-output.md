# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized inventories (redact every credential value — names and scopes only, never the secret itself):
- Connected-app / OAuth grant inventory for the CRM and marketing automation platform (app name, scopes granted, grant date, owner)
- API-key inventory (key name/label, the tool it authenticates, scope, rotation history — never the key value)
- CRM and marketing-automation role matrix (roles, permissions, which integrations and users hold each)
- The list of marketing tools in the stack and how each authenticates
- Offboarding process for departed employees and ended vendor engagements, if documented

If the user provides only a partial set, note which sections are absent and scope findings accordingly. If a user pastes an actual credential value, stop, tell them not to, and ask them to treat it as compromised and rotate it.

### Step 2 — Build the integration map

For each integration, record: what it is, what marketing function it performs, what data it touches, and the minimum scope that function requires. This "needed scope" baseline is what every granted scope is measured against.

### Step 3 — OAuth scope blast-radius audit

For every connected app, compare granted scope to needed scope.

```text
# HIGH — a form/survey tool granted full read-write over all CRM contacts
App: SurveyTool
Granted: crm.objects.contacts.read, crm.objects.contacts.write, crm.objects.deals.read
Needed:  crm.objects.contacts.write  (it only creates contacts from form fills)

# CORRECT — scope matches function
App: SurveyTool
Granted: crm.objects.contacts.write
```

Flag every scope granted beyond function as HIGH. The blast radius of any connected app is the data its scope can reach if that vendor is breached — and SaaS supply-chain breaches routinely pivot through exactly these grants.

### Step 4 — Credential-sharing and rotation audit

- A single API key or service account used by more than one tool or integration is HIGH — it cannot be rotated or revoked for one consumer without breaking the others, and a single leak compromises all of them.
- Any key or grant with no rotation schedule and no expiry is HIGH.
- A "personal" API key issued under an individual employee's account, rather than a dedicated integration identity, is MEDIUM — it breaks when they leave and carries their full personal permissions.

### Step 5 — Stale-grant audit

Identify grants that should no longer exist:
- Connected apps or tokens owned by departed employees.
- Grants from vendor engagements or trials that have ended.
- Credentials for tools that were decommissioned but never disconnected.
- OAuth refresh tokens that are still valid because nothing ever revoked them.

Every live stale grant is HIGH — it is access nobody is watching.

### Step 6 — Integration role audit

Check the role each integration authenticates with:
- An integration using an admin or owner role when an API-only or limited integration role exists is HIGH.
- An integration that can change other users' permissions, billing, or security settings, when it only needs to read or write records, is HIGH.

### Step 7 — Ownership and review-cadence audit

- Every connected app and integration credential must have a named human or team owner. Unowned credentials are HIGH — nobody will ever review or revoke them.
- There must be a recurring access-review cadence (for example, quarterly) covering connected apps and integration credentials. Its absence is MEDIUM.

### Step 8 — Bulk-export and credential-storage audit

- Identify which marketing seats hold full-database or bulk-export permission. This is the exfiltration path; it should be held by the few who need it. Broad distribution is MEDIUM.
- Identify where credentials are stored. Credentials in spreadsheets, shared docs, tag-manager variables, or automation-tool fields in plaintext are HIGH. They belong in a secrets manager.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<inventory provided | role matrix provided | documentation-based | inference>

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

- This is a static review. Never request, collect, store, or echo credential values, API keys, tokens, or secrets. Work from inventories of names and scopes only.
- If the user pastes a real credential, treat it as compromised: tell them, and recommend immediate rotation.
- Apply least privilege and zero-trust assumptions: every integration should hold the narrowest scope, the shortest-lived token, and a named owner.
- Never recommend revoking a grant without naming the integration it powers and the marketing workflow that breaks — propose the scoped-down replacement grant alongside the revocation.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
