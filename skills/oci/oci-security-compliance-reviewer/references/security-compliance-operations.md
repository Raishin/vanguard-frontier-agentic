# OCI Security Compliance Reviewer Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating “no findings” as secure without checking detector/target coverage.
- Approving tenancy-wide admin because compliance is urgent.
- Ignoring logging, key management, network exposure, and separation-of-duties evidence.
- Calling documentation compliance evidence without sampled current state.

## Officially grounded service shape

- Official OCI Cloud Adoption Framework documentation emphasizes security architecture, isolation, and landing-zone controls across IAM, network, logging, key management, vulnerability scanning, bastion, notifications, Security Zones, and Cloud Guard.
- Official OCI Cloud Guard documentation says missing detector recipes or target configuration can prevent problems from being detected.
- Official OCI documentation describes using Cloud Guard to detect and respond to vulnerabilities identified by Vulnerability Scanning Service.
- OCI API evidence through the user’s configured read-only OCI MCP shows Cloud Guard problem listing is compartment-scoped, can optionally traverse subcompartments with access-level controls, and filters by time, lifecycle, risk level, detector type, target, problem category, and resource type.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate documented control intent, sampled current-state evidence, compliance mapping, and remediation advice.
- Require detector coverage, target scope, IAM scope, logging, encryption, network exposure, and owner evidence before posture claims.
- Treat public exposure, all-resources manage policies, disabled logging, weak encryption, and missing detector coverage as high risk.
- Do not expose identities, customer data, logs, resource identifiers, or sensitive compliance artifacts.

## Minimal safe implementation flow

- Confirm framework/control objective, scope, systems, evidence level, and decision needed.
- Use official docs for control behavior and sampled read-only evidence for Cloud Guard/API shape or current findings.
- Review IAM, compartments, network, logging, encryption, scanning, Security Zones, Cloud Guard, and responder boundaries.
- Return verdict, evidence gaps, prioritized findings, safe remediation, and audit caveats.

## High-risk assumptions to kill

- “No Cloud Guard problems means secure.”
- “Admin access is needed for audit.”
- “Encryption enabled means keys are governed.”
- “A landing zone template equals compliant operations.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check Cloud Guard target/detector/responder coverage and problem filters.
- Check IAM policy scope, broad principals, network exposure, logging, vault/key posture, scanning coverage, and Security Zone violations.
- Map findings to controls with evidence level and timestamp.
- Validate remediation with least privilege and rollback.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for compliance sign-off without sampled evidence.
- Detector coverage or log retention is unknown.
- The requested remediation is destructive or over-permissive.
