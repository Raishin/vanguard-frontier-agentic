# Workflow and output contract

Use this reference only when performing the full Intune endpoint management posture review or formatting the final review.

## Review domains

Check these areas before giving a verdict:

- **Device enrollment coverage**: Enrolled vs. unmanaged device ratio, enrollment methods per platform (Microsoft Entra join, Hybrid join, Windows Autopilot, Apple ADE, Android Enterprise), and BYOD enrollment policy
- **Compliance policies**: Per-platform compliance requirements (OS version, encryption, Defender health, jailbreak/root detection), noncompliance actions, grace periods, and Conditional Access "require compliant device" enforcement
- **Configuration profiles**: Device restriction profiles, endpoint protection profiles, Settings Catalog profiles, Wi-Fi and VPN certificate delivery, and profile conflict detection
- **App protection (MAM) policies**: Data transfer restrictions, PIN requirements, managed browser enforcement, level 2 protection coverage, and unmanaged device coverage
- **Conditional Access device-compliance signal**: Require compliant device or Microsoft Entra hybrid join enforcement, app protection policy requirement, named location and platform conditions
- **Windows Autopilot**: Deployment profiles, enrollment status page (ESP) configuration, device group tag strategy, and hybrid join vs. cloud-native architecture
- **Update rings and feature updates**: Ring structure (test/pilot/broad), deferral periods, deadline enforcement, Windows Autopatch adoption, and ring assignment coverage
- **Endpoint security baselines**: Baseline selection, customization from defaults, conflict detection, and compliance monitoring
- **Endpoint Privilege Management (EPM)**: Standard user elevation policies, elevation rules, and LAPS configuration
- **Defender for Endpoint integration**: Intune connector status, device risk signal feeding Conditional Access, and vulnerability remediation task workflow

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (Intune Plan 1 or Plan 2):
   - Device platform mix and enrollment method:
   - BYOD vs. corporate-owned split:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Intune admin center evidence or Microsoft Graph read output for current-state claims when available.
   - Otherwise inspect repository IaC/config (Bicep, Terraform, JSON exports), sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What device path allows an attacker with stolen credentials to access corporate resources from an unmanaged or noncompliant device?
   - What app protection gap allows corporate data to leak from a managed app to personal storage or an unmanaged app?
   - What update ring gap leaves endpoints exposed to known vulnerabilities beyond acceptable deferral windows?
   - What compliance policy is assigned but not enforced by Conditional Access?
   - What baseline conflict or missing profile leaves a security configuration gap?
   - What rollback path exists if a compliance policy or security baseline change causes a broad lockout?
4. **Recommend the smallest safe action**
   - Prefer report mode for new compliance policies, staged rollout (pilot group), app protection policy in audit mode before enforcement, and update ring testing before broad assignment.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Intune Endpoint Management Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Control area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or reports to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
