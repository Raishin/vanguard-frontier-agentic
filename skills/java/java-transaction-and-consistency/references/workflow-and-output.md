# Workflow and Output Contract

> Static review only. Read `@Transactional`-annotated classes, their call graph, and sanitized configuration (transaction-manager beans, datasource pool settings, broker client config only insofar as it evidences a dual write). Never open a database or broker connection, start or commit a transaction, or execute code. Ask for source with placeholders — never connection strings, credentials, tenant identifiers, or customer data.

## Workflow

### Step 1 — Collect inputs

Ask the user for whichever apply, sanitized:
- The `@Transactional`-annotated method(s) under review, with their full attribute list (`propagation`, `isolation`, `readOnly`, `rollbackFor`/`noRollbackFor`, `timeout`).
- The call graph: who invokes each `@Transactional` method, and from where (same class/bean vs. a different bean; inside an existing transaction vs. not).
- Any sequence where a database write is followed by a message/event publish, a call to another service, or a cache update.
- Any post-commit side-effect handling already in place (`TransactionSynchronization`, `@TransactionalEventListener`, an outbox table, a saga orchestrator/choreography).
- Relevant configuration: the `PlatformTransactionManager`/datasource bean setup, connection-pool settings if boundary width is in scope, and whether an outbox/relay or saga infrastructure already exists.

If the call site or the annotation attributes are missing for a method under discussion, downgrade that finding to `inference (partial source)` or `assumption (source absent)` and say so.

### Step 2 — Map propagation and rollback per method

For each `@Transactional` method, record: propagation, isolation, readOnly, rollbackFor/noRollbackFor, and every checked exception it declares or can throw. Cross-reference against the default rollback rule (unchecked + `Error` only) to find gaps.

### Step 3 — Trace every call site for self-invocation and join behavior

For each `@Transactional` method, determine: is it ever called from another method on the *same* bean (self-invocation — flag unconditionally)? Is it ever called from within another `@Transactional` method elsewhere (a `REQUIRED` join — its own isolation/timeout are then unreliable)? Is its boundary width limited to DB work, or does it wrap an external call?

### Step 4 — Detect dual-write and post-commit races

For each sequence that writes to the database and then touches a second resource (broker, service call, cache): is the second write inside the same DB transaction (it can't truly be — flag if the code implies otherwise), deferred to `AFTER_COMMIT`, or routed through an outbox? A direct publish/call with no outbox and no `AFTER_COMMIT` deferral is the dual-write anti-pattern regardless of ordering.

### Step 5 — Assess cross-service consistency design

If the consistency question spans services: is atomicity attempted via a distributed transaction manager (flag as fragile) or a saga? If a saga, does every forward step with an externally visible effect have a paired, idempotent compensation, and are steps idempotent under retry?

### Step 6 — Produce the output

Format using the Output contract below. Pick each remedy by the actual resources and call graph involved; never recommend a manual pseudo-outbox, a blanket `REQUIRES_NEW` everywhere, or XA/2PC across service boundaries as a default.

## Evidence checklist

- [ ] `@Transactional` attributes for each method in scope
- [ ] Call graph (same-bean self-invocation and cross-transaction joins)
- [ ] Any write-then-publish/call/cache sequence and its resource boundaries
- [ ] Existing post-commit handling (`AFTER_COMMIT`, outbox, saga infrastructure)
- [ ] Transaction-manager and pool configuration (if boundary width is in scope)

Each unchecked item downgrades the related findings to `inference` or `assumption`.

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | Proxy self-invocation silently skipping `@Transactional`; checked exception with no `rollbackFor` committing a failed write; save()-then-send() dual write with no outbox; a required isolation/timeout silently dropped under a `REQUIRED` join with a stated correctness dependency on it. |
| high | Over-wide boundary wrapping an external call; post-commit side effect not deferred to `AFTER_COMMIT`/outbox; `readOnly = true` performing a write; cross-service atomicity attempted via XA/2PC. |
| medium | Unjustified relaxed isolation level; `REQUIRES_NEW` used without stating the commit-independence trade-off; incomplete saga compensation coverage. |
| low | Large batch operation in one long-lived transaction; a genuinely read-only method missing `readOnly = true`. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source | partial source | inference>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <call site / resource pair> — <remedy: rollbackFor / de-duplicate self-invocation / outbox / AFTER_COMMIT>

### HIGH
- [H1] <finding> — <evidence basis> — <description> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <annotation attribute, call site, or resource boundary the user must supply>
```

## Security notes

- Never request or accept connection strings, credentials, tenant identifiers, or customer data. Ask for source with placeholders.
- Static review only: never open a database or broker connection, start or commit a transaction, or execute code.
- Never recommend a manual pseudo-two-phase workaround, a blanket `REQUIRES_NEW`, or XA/2PC across service boundaries as a default fix.
- Never recommend disabling a failing gate as the fix.
