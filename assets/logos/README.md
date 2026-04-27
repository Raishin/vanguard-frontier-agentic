# Logo Assets

Repo-local logo assets for documentation, catalogs, cards, and generated
marketplace materials.

## Structure

```text
assets/logos/
├── cloud/
│   ├── aws/
│   │   ├── aws-cdnlogo.png
│   │   ├── aws-cdnlogo.svg
│   │   ├── aws-legacy-brandlogos.png
│   │   └── aws-legacy-brandlogos.svg
│   ├── azure/
│   │   └── azure.png
│   └── oci/
│       ├── oracle-cloud-infrastructure.png
│       └── oracle-cloud-infrastructure.svg
└── vendors/
    └── oracle/
        ├── oracle.png
        └── oracle.svg
```

## Category Rules

- `cloud/<provider>/` — cloud platform or service logos, such as OCI, AWS,
  Azure, and Google Cloud.
- `vendors/<vendor>/` — company/vendor logos, such as Oracle, Microsoft,
  Amazon, Google, HashiCorp, and Kubernetes project owners.
- Prefer SVG for docs and generated cards when possible.
- Keep PNG fallbacks for tools that do not render SVG reliably.
- Use lowercase, hyphenated filenames.
- Prefix ambiguous provider logos with the provider slug, for example
  `aws-cdnlogo.svg`.
- Prefer stable Markdown-friendly names: no spaces, uppercase letters,
  underscores, parentheses, source-site prefixes, or random download suffixes.
- Preserve source files elsewhere; this folder contains curated copies for this
  repository.

## Markdown Examples

```markdown
![AWS logo](assets/logos/cloud/aws/aws-cdnlogo.svg)
![Azure logo](assets/logos/cloud/azure/azure.png)
![OCI logo](assets/logos/cloud/oci/oracle-cloud-infrastructure.svg)
![Oracle logo](assets/logos/vendors/oracle/oracle.svg)
```
