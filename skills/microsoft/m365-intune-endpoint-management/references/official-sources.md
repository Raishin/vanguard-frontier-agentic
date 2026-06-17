# Official sources

Use this reference only when you need source grounding for Microsoft Intune endpoint management, device compliance, app protection, Windows Autopilot, update rings, endpoint security baselines, or Zero Trust device-health-as-signal service behavior.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Intune tenant state:

- https://learn.microsoft.com/intune/fundamentals/zero-trust
- https://learn.microsoft.com/security/zero-trust/manage-devices-with-intune-overview
- https://learn.microsoft.com/security/zero-trust/manage-devices-with-intune-compliance-policies
- https://learn.microsoft.com/security/zero-trust/manage-devices-with-intune-app-protection
- https://learn.microsoft.com/intune/device-security/security-baselines/overview
- https://learn.microsoft.com/intune/device-updates/windows/manage-update-rings
- https://learn.microsoft.com/autopilot/windows-autopilot-overview
- https://learn.microsoft.com/intune/device-security/endpoint-security-policies
- https://learn.microsoft.com/intune/device-security/compliance/overview
- https://learn.microsoft.com/intune/apps/app-protection-policy

## Grounding rule

Official documentation explains Microsoft Intune and endpoint management service behavior. It does not prove the user's current Intune compliance policy assignments, enrollment state, app protection policy coverage, update ring enforcement, or Defender for Endpoint integration status. Prefer read-only Intune admin center evidence, Microsoft Graph read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Zero Trust with Intune (per learn.microsoft.com/intune/fundamentals/zero-trust):**
- Verify explicitly: Intune compliance policies validate device health (OS version, encryption, Defender status, PIN) as a Conditional Access signal
- Least privilege: Endpoint Privilege Management (EPM) allows standard users to complete elevation tasks without standing local admin rights; LAPS manages local administrator credentials
- Assume breach: Intune integrates with Microsoft Defender for Endpoint for mobile threat defense, device risk scoring, and vulnerability remediation workflows via Security Copilot

**Compliance policies (per learn.microsoft.com/security/zero-trust/manage-devices-with-intune-compliance-policies):**
- Define minimum device health requirements per platform (Windows, iOS/iPadOS, Android, macOS)
- Noncompliant devices trigger configurable actions: mark noncompliant, notify user, block, retire
- Compliance signals feed Conditional Access — "require compliant device" policy blocks noncompliant access
- Devices must be enrolled in Intune before compliance policies apply

**App protection (MAM) policies (per learn.microsoft.com/security/zero-trust/manage-devices-with-intune-app-protection):**
- Level 2 enterprise enhanced data protection is the recommended starting level for devices accessing sensitive data
- MAM policies protect corporate data in managed apps without requiring device enrollment
- Key controls: restrict cut/copy/paste to managed apps, require PIN, block backup to personal storage, require managed browser, wipe corporate data on unenrollment
- Coordinate MAM policies with Conditional Access "require approved app and app protection" policy

**Security baselines (per learn.microsoft.com/intune/device-security/security-baselines/overview):**
- Preconfigured groups of Windows settings recommended by Microsoft security teams
- Available baselines: Windows MDM security baseline, Microsoft Defender for Endpoint baseline, Microsoft Edge baseline
- Test in isolation before broad deployment — some settings may conflict with Windows Autopilot or application delivery

**Update rings (per learn.microsoft.com/intune/device-updates/windows/manage-update-rings):**
- Control Windows update deferral periods, deadlines, restart behavior, and active hours
- Common ring structure: test (0-day deferral), pilot (7-day), broad (14-21 day)
- Windows Autopatch manages ring cadence automatically for qualifying tenants

**Common failure modes:**
- No app protection policies for BYOD/unmanaged devices accessing corporate data
- Compliance policies assigned but Conditional Access not enforcing "require compliant device"
- Missing Defender for Endpoint integration — device risk not flowing to Conditional Access
- Update rings defined but not assigned — endpoints receiving updates without deferral control
- Security baselines not tested before broad deployment causing application compatibility issues
- Autopilot profiles missing enrollment status page (ESP) — users reaching desktop before policy applies

Review implications:
- Do not approve Conditional Access designs that allow noncompliant devices as a broad exception without compensating controls.
- Compliance policy assignment alone is insufficient — verify that Conditional Access is enforcing the compliance signal.
- Documentation cannot prove the user's actual Intune enrollment coverage, compliance policy assignments, or Defender for Endpoint integration state.
