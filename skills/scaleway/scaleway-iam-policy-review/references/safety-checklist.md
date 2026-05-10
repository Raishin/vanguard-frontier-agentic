# Safety checklist

Use this reference before privileged, compliance-impacting, or production-affecting IAM recommendations.

## Non-negotiables

- Never ask users to paste `SCW_ACCESS_KEY`, `SCW_SECRET_KEY`, raw key values, project IDs, or organization IDs into chat.
- Prefer sanitized Terraform resource definitions or sanitized CLI/console output. Do not work from memory or invented policy state.
- Do not invent permission set names, policy rule structures, API key scopes, or Scaleway IAM defaults.
- Require explicit user approval before recommending revocation of a key or policy that is actively in use.
- Flag organization-level scope as high-risk before any other finding — it grants access to ALL projects.
- Flag API keys with no expiry as high-risk. Recommend expiry even for keys believed to be low-privilege.
- Use Context7 or official Scaleway IAM documentation for current permission set behavior when the answer depends on Scaleway service details.

## Stress checks

- Which keys can expose data or perform cross-project actions due to org-level scope?
- Which permission sets grant delete, update, or admin access beyond what the workload requires?
- Which key revocations could break a running workload if applied immediately?
- What compliance or audit evidence is missing (e.g., rotation records, key ownership)?
- What rollback path exists if a permission set change locks out a legitimate workload?

## Evidence labels

Use `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Scaleway IAM state. Findings labeled `inference` must be clearly marked as unconfirmed.
