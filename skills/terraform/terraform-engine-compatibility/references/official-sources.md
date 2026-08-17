# Official Sources

Primary sources for upgrade guidance, compatibility promises, and engine migration, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/language/upgrade-guides> | HashiCorp | Per-version upgrade guidance and the current stable and beta lines | Whether a core version move is supported and what it requires | Terraform v1.15 stable; v1.16 beta | Vendor's own upgrade guidance, the only place breaking changes are enumerated per version | Release notes describe features; only this set states what an upgrade requires |
| <https://developer.hashicorp.com/terraform/language/v1-compatibility-promises> | HashiCorp | What the v1 line promises to keep working, and the explicit exclusions | Whether a minor core upgrade may be treated as low-risk, and where the promise does not reach | Terraform v1.x | Vendor's binding compatibility statement rather than a summary of it | The upgrade guides describe individual versions; only this defines the guarantee that spans them |
| <https://developer.hashicorp.com/terraform/language/files/dependency-lock> | HashiCorp | `-upgrade` and how a version constraint becomes a selected version | Which upgrade action actually changes what runs, and when it happens | Terraform v1.15 | Vendor reference for the selection mechanism an upgrade depends on | Cited here for the upgrade trigger, a different decision than the supply-chain board's verification question |
| <https://opentofu.org/docs/intro/migration/> | OpenTofu (Linux Foundation) | Migrating an existing estate to OpenTofu, and the `terraform_remote_state` caveat | Whether an engine migration is reversible, and what needs care beyond the binary swap | OpenTofu 1.12 | The receiving engine's own migration guidance | No HashiCorp source documents migration away from Terraform |
| <https://opentofu.org/docs/intro/> | OpenTofu (Linux Foundation) | OpenTofu's current release line and governance | Which engine version an engine-choice recommendation is actually about | OpenTofu 1.12 | The project's own statement of its current version and stewardship | Version and governance facts that no third party can establish authoritatively |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
