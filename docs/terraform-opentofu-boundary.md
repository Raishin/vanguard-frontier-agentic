# Terraform / OpenTofu Boundary

The IaC board is **engine-shared**: one provider (`terraform`) covers both Terraform and
OpenTofu. This page records why, where the engines actually diverge, and the rule every
specialist on the board carries.

## Why one board rather than two

A separate `opentofu` provider was considered and rejected. The engines share the
overwhelming majority of their surface — HCL, the resource and module model, state
semantics, the provider plugin protocol, and the plan/apply workflow — so a split board
would duplicate eight specialists to express a divergence set small enough to fit on this
page. Every duplicated specialist doubles the maintenance cost of a provider deprecation,
a lifecycle rule, or a policy change, and forces the router to make an engine decision
before it has made a domain decision.

Adding a provider is also not free here: it touches two schemas, the catalog validator's
`ALLOWED_PROVIDERS`, the Rust `Provider` enum the TUI deserializes with, the docs-data
taxonomy, and two hand-written provider lists. That cost is worth paying for a genuinely
distinct domain. It is not worth paying to express "the same board, different binary".

The split that does matter is the **decision right**, not the engine — which is what the
eight specialists are organized around.

## The rule every specialist carries

Every agent on this board carries this as a fixed operating rule:

> Name the engine and the version behind every version-sensitive claim: Terraform and
> OpenTofu diverge on state and plan encryption, provider registry defaults, and parts of
> the language surface, so a behaviour verified on one engine is never reported as true of
> the other without a second source.

This is the mechanism that makes a shared board safe. The alternative — two boards — moves
the engine question to routing time, where the user has to answer it before anyone has
established whether it matters for their question. Most of the time it does not.

## Divergence register

Verified 2026-08-17 against each project's own documentation. Terraform's current stable
line is v1.15 (v1.16 in beta); OpenTofu's current release is 1.12. Version numbers do not
correspond between the projects and comparing them numerically is meaningless.

| Concern | Terraform | OpenTofu | Owning specialist |
|---|---|---|---|
| State encryption at rest | No engine-level feature; depends entirely on the backend | Native state **and plan** encryption; PBKDF2, AWS KMS, GCP KMS, Azure Key Vault, OpenBao key providers; AES-GCM only production method | `terraform-state-reliability-agent` |
| Encryption key rollover | Not applicable | `fallback` block: reads try the fallback, writes always use the new method | `terraform-state-reliability-agent` |
| Default provider registry | Resolves to the HashiCorp registry | Resolves to the OpenTofu registry | `terraform-supply-chain-integrity-agent` |
| Import config generation | `-generate-config-out` supported | Supported but marked experimental, and **cannot** be combined with `for_each` on `import` blocks | `terraform-estate-reconciliation-agent` |
| Compatibility guarantee | Published v1 compatibility promises with explicit exclusions | Separate project governance and release line | `terraform-engine-compatibility-agent` |
| Migration guidance | None (does not document migration away) | Published by the receiving project; `terraform_remote_state` coupling needs care | `terraform-engine-compatibility-agent` |
| Language and function surface | Verify against HashiCorp's own function reference | Verify against OpenTofu's own function reference | `terraform-reviewer` |

The single largest divergence is **state and plan encryption**. It is the one place where
advice given for one engine is actively wrong for the other, which is why it appears as a
`HIGH` operating rule on the state specialist rather than only in this table.

## What this register is not

It is a dated snapshot, not a standing truth. Both engines add features independently, so
any entry must be re-verified against both projects' own documentation before it is relied
on. A specialist that cannot verify a claim on the engine in question labels it `unknown`
rather than carrying it across from the other engine.

## Related

- `agents/terraform/terraform-maestro-agent/README.md` — the board's routing table and thresholds
- [Execution tiers](execution-tiers.md) — the privilege model every specialist here sits in
- [Compatibility](compatibility.md) — harness support contract
