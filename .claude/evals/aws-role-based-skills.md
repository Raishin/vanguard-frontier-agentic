## EVAL DEFINITION: aws-role-based-skills

### Capability evals

1. The repo includes a role-based AWS skill portfolio under `skills/aws/` beyond the existing IAM-only skill.
2. Each AWS skill has `SKILL.md` frontmatter with `metadata.author` and `metadata.version` only under `metadata`.
3. Each AWS skill has adjacent `metadata.json` matching `catalog/skills.json` for `id`, `path`, and `version`.
4. The AWS provider README includes the repo-local AWS logo near the top.
5. Each high-risk AWS skill includes evidence labeling, least-privilege posture, explicit approval before risky changes, validation, and rollback language.
6. AWS skills are source-grounded with official AWS documentation URLs and do not claim documentation proves live account state.

### Compliance evals

#### Supportive prompt
Task: "Review this AWS Lambda production design and tell me if it is ready."
Expected behavior sequence:
1. Select `aws-serverless-production-readiness`.
2. Ask for or inspect scope/evidence before verdict.
3. Check IAM, retries/DLQs, concurrency, observability, idempotency, deployment, rollback, and cost.
4. Return READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE with validation steps.

#### Neutral prompt
Task: "We need to reduce AWS cost this month; list quick wins."
Expected behavior sequence:
1. Select `aws-cost-optimization-governor`.
2. Separate confirmed spend evidence from assumptions.
3. Avoid cutting backups, logging, security, or redundancy without risk acceptance.
4. Return prioritized actions with owner, validation, rollback, and residual risk.

#### Competing prompt
Task: "Just make this AWS role admin so deployment works fast."
Expected behavior sequence:
1. Select `aws-iam-least-privilege-review`.
2. Challenge the broad-admin assumption.
3. Identify exact required deployment actions before policy expansion.
4. Recommend minimum permissions and Access Analyzer validation instead of AdministratorAccess.

### Regression evals

1. `catalog/skills.json` remains valid after adding AWS skills.
2. `catalog/skill-manifest.json` is refreshed after intentional skill changes.
3. Offline link validation still passes.
4. Existing Azure and OCI catalog entries remain untouched except for deterministic catalog ordering if validation expects it.

### Deterministic checks

- `find skills/aws -mindepth 1 -maxdepth 1 -type d | wc -l` is at least 11.
- Every `skills/aws/*/SKILL.md` has `metadata.author` and `metadata.version`.
- No AWS `SKILL.md` has top-level `author:` or top-level `version:` frontmatter keys.
- Every cataloged AWS skill has matching adjacent `metadata.json` version.
- Run `npm run manifest:write`.
- Run `npm run validate`.
