# Workflow And Output

Review sequence and output contract for bundle configuration and deployment readiness assessment.

## Workflow

1. Establish the target environment (development / staging / production) and deployment mode intended.
2. Check the bundle configuration for exactly one `databricks.yml` and the required top-level keys; flag missing or renamed config files.
3. Verify deployment-mode semantics: development mode's short_name prefix and pipeline `development: true` flag, production mode's branch matching and cluster-override prohibition.
4. Confirm run-as identity: if it differs from the deploying identity, flag any non-job, non-pipeline resources as incompatible; if it is a non-admin user, flag any grants or elevated privilege.
5. Verify variables are resolved at deployment time only, not referenced at runtime; check precedence order and that required lookups (if any) are resolvable.
6. Confirm authentication is wired through OAuth or environment variables, never persisted tokens; flag any `DATABRICKS_AUTH_STORAGE=plaintext` fallback as an insecure exception.
7. Verify the Git folder flow segregates admin and user branches and that production branches are protected from direct user pushes.
8. Check that CI/CD gates prevent promotion across environments without explicit approval; confirm rollback readiness and isolation.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the target environment and deployment mode assumed.
- Structure and deployment-mode findings, with severity labels (critical / high / medium / low) and evidence basis.
- Run-as identity, variables, and authentication findings — each with a specific constraint or unsafe pattern identified.
- Git and CI/CD gate findings — each naming the specific segregation or prevention gap.
- Safe next actions and any required confirmations (target environment, deployment identity, run-as principal, rollback owner).
