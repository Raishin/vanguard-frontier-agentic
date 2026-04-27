# Security Policy

## Supported surface

This repository publishes agentic workflows and references. Treat every asset as executable influence over an AI system, even when it is "just markdown."

## Never include

- Access keys, API tokens, private keys, session tokens, passwords, or `.env` values.
- Instructions to bypass approval, audit, least privilege, or change-management controls.
- Credential-harvesting workflows.
- Destructive production automation without explicit approval gates and rollback guidance.

## Reporting a vulnerability

Open a private security advisory if available, or contact the maintainers through the repository owner profile. Do not publish exploit details in a public issue.

## MCP-specific risks

MCP servers can expose tools that read or mutate cloud resources. Every MCP reference must state:

- vendor or maintainer,
- official/community status,
- authentication model,
- expected permissions,
- whether tools can mutate resources,
- local/remote execution assumptions.

## Review standard

If a contribution gives an agent more power over cloud resources, reviewers should demand stronger provenance, least-privilege guidance, and verification evidence.
