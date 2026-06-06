# Safety checklist

Use before Azure workload security reviews, hardening recommendations, policy/Defender changes, identity changes, network exposure decisions, incident-response claims, or production-readiness statements.

## Non-negotiables

- Do not ask for or print credentials, tokens, secrets, tenant IDs, subscription IDs, resource IDs, customer data, private keys, or raw incident payloads.
- Keep the default mode read-only and advisory.
- Require explicit approval before changing Entra ID, Conditional Access, RBAC, PIM, Azure Policy, Defender plans, Sentinel connectors, network controls, Key Vault settings, or production diagnostics.
- Prefer managed identities and least privilege over stored secrets and broad standing privilege.
- Separate Microsoft Learn documentation evidence from sampled read-only Azure evidence and sanitized user evidence.
- Do not call a workload secure unless evidence covers identity, segmentation, data, secrets, logging, threat detection, vulnerability management, and incident response.
- Treat paid-plan, preview, manual, and shared-responsibility controls as caveated evidence.

## Component risks

- **Identity and access:** broad Owner/Contributor grants, weak Conditional Access, standing privilege, unmanaged workload identities, stale service principals.
- **Segmentation and networking:** public ingress, uncontrolled egress, flat networks, missing Private Link DNS review, NSG/firewall rules without owners.
- **Data protection:** unclassified data, unmanaged keys, missing audit trails, untested backup/recovery security, exfiltration paths.
- **Secrets:** secret values in code, app settings, pipelines, logs, tickets, or chat; missing rotation and emergency rotation.
- **Policy and compliance:** broad deny effects without safe deployment, stale exemptions, compliance dashboards used as proof without control review.
- **Threat monitoring:** unowned alerts, missing diagnostic settings, weak log retention, disconnected SecOps process.
- **DevSecOps:** no threat model, no SAST/dependency/IaC/image scanning, findings without SLA or release gate.
- **Incident response:** paper runbooks, unclear owner, no tabletop, no recovery security validation.

## Evidence labels

Use `documentation-based`, `sampled current-state evidence`, `repo evidence`, `user-provided evidence`, or `inference`. Documentation alone never proves the user's live Azure posture.
