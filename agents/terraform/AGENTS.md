# AGENTS.md

## Purpose
- Store Terraform marketplace agents with clear separation between advisory review, repo-write execution, and guarded live operation.

## Rules
- Keep Terraform advisory, repo-write, and live-operation roles separate; do not blur plan review into apply authority.
- Keep Markdown harness adapters flush-left after frontmatter.
- Keep Codex TOML flat and valid.
- Treat backend, state, lock, and workspace semantics as first-class concerns, not footnotes.
- Do not normalize `terraform apply`, `terraform destroy`, `-auto-approve`, `-lock=false`, or `force-unlock` without explicit user intent and role-appropriate guardrails.
- Run `npm run manifest:write` and `npm run validate` after Terraform catalog changes.
