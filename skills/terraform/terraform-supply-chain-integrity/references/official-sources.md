# Official Sources

Primary sources for source addressing, lock files, registry protocol, and installation configuration, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/language/files/dependency-lock> | HashiCorp | Lock file contents, `h1:` versus `zh:` hash schemes, and `-upgrade` behaviour | Whether the lock file actually constrains what gets installed, on every platform that runs it | Terraform v1.15 | Vendor reference for the only artifact that pins provider identity | The single source defining the two hash schemes and what each can verify |
| <https://developer.hashicorp.com/terraform/cli/commands/providers/lock> | HashiCorp | Pre-populating lock hashes for multiple platforms | How to close a cross-platform verification gap between a developer machine and CI | Terraform v1.15 | Vendor reference for the command that fixes the gap the lock-file page identifies | The lock-file page names the problem; only this page documents the remedy |
| <https://developer.hashicorp.com/terraform/language/providers/requirements> | HashiCorp | `required_providers`, source addresses, and namespace resolution | Whether a provider resolves to the namespace the author intended | Terraform v1.15 | Vendor reference for how a provider name becomes a registry address | Addressing semantics that the lock-file page assumes rather than defines |
| <https://developer.hashicorp.com/terraform/cli/config/config-file> | HashiCorp | `provider_installation`, filesystem and network mirrors, and `dev_overrides` | Whether an installation path bypasses registry verification, and whether a developer override could reach CI | Terraform v1.15 | Vendor reference for the configuration that can silently redirect every provider install | The only source covering installation redirection, which no dependency declaration reveals |
| <https://developer.hashicorp.com/terraform/internals/provider-registry-protocol> | HashiCorp | Registry protocol, package discovery, and signature metadata | What a registry actually attests to, and what it does not | Terraform v1.15 | Vendor protocol specification rather than a description of it | Establishes the limits of registry trust that the user-facing pages do not state |
| <https://opentofu.org/docs/language/providers/requirements/> | OpenTofu (Linux Foundation) | OpenTofu provider requirements and default registry namespace | Which registry an unqualified provider name resolves to on each engine | OpenTofu 1.12 | The engine's own reference for its default resolution behaviour | The engines resolve to different default registries; this divergence exists in no HashiCorp page |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
