# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports (no orchestrator URLs, no runner credentials, no production queue data, no PII in test variables):
- Exported workflow definitions: UiPath `.xaml` files, Automation Anywhere `.atmx` or task JSON, Power Automate Desktop `.zip` or flow JSON, Blue Prism process XML
- Project dependency manifest or `project.json` (UiPath) / package descriptor
- Orchestrator asset list (names and types only — no values, no credential content)
- Optional: a recent orchestrator job log excerpt showing the failure (sanitized — no connection strings, no stack traces containing file paths with tenant IDs)

If only a partial set is provided, note which inputs are absent and scope findings accordingly. A workflow without its project manifest leaves dependency and version blind spots. An asset list without the workflow leaves credential-usage patterns invisible — say so explicitly.

### Step 2 — Credential and secrets audit

Scan every workflow variable, argument, activity property, configuration file, and annotation for hardcoded secrets.

**2a. Hardcoded credentials**

```xml
<!-- CRITICAL — password stored as a plain workflow argument default -->
<x:Property Name="SAPPassword" Type="InArgument(x:String)" />
<!-- Default value: "P@ssw0rd2024" -->
```

Credentials must live exclusively in the orchestrator credential vault (UiPath Orchestrator Assets of type Credential, AA Control Room credentials, Power Platform environment variables with secret type, or Blue Prism credential manager). The workflow retrieves them at runtime via a `Get Credential` / `Get Asset` activity and holds them in a `SecureString` variable scoped to the minimum lifetime needed.

**2b. Connection strings and API tokens**

```xml
<!-- CRITICAL — API key baked into an Assign activity -->
<Assign>
  <Assign.To><OutArgument x:TypeArguments="x:String">apiKey</OutArgument></Assign.To>
  <Assign.Value><InArgument x:TypeArguments="x:String">sk-live-abc123XYZ</InArgument></Assign.Value>
</Assign>
```

Any string matching patterns for API keys (`sk-`, `Bearer `, `token=`, JDBC/ODBC DSNs with `Password=`) is CRITICAL regardless of whether it looks like a placeholder.

### Step 3 — Selector resilience audit

Review every UI activity's selector for volatile versus stable attributes.

| Selector attribute | Verdict | Why |
|---|---|---|
| `automationid`, `name`, `controltype` from accessibility tree | preferred | stable across layout changes and minor version bumps |
| Fixed `title` with exact window name (non-session-specific) | acceptable | stable if the application version is locked |
| `idx` positional attribute | HIGH | breaks when the UI gains or loses elements above the target |
| Absolute screen coordinates (`x`, `y` in click activity) | HIGH | breaks on any resolution, DPI, or window-size change |
| Dynamic `title` containing session IDs, timestamps, or user names | HIGH | every robot instance generates a unique value |
| Auto-generated IDs (WinForms `textBox1`, SAP session `wnd[0]/usr/txtRSYST-BNAME`) with session ordinals | HIGH | ordinal changes when multiple SAP sessions are open |
| Partial match (`*` wildcard) on a stable prefix | acceptable | use sparingly; too broad a wildcard matches wrong elements |

Flag each HIGH selector with the activity name, the volatile attribute, and the recommended stable replacement.

**Example remediation for a SAP session-ordinal selector:**

```xml
<!-- HIGH — wnd[0] ordinal is relative to open SAP windows -->
<uipath:TypeInto Selector="&lt;wnd app='saplogon.exe' title='SAP Easy Access' /&gt;&lt;wnd idx='1' /&gt;" />

<!-- CORRECT — use the window's automation ID instead -->
<uipath:TypeInto Selector="&lt;wnd app='saplogon.exe' automationid='MainWindow' /&gt;" />
```

### Step 4 — Exception handling coverage audit

Verify that every application or UI interaction boundary is wrapped in an exception handler.

Check for:
- Any `Click`, `Type Into`, `Get Text`, `Attach Browser`, `Open Application`, `Send Hotkey`, or platform-equivalent activity outside a Try/Catch or Retry Scope → HIGH (silent failure on the happy path; unattended run terminates with no item status update)
- A Try/Catch that catches `System.Exception` but only logs a generic message with no item-level failure status update to the orchestrator queue → MEDIUM (the exception is swallowed; the queue item stays In Progress forever)
- A Retry Scope with `NumberOfRetries` set to 0 or left at default without justification → MEDIUM (equivalent to no retry on transient failures)
- No Global Exception Handler configured at the project level → MEDIUM (any unhandled exception in an invoked workflow bypasses all local handlers)

```xml
<!-- HIGH — UI interaction with no surrounding exception handler -->
<uipath:Click Selector="&lt;html app='chrome.exe' /&gt;&lt;webctrl id='submit-btn' /&gt;" />

<!-- CORRECT — wrapped in Try/Catch with queue item failure status on catch -->
<TryCatch>
  <Try>
    <uipath:Click Selector="..." />
  </Try>
  <Catches>
    <Catch x:TypeArguments="s:Exception">
      <uipath:SetTransactionStatus Status="Failed" ErrorMessage="[ExceptionMessage]" />
      <Rethrow />
    </Catch>
  </Catches>
</TryCatch>
```

### Step 5 — Idempotency and transaction safety audit

Verify that every workflow is safe to re-run after a partial failure without duplicating side effects.

- A workflow that submits a form, sends an email, posts a financial transaction, or creates a record with no "already processed" guard (a status field check, a queue item deduplication key, a database flag) → HIGH
- A workflow that reads from an orchestrator queue but does not update the item status to `Successful` or `Failed` on every exit path → HIGH (items stuck In Progress block the queue and prevent retry)
- Two robot instances that can claim the same queue item simultaneously with no server-side lock (i.e., not using orchestrator's built-in queue transaction model, instead reading from a shared spreadsheet with no advisory lock) → MEDIUM
- No rollback or compensating transaction when a multi-step process fails partway through → MEDIUM

```xml
<!-- HIGH — no idempotency guard before submitting payment -->
<uipath:Click Selector="'Submit Payment'" />

<!-- CORRECT — check whether this transaction ID was already posted -->
<uipath:GetQueueItemData ItemField="Reference" Result="[transactionRef]" />
<If Condition="[alreadyPostedLookup(transactionRef)]">
  <Then>
    <uipath:SetTransactionStatus Status="Successful" />
    <!-- skip re-submission -->
  </Then>
  <Else>
    <uipath:Click Selector="'Submit Payment'" />
    <uipath:SetTransactionStatus Status="Successful" />
  </Else>
</If>
```

### Step 6 — Wait strategy audit

Scan every workflow for fixed Delay activities used as application synchronization.

- Any `Delay` activity with a hardcoded duration (1 s, 3 s, 5 s) placed before a UI interaction → HIGH (races the application on fast machines; adds unnecessary latency on slow ones; the RPA equivalent of `Thread.Sleep`)
- `WaitForReady` property left at `None` on a UI activity that targets a freshly loaded page or dialog → HIGH (the activity fires before the target element exists)
- A pattern of `Delay` + `Element Exists` polling in a loop instead of `Wait Element Vanish` / `On Element Appear` / `Check App State` → MEDIUM

```xml
<!-- HIGH — hard sleep before clicking a dynamically loaded button -->
<Delay Duration="[00:00:03]" />
<uipath:Click Selector="'Confirm'" />

<!-- CORRECT — wait for the element to become ready -->
<uipath:WaitForElement Selector="'Confirm'" Timeout="[00:00:30]" />
<uipath:Click Selector="'Confirm'" />
```

### Step 7 — Attended/unattended compatibility audit

Identify attended-only constructs inside workflows scheduled or deployed for unattended execution.

- Any `Message Box`, `Input Dialog`, `Prompt` activity, or platform-equivalent user interaction prompt inside a workflow that will run on an unattended robot or headless VM → HIGH (blocks indefinitely; consumes a licensed robot slot until manually killed)
- A workflow that assumes a logged-in desktop session or a specific screen resolution without a session-setup or auto-login step → HIGH
- A workflow that calls `Kill Process` or `Close Application` on a process that may not be running (no existence check first) → MEDIUM

### Step 8 — Observability audit

Verify that failures are visible before downstream systems surface them.

- No `Log Message` activities at key decision points, transaction boundaries, or exception handlers → MEDIUM (failures are invisible until SLA breach)
- No per-item status update (`Set Transaction Status` or equivalent) on every exit path, including exception branches → MEDIUM (queue backlog grows silently)
- No alert or notification on repeated failure (three consecutive `Failed` items, orchestrator alert rule, or a monitoring webhook call) → MEDIUM

### Step 9 — Cleanup and session hygiene audit

Verify that failure paths close all acquired resources.

- Browser sessions, SAP logons, application windows, or file handles acquired in the workflow with no corresponding close or kill activity on the exception path → MEDIUM (session leaks; connection pool exhaustion on the orchestrator VM)
- A Finally block absent from Try/Catch sequences that open external connections → MEDIUM

### Step 10 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<exported workflow provided | partial artifacts | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept orchestrator URLs with embedded credentials, runner service-account passwords, production queue data, or PII in variable defaults. Ask for sanitized exports with placeholder values.
- This is a static review: do not connect to a live orchestrator, execute a bot, or resolve orchestrator asset values. The review is based solely on the exported workflow artifact.
- Do not recommend removing exception handling or disabling logging to simplify a workflow — both are load-bearing safety mechanisms. Refuse and explain.
- Do not recommend hardcoding credentials even temporarily, even in a test workflow — credential exposure in source control is irreversible once committed.
- If a workflow export contains apparent PII (real customer names, account numbers, national IDs in variable defaults or test-data annotations), flag it as HIGH, redact it from the review output, and instruct the user to sanitize before re-sharing.
