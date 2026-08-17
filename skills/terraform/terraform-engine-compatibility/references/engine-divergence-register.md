# Terraform And OpenTofu Divergence Register

What actually differs between the engines, stated as evidence for a decision rather than as advocacy.

- The engines share the overwhelming majority of their surface — HCL, the resource and module model, state semantics, the provider protocol, and the plan and apply workflow — which is why a shared board with an engine-naming rule is more accurate than two parallel boards.
- OpenTofu supports native encryption of state and plan files at rest, with several key providers and an explicit fallback mechanism for key rollover; Terraform has no engine-level equivalent, so a Terraform estate's state confidentiality rests entirely on the backend.
- The engines resolve unqualified or legacy provider references to different default registries, which means the same configuration text can install different packages depending on which engine ran it — a migration concern that is invisible in the configuration.
- Configurations coupled by `terraform_remote_state` need care during a migration because a consumer reads a producer's state; migrating them independently and in the wrong order leaves a consumer reading state the other engine wrote.
- Feature parity is directional and moves over time: both engines add features independently, so a divergence register is a dated snapshot that must be re-verified against both engines' own documentation rather than remembered.
- Terraform's current stable line is v1.15 with v1.16 in beta, and OpenTofu's current release is 1.12; version numbers do not correspond between the projects and comparing them numerically is meaningless.
- An engine decision has licensing, procurement, and vendor-relationship dimensions that are not compatibility questions; the technical assessment should state what each engine supports for this estate and stop there, leaving the rest to the named human owner.
- Migration guidance is published by the receiving project, so the authoritative statement of what a migration requires comes from OpenTofu rather than from HashiCorp — and the specific supported starting versions must be read from that guidance rather than assumed.
