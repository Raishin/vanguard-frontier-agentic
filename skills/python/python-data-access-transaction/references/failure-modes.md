# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- An N+1 over a large collection turns one page load into thousands of queries and times out under load.
- A missing rollback on an error path leaves the Session in a failed transaction and every subsequent request errors.
- A migration adding a NOT NULL column with a default takes an exclusive lock and stalls the whole service during deploy.
- A leaked connection per request exhausts the pool and the service stops accepting work.
- A query missing its tenant filter returns another customer's rows.
