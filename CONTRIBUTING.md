# Contributing

This project is curated, not crowdsourced chaos. Contributions are welcome only when they improve trust, coverage, or installability.

## Acceptance bar

Every contribution must:

- Include metadata that passes `python tests/validate-catalog.py`.
- Link to official provider documentation when making provider-specific claims.
- State supported harnesses honestly.
- Include security notes, especially for IAM, RBAC, Terraform, MCP, credentials, and production mutation.
- Avoid secrets, example real credentials, private endpoints, and customer-specific data.
- Prefer least privilege and read-only discovery before mutation.

## Asset types

- **Skills**: repeatable workflows with clear triggers and output expectations.
- **Agents**: role definitions for review, architecture, operations, or remediation.
- **Rules**: harness-specific behavioral guidance.
- **MCP references**: vetted entries for official or community MCP servers.

## Required checks

Before opening a pull request:

```bash
python tests/validate-catalog.py
python tests/validate-links.py --offline
```

For release-quality changes, also run online link validation:

```bash
python tests/validate-links.py
```

## Provenance rules

- Use `source_type: original` for assets created specifically for this repository.
- Use `source_type: adapted` when derived from another public project; cite the source and confirm license compatibility.
- Use `source_type: reference-only` for catalog entries that point to official or third-party resources without bundling their content.

## Review philosophy

Ruthless truth: cloud automation can destroy real systems. Pretty prompts are not enough. Reviewers should reject contributions that are vague, unverifiable, unsafe, or too generic to be useful.
