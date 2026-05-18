# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports (no live controller IP addresses, no plant-network hostnames, no historian credentials, no production asset tags that identify a specific facility or unit):
- Exported program logic in text form: Structured Text (`.st`, `.txt`), L5X/L5K (Rockwell), exported XML (IEC 61131-3 PLCopen XML), or a pasted ladder rung / function block description.
- The I/O list or hardware configuration (module types, I/O addresses, safety vs. standard I/O).
- The safety requirements specification (SRS) or SIL/PL assessment for the relevant safety instrumented function (SIF), if available.
- The process hazard analysis (PHA) or HAZOP summary for the unit, if available.
- The watchdog and communication-loss behavior specification, if documented separately.

If only a partial set is provided, note which inputs are absent and scope every finding accordingly. Logic without an I/O list leaves safety-rated channel identification as inference; say so explicitly.

### Step 2 — Safety function and E-stop implementation audit

Identify every E-stop, emergency trip, or safety instrumented function (SIF) referenced in the exported logic.

**2a. Software-only E-stop — CRITICAL**

A safety function implemented entirely in standard PLC software cannot achieve SIL 1 or higher under IEC 61508 without architectural redundancy and systematic capability claims that a standard PLC runtime does not provide. A scan fault, firmware defect, or communications outage can prevent the software path from executing.

Correct implementation pattern:
- Hardware E-stop loop through a safety relay or safety-rated PLC (e.g., Pilz PNOZ, Siemens SIRIUS, Allen-Bradley GuardLogix safety task with dual-channel input).
- Standard PLC logic may signal or acknowledge the trip, but must never be the sole means of de-energizing the hazardous output.

Flag any rung or ST block where an E-stop coil address is driven exclusively by software logic with no cross-reference to a hardware-forced safety output or a safety PLC output.

**2b. Safety PLC vs. standard PLC**

If the exported logic is from a safety-rated task (e.g., GuardLogix safety task, Siemens F-CPU, HIMA HIMax), confirm that safety I/O is referenced via the safety I/O map, not standard I/O. Mixing safety and standard I/O addresses for the same SIF is CRITICAL.

### Step 3 — Output fail-safe and de-energization audit

For every output coil or output block in the export:

- Trace whether a fail-safe (de-energized) state is reachable when: (a) the PLC transitions to STOP or FAULT mode, (b) the remote I/O link drops, (c) the safety function trips.
- Flag outputs where the only de-energization path is through software logic that may not execute during a PLC halt.

**Example — CRITICAL: output holds last state on I/O dropout**

```
(* Structured Text — standard remote I/O, no watchdog output *)
IF ConveyorRunCmd AND NOT EStop_SoftBit THEN
    ConveyorMotor_Out := TRUE;
END_IF;
(* No else-branch; no watchdog; remote I/O module holds last state on link loss *)
```

Correct pattern — explicit safe state on every execution path:

```
(* Every path terminates in an explicit assignment *)
IF ConveyorRunCmd AND NOT EStop_HW AND CommOK THEN
    ConveyorMotor_Out := TRUE;
ELSE
    ConveyorMotor_Out := FALSE;  (* de-energize on any fault condition *)
END_IF;
```

### Step 4 — Latch and SET/RESET integrity audit

Scan every SET coil, SR function block, retentive output, and latch pattern in the export.

Check for:
- A SET with no cross-referenced RESET anywhere in the POU or program — HIGH (output permanently energized; requires a force to clear).
- A RESET gated behind a condition that is logically unreachable (e.g., RESET of a latch that is itself the condition for the RESET) — HIGH.
- SR blocks where the S1 (dominant set) input is tied to a non-safety-rated signal for a safety-rated latch — HIGH.

```
(* HIGH — SET has no reachable RESET in export *)
IF FaultDetected THEN
    FaultLatch (S := TRUE, R1 := FALSE);
END_IF;
(* FaultLatch.Q will remain TRUE forever — operator has no reset path *)
```

Correct pattern:

```
FaultLatch(
    S  := FaultDetected,
    R1 := OperatorReset AND NOT FaultDetected
);
```

### Step 5 — Memory-write race audit

Search for any output bit, memory flag (`%M`, `%MW`, internal variable), or output coil address that appears on the left-hand side (assignment target, coil address) in more than one rung, network, or POU within the same task scan.

- Multiple writers to the same address — HIGH (last scan position wins; behavior changes silently when rungs are reordered or POUs are added).
- Output written in both a periodic task and an event-driven interrupt task — HIGH (non-deterministic; interrupt preemption creates a race).

Document the addresses, the rung or line numbers where the conflict occurs, and the task priority if available.

### Step 6 — Forced I/O and commissioning override audit

Scan the exported file for:
- Force table entries, force lists, or any tag marked with a force flag in the export format (e.g., `Force="1"` in L5X, `%IX0.0 := TRUE (*FORCED*)` annotations).
- Debug constants or literal-TRUE inputs substituted for field sensor addresses.
- Comments containing `//FORCED`, `(* DEBUG *)`, `TODO: remove`, `commissioning`, or similar.

Any force or commissioning override found in a production export — HIGH. Forces suppress the live field signal; the control loop no longer sees the physical process.

### Step 7 — Interlock bypass and maintenance override audit

Identify every maintenance mode, bypass, or inhibit bit that disables or overrides a protective interlock.

For each bypass:
- Confirm a time-limit timer (TON) in logic resets the bypass after a defined interval — if absent, HIGH.
- Confirm gating by a supervisor key-switch input, a safety-rated hardware input, or a logged credential acknowledgment — if absent, HIGH.
- Confirm the bypass state is annunciated to the operator and historian — if absent, MEDIUM.

```
(* HIGH — indefinite bypass with no time limit, no key gate, no annunciation *)
IF MaintenanceBypassBit THEN
    HighTempTrip := FALSE;
END_IF;
```

Correct pattern:

```
(* Time-limited, key-gated bypass with annunciation *)
MaintenanceTimer(IN := MaintenanceBypassBit AND KeySwitchIn, PT := T#15m);
IF MaintenanceTimer.Q THEN
    MaintenanceBypassBit := FALSE;  (* auto-expire *)
END_IF;
HighTempTrip_Active := HighTempTrip AND NOT (MaintenanceBypassBit AND KeySwitchIn);
BypassAnnunciation := MaintenanceBypassBit;
```

### Step 8 — Timer determinism and watchdog audit

**8a. Scan-count timers — HIGH**

Flag any timer pattern that increments a counter every scan and compares to a literal count rather than using a real-time-based function block (IEC 61131-3 TON, TOF, TP with a PT operand in time literals).

```
(* HIGH — scan-count "timer"; breaks when scan time changes *)
ScanCounter := ScanCounter + 1;
IF ScanCounter >= 500 THEN  (* intended: 500 scans * assumed 10ms = 5s *)
    ScanCounter := 0;
    TimeoutAction();
END_IF;
```

Correct pattern:

```
(* Real-time timer — deterministic regardless of scan load *)
DelayTimer(IN := TriggerCondition, PT := T#5s);
IF DelayTimer.Q THEN
    TimeoutAction();
END_IF;
```

**8b. Watchdog and communications-loss handling — HIGH if absent**

Confirm the program drives a watchdog output (toggling bit or heartbeat write) and that a defined default output state is explicitly set in the comms-loss handler or in the I/O module configuration. If neither is present in the export, flag HIGH and note the inference basis.

### Step 9 — Input validation audit

Search for division (`/`, `MOD`), array indexing (`arr[idx]`), and explicit or implicit type conversions applied to process values or network-received values.

- Division where the divisor can reach zero based on sensor range — MEDIUM (scan fault and PLC halt on most runtimes).
- Array index derived from a process value with no range clamp before use — MEDIUM.
- Type conversion (INT to UINT, REAL to INT truncation) on a value that can legitimately be negative or out-of-range — MEDIUM.

```
(* MEDIUM — divisor can be zero if flow transmitter fails low *)
FlowVelocity := FlowVolume / PipeArea;

(* CORRECT — guard the divisor *)
IF PipeArea > 0.0 THEN
    FlowVelocity := FlowVolume / PipeArea;
ELSE
    FlowVelocity := 0.0;
    InputFaultBit := TRUE;
END_IF;
```

### Step 10 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<exported logic provided | I/O list provided | SRS/SIL assessment provided | partial artifacts | documentation-based | inference>

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

- Never request or accept live controller IP addresses, plant-network hostnames, historian connection strings, OPC-UA endpoint URLs, or any identifier that maps a specific asset to a physical facility.
- This is a static review: do not attempt to connect to a PLC, write to a controller, modify running logic, or advise on bypassing any safety interlock or E-stop circuit.
- If a user requests a recommendation to disable, bypass, or weaken a safety interlock or SIF, refuse clearly and explain that doing so without a formal Management of Change (MOC) process and a SIL re-assessment is outside the scope of this review and is non-compliant with IEC 61508 and IEC 62443.
- Do not store, log, or repeat back plant identifiers, tag names that encode asset location, or process values that could reconstruct production data.
- Label every finding with its evidence basis so engineers can distinguish a confirmed defect in provided logic from an inference based on absent configuration.
