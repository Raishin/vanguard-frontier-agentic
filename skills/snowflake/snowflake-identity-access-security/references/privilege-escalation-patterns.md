# Privilege Escalation Patterns

The concrete sequences by which a Snowflake principal reaches capability it was never intended to have. Load when answering 'what happens if this identity is compromised'.

## Patterns to hunt for by name

- **Ownership-mediated escalation.** A role owns an object; the owner can grant on it; therefore anyone holding that role can extend access to anyone. Look for OWNERSHIP held by roles that are widely inherited.
- **Convenience-edge escalation.** Role A was granted to role B once, for one project. Every future privilege on A silently flows to B and everything above B. These accumulate and are never audited.
- **Future-grant escalation.** A future grant at database scope means every table a pipeline creates tomorrow is already readable by a role nobody re-reviewed.
- **PUBLIC escalation.** Anything granted to PUBLIC reaches every identity in the account, including every service user and every future user.
- **Grant-management overuse.** A role able to manage grants broadly can construct any access it wants; it is administratively equivalent to the access it can create.
- **Static-credential escalation.** A key or token that lives in a repository, a CI variable, a notebook, or a connector configuration is as privileged as the role it uses, and it does not expire when the person who created it leaves.
- **Default-role escalation.** A service user whose `DEFAULT_ROLE` is a system role starts every session at maximum privilege, so any injection or misconfiguration inherits it.
- **Integration escalation.** Security, storage, and external access integrations bind Snowflake to an external identity. Compromise of either side crosses the boundary in both directions.
- **Stale-principal escalation.** A disabled user is not a removed role. If the role persists and is granted elsewhere, the access persists too.
- **Break-glass without controls.** An emergency account with a stored password, no MFA, no network constraint, and no alert on use is a permanent backdoor rather than a break-glass control.

## How to report an escalation path

- State the starting principal, each edge in the path with the evidence that established it, and the terminal capability reached.
- State the time to revoke: which single change breaks the path, who can make it, and whether it can be made outside business hours.
- State what the path would have accessed if it had been used — from access history where available — so the report distinguishes a theoretical path from an exercised one.
- Rank by terminal capability and reachability, never by count of findings. Twenty low-consequence findings above one path to account administration is a report that will be ignored for the right reason.
