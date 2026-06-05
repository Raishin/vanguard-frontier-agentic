# Official Sources

## OCI documentation URLs

- https://docs.oracle.com/en-us/iaas/Content/GSG/support/list-incidents.htm
- https://docs.oracle.com/en-us/iaas/tools/oci-cli/3.48.2/oci_cli_docs/cmdref/support/incident/list.html
- https://docs.oracle.com/en-us/iaas/Content/General/Concepts/servicelimits.htm

## Current evidence notes

- Verified on 2026-06-05 against official OCI documentation and sampled read-only command-shape evidence where applicable.
- Sampled API evidence confirmed support request listing shape and sensitive support-context parameters; committed docs must not include those values.
- Documentation evidence proves documented behavior, not the user's tenant, compartments, IAM, deployed resources, limits, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP must be described as sampled configured-environment evidence.
