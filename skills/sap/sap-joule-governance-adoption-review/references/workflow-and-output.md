# Workflow and output contract — SAP Joule Governance and Adoption Review

Use this reference for all finding classification, risk level assignment, remediation path selection, and output formatting.

## Joule governance domain taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `scope-grounding` | `unapproved-skill-activation` | Joule skill or action enabled for a connected SAP solution that has not been through the organization's Joule rollout approval process |
| `scope-grounding` | `write-back-without-confirmation` | Joule action skill that creates, updates, or deletes records in an underlying SAP system without an explicit user confirmation step |
| `scope-grounding` | `overly-broad-capability-scope` | Joule capabilities activated for a user population broader than required — end users with access to skills intended for managers or administrators |
| `data-access-boundary` | `authorization-bypass-risk` | Joule can surface data that the user is not authorized to see in the underlying SAP application — authorization model not correctly enforced in Joule context |
| `data-access-boundary` | `cross-application-unreviewed-aggregation` | Joule aggregates data across multiple connected SAP systems for a user without a governance review of the user's cross-system access rights |
| `data-access-boundary` | `sensitive-data-in-joule-response` | Joule responses include sensitive business data (compensation, financial positions, procurement prices, personal data) visible to users who are not authorized for that data in the underlying system |
| `auditability` | `no-interaction-logging` | Joule interaction logging not enabled for a deployment in a regulated or compliance-sensitive business process |
| `auditability` | `log-access-not-restricted` | Joule interaction logs are accessible without role restriction — any user or administrator can read logs containing other users' prompt content |
| `auditability` | `insufficient-log-retention` | Joule interaction log retention period does not meet the organization's audit or regulatory requirements |
| `role-aware-config` | `role-context-not-configured` | Joule role-aware context configuration has not been set up for one or more connected SAP solutions — Joule may not tailor responses to the user's business role |
| `role-aware-config` | `role-misconfiguration-over-disclosure` | Incorrect role-aware context configuration causes Joule to surface data appropriate for a more privileged role than the user holds |
| `hallucination-risk` | `regulated-workflow-no-verification` | Joule is used in a financial, HR, legal, or procurement decision workflow without a documented human verification requirement before actioning the output |
| `hallucination-risk` | `no-source-attribution` | Joule responses for compliance-relevant queries do not include source attribution or grounding data citation |
| `hallucination-risk` | `no-acceptable-use-training` | End users have not received training on Joule output validation — over-trust risk not mitigated through user awareness |
| `change-management` | `no-acceptable-use-policy` | No documented acceptable-use policy exists defining permitted and prohibited uses of Joule, output validation requirements, and escalation paths |
| `change-management` | `no-rollout-governance-process` | No documented process exists for approving and activating additional Joule capabilities beyond the initial deployment |
| `change-management` | `no-feedback-monitoring-mechanism` | No feedback or monitoring mechanism exists to detect Joule misuse, over-reliance, or systematic output quality issues post-deployment |

## Risk classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Confirmed Joule data access boundary breach: Joule surfaces data the user is not authorized to see in the underlying SAP system; cross-application aggregation exposes regulated personal data without authorization |
| `high` | Write-back skill without confirmation step; cross-application unreviewed data aggregation; no interaction logging for a regulated business process; regulated workflow with no human verification requirement for Joule outputs; role misconfiguration causing over-disclosure of sensitive business data |
| `medium` | Missing acceptable-use policy; untested role-aware configuration; insufficient log retention; no feedback and monitoring mechanism; no source attribution for compliance-relevant responses |
| `low` | Best practice deviation in rollout governance materials, adoption documentation, or monitoring coverage without immediate risk |

## Remediation path decision tree

For each finding:

1. **Is this a confirmed Joule data access boundary breach (Joule surfaces unauthorized data)?**
   - Yes → `critical`. Immediately restrict the affected Joule capability or suspend the connected system integration. Escalate to the security and data protection team. Do not resume the capability until the authorization boundary is confirmed to be enforced. State this explicitly.
   - No → continue.

2. **Is this a write-back skill without an explicit confirmation step?**
   - Yes → `high`. Disable the write-back skill until a confirmation step is implemented in the Joule skill configuration or the connected SAP system workflow. Document the required confirmation UX and approval path before re-enabling.
   - No → continue.

3. **Is this a regulated workflow (financial, HR, legal, procurement) with no human verification requirement?**
   - Yes → `high`. Define and document a human verification requirement for the affected workflow. Add an explicit policy statement in the acceptable-use policy that Joule outputs in this workflow must be verified by a qualified human before actioning. Train affected users.
   - No → continue.

4. **Is this missing interaction logging for a compliance-sensitive deployment?**
   - Yes → `high`. Enable Joule interaction logging per SAP Joule administration documentation. Restrict log access to authorized roles. Define and document a log retention policy meeting the organization's audit requirements.
   - No → continue.

5. **Is this a governance process or policy gap (missing acceptable-use policy, no rollout governance, untested role configuration)?**
   - Yes → `medium`. Draft the acceptable-use policy covering permitted uses, prohibited uses, output validation requirements, and escalation paths. Define a rollout governance process for capability additions. Test role-aware configuration in a sandbox environment.
   - No → classify as `low` and provide adoption best practice guidance.

## Workflow

1. **Receive artifacts** — Joule configuration documentation, skill activation lists, data access boundary descriptions, audit log configuration summaries, acceptable-use policy documents, training material descriptions, or written governance posture descriptions.
2. **Classify each finding** by Joule governance domain and finding class above.
3. **Assign risk level** per risk classification table (critical / high / medium / low).
4. **Flag critical findings immediately** — confirmed data access boundary breaches must be escalated before other remediation is discussed.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical data boundary findings first; then high write-back, logging, and regulated workflow findings; then medium policy and configuration gaps; then low adoption best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Joule governance domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Capability or workflow detail (if applicable): Joule skill name or type, connected SAP solution, affected user population, or business process
5. Recommended governance control per finding (skill deactivation, confirmation step addition, interaction logging enablement, role-aware config correction, verification policy, acceptable-use policy, etc.)
6. Joule governance posture after remediation
7. Escalation notice for confirmed data access boundary breaches — explicit statement that security and data protection team must be engaged before resuming the affected capability
8. Prioritized remediation sequence
