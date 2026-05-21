# Apex Debug Log Format Reference

Adapted from forcedotcom/sf-skills debugging-apex-logs references (Apache-2.0).

## Log Line Structure

Every log line follows this format:

```
HH:MM:SS.mmm (elapsed_ms)|LOG_CATEGORY|description
```

Example:
```
12:34:56.123 (123456789)|SOQL_EXECUTE_BEGIN|[45]|Rows:50|SELECT Id, Name FROM Account WHERE OwnerId = :ownerId
```

- `HH:MM:SS.mmm` — wall clock time
- `(elapsed_ms)` — nanoseconds since transaction start (used for performance timing)
- `LOG_CATEGORY` — the event type
- `description` — category-specific content

---

## Key Log Categories

### Execution Entry Points

| Category | Description |
|---|---|
| `EXECUTION_STARTED` | Transaction begins |
| `EXECUTION_FINISHED` | Transaction ends |
| `CODE_UNIT_STARTED` | A class/trigger/process starts executing |
| `CODE_UNIT_FINISHED` | A class/trigger/process finishes |
| `ENTERING_MANAGED_PKG` | Execution enters a managed package namespace |

### Method Tracking

| Category | Description |
|---|---|
| `METHOD_ENTRY` | Entering a method; shows `[lineNum]|namespace.ClassName.methodName` |
| `METHOD_EXIT` | Exiting a method |
| `CONSTRUCTOR_ENTRY` | Entering a constructor |
| `CONSTRUCTOR_EXIT` | Exiting a constructor |

### SOQL

| Category | Description |
|---|---|
| `SOQL_EXECUTE_BEGIN` | SOQL query begins; shows `[lineNum]\|Rows:N\|<query text>` |
| `SOQL_EXECUTE_END` | SOQL query ends; shows row count returned |

**N+1 pattern signature:** Multiple `SOQL_EXECUTE_BEGIN` entries with the same query
pattern appear sequentially, often interleaved with `METHOD_ENTRY`/`METHOD_EXIT` for
the same method — indicates a loop calling SOQL.

### DML

| Category | Description |
|---|---|
| `DML_BEGIN` | DML operation begins; shows `[lineNum]\|Op:Insert\|Type:Account\|Rows:N` |
| `DML_END` | DML operation ends |

**DML in loop signature:** Multiple `DML_BEGIN` entries with `Rows:1` for the same
object type appearing in rapid succession, with loop method frames on the stack.

### Governor Limits

| Category | Description |
|---|---|
| `LIMIT_USAGE_FOR_NS` | Snapshot of limit consumption at transaction end or limit breach |

Example entry:
```
LIMIT_USAGE_FOR_NS|(default)|
  Number of SOQL queries: 101 out of 100
  Number of query rows: 4523 out of 50000
  Number of DML statements: 23 out of 150
  Number of DML rows: 4500 out of 10000
  CPU time (ms): 8234 out of 10000
  Maximum heap size (MB): 3.2 out of 6
```

**Hard limit breach signature:** `LIMIT_USAGE_FOR_NS` showing a value equal to or
exceeding the limit, followed by `FATAL_ERROR` with `LimitException`.

### Exceptions

| Category | Description |
|---|---|
| `EXCEPTION_THROWN` | Exception instantiated; shows type and message |
| `FATAL_ERROR` | Unhandled exception; transaction will roll back; shows full stack trace |

Example:
```
12:34:56.789 (789456123)|FATAL_ERROR|System.LimitException: Too many SOQL queries: 101
Class.AccountSelector.getAccountsByOwnerIds: line 12, column 1
Class.AccountService.processOwnerChange: line 34, column 1
Trigger.AccountTrigger: line 8, column 1
```

### User Debug

| Category | Description |
|---|---|
| `USER_DEBUG` | `System.debug` output; shows `[lineNum]\|DEBUG\|<message>` |

**Caution:** USER_DEBUG lines may contain field values, record IDs, or PII if
`System.debug` serializes SObjects. Apply redaction to these lines.

### Callouts

| Category | Description |
|---|---|
| `CALLOUT_REQUEST` | Outbound HTTP request begins; shows URL and method |
| `CALLOUT_RESPONSE` | Outbound HTTP response returns; shows status code |

### Async

| Category | Description |
|---|---|
| `FUTURE_CALL_PROCESS` | Future method queued |
| `SCHEDULED_PROCESS_STARTING` | Scheduled job begins executing |
| `BATCH_PROCESS_STARTED` | Batch execute begins |

---

## Log Levels

Set via Developer Console or `sf apex tail log`. Higher log levels produce more verbose
output but consume log size limit faster.

| Level | What it captures |
|---|---|
| `NONE` | Nothing |
| `ERROR` | Fatal errors only |
| `WARN` | Errors + warnings |
| `INFO` | + method boundaries |
| `DEBUG` | + USER_DEBUG output |
| `FINE` | + method entry/exit |
| `FINER` | + constructor tracking |
| `FINEST` | + all SOQL/DML details |

**Recommended for diagnosis:** `APEX_CODE=FINEST, APEX_PROFILING=INFO, DB=FINEST`

---

## Log Size Limit

Apex debug logs are capped at **20 MB**. Logs exceeding this limit are truncated with
a `*** Skipped N bytes of detailed log ***` entry. When analyzing truncated logs:
- Focus on the end of the log (where the exception or limit hit occurred)
- Reduce log level for categories not relevant to the issue
- Use `sf apex tail log` for streaming analysis of in-progress transactions
