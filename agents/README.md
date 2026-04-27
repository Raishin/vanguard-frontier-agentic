# Agents

Role definitions for repeatable review, architecture, and operations work.

## Provider-Oriented Layout

```text
agents/
├── aws/
├── azure/
├── gcp/
├── oci/
├── multi-cloud/
├── security/
└── terraform/
```

## Placement Rules

- Put provider-specific agents under the matching cloud folder:
  - `agents/aws/`
  - `agents/azure/`
  - `agents/gcp/`
  - `agents/oci/`
- Put cross-cloud architecture agents under `agents/multi-cloud/`.
- Put security-domain agents that are not provider-specific under
  `agents/security/`.
- Put Terraform/IaC-review agents under `agents/terraform/` unless they are
  tightly bound to a single provider.

## Current Organization

```text
agents/
├── multi-cloud/
│   └── cloud-architect/
├── security/
│   ├── iam-reviewer/
│   └── incident-responder/
└── terraform/
    └── terraform-reviewer/
```

The `aws/`, `azure/`, `gcp/`, and `oci/` folders are reserved for incoming
provider-specific agents.

