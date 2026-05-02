# Official sources

Use this reference only when you need source grounding for AWS Private CA behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/privateca/latest/userguide/
- https://docs.aws.amazon.com/privateca/latest/userguide/CT-CreateCertificate.html
- https://docs.aws.amazon.com/privateca/latest/userguide/PcaIssueCert.html
- https://docs.aws.amazon.com/privateca/latest/userguide/CT-IssueCertificate.html
- https://docs.aws.amazon.com/privateca/latest/userguide/crl-planning.html
- https://github.com/cert-manager/aws-privateca-issuer

## cert-manager AWS PCA issuer plugin

- https://cert-manager.io/docs/configuration/issuers/
- https://github.com/cert-manager/aws-privateca-issuer/blob/main/pkg/api/v1beta1/types.go

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current PCA hierarchy, IRSA trust policy, CRL reachability, RAM share scope, or live cert-manager configuration. Prefer live AWS MCP/CLI evidence or sanitized user-provided YAML for current-state claims.
