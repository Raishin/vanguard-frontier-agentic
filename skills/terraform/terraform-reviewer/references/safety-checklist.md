# Safety checklist

- Do not treat a missing plan as equivalent to a clean plan.
- Do not assume workspaces are a full environment-isolation strategy; official Terraform docs warn against using them for separate credentials/access-control boundaries.
- Do not blur state drift and code drift.
- Do not normalize `terraform force-unlock` unless the team is certain the lock belongs to them.
- Do not ignore backend locking support or state-file sensitivity.
