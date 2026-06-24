# Safety checklist

Use this reference before any recommendation that involves containment actions, AIR automation level changes, automated-response policy modifications, Sentinel playbook execution, or any other live Microsoft Defender XDR or Sentinel production action.

## Non-negotiables

- Never recommend or initiate device isolation, user account disable, file or URL block, process termination, or any other containment action without explicit SecOps owner approval and a documented blast-radius assessment. State this escalation requirement plainly.
- Never recommend changing AIR automation levels (especially from Semi to Full for untested device groups) without a false-positive rate assessment and a tested rollback procedure.
- Never recommend executing Sentinel playbooks against production environments without a dry-run validation and SecOps owner approval.
- Never ask users to paste secrets, admin credentials, tenant IDs, API keys, certificates, private keys, or customer data into chat.
- Use read-only Defender XDR portal evidence, Graph Security API read evidence, or Sentinel workspace query results for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent incident states, AIR verdicts, alert volumes, or Sentinel analytics rule coverage.
- Require explicit SecOps owner approval before recommending any containment action, AIR configuration change, custom detection rule deployment, or Sentinel playbook modification.
- Keep remediation least-privilege, reversible, staged (advisory before live execution), and scoped to the requested threat scenario.
- Apply Zero Trust assume-breach: treat every unconfirmed incident as active and every unreviewed containment reversal as a potential re-exposure risk.
- Treat any AIR automation level set to None for all device groups as a gap — manual-only remediation creates unacceptable response latency for ransomware and lateral movement scenarios.

## Stress checks

- What containment action is being recommended, and has the SecOps owner explicitly approved it?
- What is the blast radius if the device isolation or account disable is incorrect (wrong device, wrong user)?
- What rollback path exists to restore a contained device or re-enable a disabled account if the containment was a false positive?
- What AIR device group automation level change is being recommended, and has the false-positive rate been assessed for that group?
- What Sentinel playbook is being triggered, and has it been validated in a non-production environment?
- What post-containment review cadence confirms that disrupted entities are safe to restore?
- What advanced hunting query is being run in production, and does it scope to a time-bounded read-only query with no response actions?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Microsoft Defender XDR incident state, AIR automation level, or Sentinel analytics rule deployment.

## Escalation triggers

Escalate to SecOps owner live-guard gate before any of the following:

- Initiating device isolation, network containment, or any endpoint response action in Defender for Endpoint
- Disabling user accounts, revoking sessions, or blocking sign-in in Microsoft Entra ID as a containment response
- Blocking files, URLs, IP addresses, or domains via Defender for Endpoint indicators
- Changing AIR automation levels for any device group (especially Semi to Full or Full to None)
- Approving or rejecting pending AIR remediation actions in the Action Center for high-severity incidents
- Creating, modifying, or enabling Sentinel analytics rules that would fire on production data
- Triggering Sentinel playbooks against production resources or user accounts
- Modifying custom detection rules that generate automatic response actions
- Reversing automatic attack disruption actions (restoring isolated devices, re-enabling disrupted accounts)
