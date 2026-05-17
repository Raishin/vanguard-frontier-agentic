# Marketing Agents

Marketing-governance agent catalog for this marketplace.

These agents apply the repository's zero-trust, least-privilege, and
compliance-aware stance to the marketing technology surface — the consent
layer, advertising pixels, and the martech access stack. Marketing
operations hold the customer database and are a leading compliance and
breach surface; these agents review that surface as static, read-only
auditors.

## Agent tiers

| Tier | Purpose | Default access | Live mutation |
|---|---|---|---|
| Review agents | Audit marketing consent, pixel, and access-governance posture from sanitized evidence | read-only | not allowed |

## Marketing governance review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `marketing-consent-data-collection-review-agent` | Review CMP banner config, tag-manager containers, Consent Mode wiring, and cookie policy for GDPR/ePrivacy/CCPA correctness, dark patterns, and undisclosed trackers | read-only | asked for real visitor data, consent-string archives, or analytics credentials |
| `marketing-pixel-data-leakage-review-agent` | Review advertising pixels and conversion event tracking for PII leakage to ad networks, form-field auto-capture, and pixels on sensitive pages | read-only | asked for real visitor data, conversion logs, or ad-platform credentials |
| `martech-access-governance-review-agent` | Review OAuth connected apps, API keys, CRM and marketing-automation roles, and integration scopes for least-privilege violations and stale credentials | read-only | asked to collect or echo credential values, keys, tokens, or secrets |

## Operating note

- Marketing tags that fire before a consent signal collect personal data with no lawful basis — a primary GDPR/ePrivacy enforcement and CCPA class-action surface.
- Advertising pixels that capture email, phone, health, or financial data transmit it to third-party ad networks with no contract and no breach visibility — the pattern behind major HIPAA settlements and FTC Health Breach Notification Rule actions.
- Martech stacks accumulate OAuth grants, API keys, and seats faster than they deprovision them; over-scoped connectors and stale credentials are a heavily exploited SaaS breach path.
- These agents surface regulatory risk and route binding legal determinations to qualified counsel; they do not issue legal conclusions.

## Install

```bash
# Install the marketing consent and data-collection review agent
npx vfa-export-agents --platform claude-code --agents marketing-consent-data-collection-review-agent --repo .

# Install the marketing pixel data-leakage review agent
npx vfa-export-agents --platform claude-code --agents marketing-pixel-data-leakage-review-agent --repo .

# Install the martech access governance review agent
npx vfa-export-agents --platform claude-code --agents martech-access-governance-review-agent --repo .
```
