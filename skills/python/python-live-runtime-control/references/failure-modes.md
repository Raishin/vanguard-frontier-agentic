# Failure Modes This Role Prevents

The concrete production incidents this role's read-only diagnostics are designed to catch early.

- A stuck worker pool goes undetected because no one reads live thread/task state, and requests queue until the service times out.
- A slow memory leak is missed until an out-of-memory kill takes down the process, because no diagnostic snapshot was captured earlier.
- A diagnostic tool that can also restart a process is used to 'quickly fix' an issue, mutating production state outside of a governed release.
- A stale diagnostic snapshot is presented as current live state, misleading an incident responder about what is happening now.
- Leaked asyncio tasks accumulate silently because no read-only health check ever surfaces them as a finding.
