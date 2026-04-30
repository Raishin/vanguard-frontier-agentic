# Workflow and output contract

1. Confirm the Terraform root, module, workspace assumptions, and backend context.
2. Patch the smallest repo-side Terraform change needed.
3. Run the strongest safe local validation available, such as `terraform fmt -check`, `terraform validate`, or deterministic parsers.
4. Do not cross the line into live apply or direct state mutation unless a separate guarded-live step is explicitly requested.
5. Return a compact report with changed files, validation, rollback notes, and remaining risks.

## Output shape

1. Verdict
2. Changed files or planned edits
3. Validation results
4. Rollback notes
5. Open risks
