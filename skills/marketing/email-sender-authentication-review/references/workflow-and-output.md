# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide the following as a sanitized DNS record export (replace real selector names with generic placeholders only if the user prefers; SPF/DKIM/DMARC records are public data but never request ESP credentials or DMARC aggregate XML):
- SPF TXT record for the root sending domain and all active ESP subdomains
- DKIM TXT record(s) identified by selector name (e.g., `selector1._domainkey.example.com IN TXT "v=DKIM1; k=rsa; p=..."`)
- DMARC TXT record at `_dmarc.example.com`
- BIMI TXT record at `default._bimi.example.com` and VMC/CMC certificate URL if present
- The list of all active ESP and transactional sending paths (e.g., Mailchimp, Salesforce Marketing Cloud, SendGrid transactional, Postmark) and whether each uses a subdomain or the root domain

If the user provides only partial records, note which paths are unassessed.

### Step 2 — SPF audit

Parse the SPF record from `v=spf1` through the terminating `all` mechanism:

1. Count every mechanism that requires a DNS lookup: `include:`, `a`, `mx`, `ptr`, `exists`. RFC 7208 mandates a hard limit of ten such lookups; exceeding it produces a permerror treated as an SPF fail by receiving MTAs.
2. Identify the `all` qualifier: `~all` (softfail), `-all` (hardfail), `+all` (pass all — HIGH), `?all` (neutral).
3. Identify any mechanisms that are redundant, deprecated (`ptr:`), or that enumerate IP ranges far wider than the actual sending infrastructure.

```text
# HIGH — SPF with +all negates all restrictions
v=spf1 include:esp1.com include:esp2.com +all

# HIGH — SPF with 13 DNS lookups; permerror on receipt
v=spf1 include:_spf.google.com include:sendgrid.net include:mail.zendesk.com
        include:servers.mcsv.net include:spf.mailjet.com include:_spf.salesforce.com
        include:postmarkapp.com include:emailsig.com include:mktomail.com
        include:smtp.hubspot.net include:spf1.mailchimp.com include:esp12.com
        include:sp.example.com ~all
# (13 include: mechanisms, each resolves to at least one more lookup → permerror)

# CORRECT — SPF with eight lookups and -all
v=spf1 include:_spf.google.com include:sendgrid.net include:postmarkapp.com -all
```

### Step 3 — DKIM audit

For each active sending path identified in Step 1:
- Confirm a DKIM selector exists and the TXT record is present and well-formed (`v=DKIM1`, key type, public key).
- Confirm the key length is at least 1024 bits; 2048 bits is recommended.
- Confirm the signing domain (`d=` tag in the DKIM signature) aligns with the `From:` domain at the level required by the DMARC alignment mode (relaxed: organizational domain match; strict: exact domain match).
- Flag any sending path with no DKIM selector as HIGH.
- Flag keys shorter than 1024 bits as HIGH (deprecated, breakable).
- Note whether key rotation documentation was provided; absence is MEDIUM.

```text
# HIGH — transactional ESP subdomain has no DKIM selector
tx.example.com: no DKIM TXT record found for any known selector
DMARC alignment for mail sent via tx.example.com: fails (no signature to align)

# CORRECT — selector and key present, 2048-bit key
selector2._domainkey.example.com IN TXT "v=DKIM1; k=rsa; p=MIIBIjANBgkqh..."
```

### Step 4 — DMARC audit

Parse the DMARC record at `_dmarc.<domain>`:
- `p=` (policy): `none`, `quarantine`, or `reject`. `none` provides monitoring only; it does not prevent spoofing or satisfy Google/Yahoo bulk-sender enforcement requirements when operating at scale.
- `pct=` (percentage): defaults to 100; values below 100 mean the policy applies to only that fraction of non-aligning mail.
- `rua=` (aggregate report URI): absence means no visibility into alignment failures.
- `ruf=` (forensic report URI): optional but useful for debugging.
- `aspf=` and `adkim=` (alignment modes): `r` (relaxed, default) or `s` (strict); strict requires an exact domain match between the `From:` header and the SPF/DKIM signing domain.
- `sp=` (subdomain policy): defaults to the `p=` value if absent; explicit `sp=reject` is recommended when subdomains are not used for sending.

```text
# HIGH — p=none with no enforcement path
_dmarc.example.com IN TXT "v=DMARC1; p=none; rua=mailto:dmarc@example.com"
→ spoofing is possible; Google/Yahoo bulk-sender requirements not satisfied for enforcement

# MEDIUM — p=quarantine with pct=10 and no ruf
_dmarc.example.com IN TXT "v=DMARC1; p=quarantine; pct=10; rua=mailto:dmarc@example.com"
→ only 10% of failing mail is quarantined; 90% is unaffected

# CORRECT — p=reject, full enforcement, reporting configured
_dmarc.example.com IN TXT "v=DMARC1; p=reject; pct=100; rua=mailto:dmarc@example.com; ruf=mailto:forensic@example.com"
```

### Step 5 — DMARC alignment verification

DMARC requires at least one of SPF or DKIM to align with the `From:` header domain:
- For SPF alignment: the envelope `MAIL FROM` domain must match the `From:` header domain at the configured alignment level.
- For DKIM alignment: the `d=` tag in the DKIM signature must match the `From:` header domain at the configured level.
- If neither SPF nor DKIM aligns, DMARC fails regardless of `p=` value — flag as HIGH if structural misalignment is evident from the record set.

### Step 6 — BIMI and certificate audit

If a BIMI record is present at `default._bimi.<domain>`:
- Confirm `v=BIMI1; l=<logo-url>; a=<certificate-url>` syntax.
- Confirm the certificate URL resolves to a VMC (Verified Mark Certificate) or CMC (Common Mark Certificate).
- Without a VMC/CMC, BIMI display is ignored by Gmail, Yahoo, and Apple Mail — flag as LOW.
- If no BIMI record is present, note it as informational (not a deficiency unless the user has a BIMI adoption goal).

### Step 7 — Bulk-sender compliance assessment

Assess compliance with Google and Yahoo bulk-sender requirements (enforced Feb 2024 for Google, June 2024 for Yahoo):
- DMARC record present at organizational domain level: required.
- SPF or DKIM alignment passing: required.
- Spam complaint rate below 0.10% (0.08% recommended): not assessable from DNS records alone — note as out-of-scope.
- One-click unsubscribe (RFC 8058 `List-Unsubscribe-Post` header): not assessable from DNS records — note as out-of-scope.

Summarize the DNS-assessable compliance gap clearly.

### Step 8 — Produce the output

Format findings using the Output format section below.

---

## Output format

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<DNS record provided | documentation-based | inference from absent record>

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

- This is a static review. DNS records are public, but never request ESP account credentials, DMARC aggregate report XML containing real email metadata, or sending-platform API keys.
- A domain at `p=none` is exploitable for spoofing attacks and phishing campaigns impersonating the brand. Surface this risk explicitly; do not understate it as a deliverability issue only.
- When evidence is partial (e.g., SPF record provided but no DKIM selectors listed), scope each finding to what was provided and state the inference basis explicitly.
- Do not recommend removing an active ESP's SPF `include:` to solve the lookup-count problem without first confirming DKIM-only alignment is available for that path — removing SPF coverage without DKIM will break DMARC alignment.
- Key rotation guidance is advisory hygiene; the urgency depends on key age and organizational risk tolerance; surface it as MEDIUM, not blocking.
