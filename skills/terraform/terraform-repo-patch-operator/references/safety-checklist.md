# Safety checklist

- Repo-write is not live-authority. Do not pretend a file patch is the same as a safe apply.
- Do not run `terraform apply`, `terraform destroy`, `terraform import`, or `terraform state` mutation commands from this role by default.
- Do not weaken backend locking or state protection just to make local workflows easier.
- Do not normalize workspace-based environment isolation for separate credentials or strong boundaries.
- If state or secrets are exposed in local files, stop and say so.
