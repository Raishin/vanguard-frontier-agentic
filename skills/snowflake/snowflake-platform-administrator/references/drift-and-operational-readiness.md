# Drift and Operational Readiness

How to measure drift instead of asserting it, and what makes an estate genuinely operable by someone who did not build it. Load for readiness reviews and post-incident hygiene work.

## Measuring drift

- Drift is a diff, not an adjective. Produce it as counts by object class: present in intent and absent in the account, present in the account and absent from intent, and present in both with differing properties.
- The third category is where the damage lives. An object that exists in both but with a different retention, owner, or parameter is the one that passes a naive check and fails in an incident.
- Recurring drift after remediation is a pipeline defect. Reporting it repeatedly as an administrative finding hides the real owner.
- Objects created outside IaC are not automatically wrong — but every one of them needs a named owner and a reason, or it is an unmanaged dependency.

## Readiness is rehearsal, not documentation

- For each routine failure mode — a suspended task chain, a failing pipe, an expired integration credential, a runaway query, an exhausted resource monitor — ask: is there a written procedure, does it name an owner, and has someone who did not write it executed it?
- A runbook that has never been executed by a second person is documentation. The readiness finding is the rehearsal gap.
- Monitoring coverage is stated as what would go unseen. 'We monitor query history' is not coverage; 'a pipe that stops ingesting is detected within N minutes by X' is.
- Account Usage latency bounds every detection claim. If the view is the detection mechanism, the detection time cannot be shorter than its latency.
- Escalation paths must resolve to a person with the privilege to act. A path that ends at a role nobody holds outside business hours is not an escalation path.
