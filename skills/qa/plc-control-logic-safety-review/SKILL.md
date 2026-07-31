---
name: plc-control-logic-safety-review
description: Use this skill when reviewing exported PLC program logic (Ladder Diagram, Structured Text, Function Block Diagram, or Sequential Function Chart) for safety and reliability defects. Trigger when a user provides exported IEC 61131-3 program source, an I/O list, a safety requirements spec, a SIL assessment, or asks whether their PLC logic has a safe state, a correct E-stop implementation, unresolved latches, forced I/O, or interlock bypass risks. This is OT/ICS — defects injure people or destroy equipment. The skill performs static review only; it never connects to a live PLC, never writes to a controller, and never advises modifying running logic or bypassing a safety function.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-17"
  category: resilience
  lifecycle: experimental
---

# PLC Control Logic Safety Review

## Purpose
This skill statically reviews exported IEC 61131-3 PLC program logic for safety and reliability defects before that logic reaches a live controller. In operational technology (OT) and industrial control systems (ICS), a logic defect that would be a bug in enterprise software can injure people, destroy equipment, or trigger a process shutdown with downstream consequences measured in hours of downtime or lives at risk. The review covers E-stop and safety function implementation, output fail-safe behavior, latch integrity, memory-write races, forced I/O left in production exports, interlock bypass governance, timer determinism, watchdog coverage, and input-validation gaps. It never touches a live controller, never modifies logic, and never advises weakening a safety function.

## Lean operating rules
- E-stop or safety function implemented in standard-PLC software logic instead of a hardwired, fail-safe safety relay or a safety-rated PLC/SIL-rated controller — CRITICAL (violates IEC 60204-1 / IEC 61508; a scan fault, firmware bug, or communications loss can defeat a software-only E-stop).
- An output coil that can be energized but has no reachable path to de-energize on fault, communications loss, or PLC STOP/mode change — CRITICAL (remote I/O modules may hold last state on network dropout; a stuck energized output can sustain hazardous motion or heat).
- A latch (SET coil, SR block, retentive coil) with no reachable RESET anywhere in the program, or a RESET gated behind a condition that can never evaluate TRUE — HIGH (output permanently energized; no operator recovery path without forcing).
- The same output bit, memory flag, or output coil address written by more than one rung, task, or Program Organization Unit (POU) within a single scan cycle — HIGH (last-write-wins race; behavior is non-deterministic and scan-order dependent).
- Forced I/O values or commissioning force-tables present in the exported program file — HIGH (commissioning state or debug override shipped to production; control loop sees forced value, not the live field sensor).
- An interlock bypass or maintenance-override bit with no time limit enforced in logic and no supervisor key-switch, credential gate, or logged acknowledgment — HIGH (silent, indefinite defeat of a safety interlock; not compliant with IEC 62443-3-3 SR 2.12 and typical SIF management procedures).
- Timer or counter logic whose numerical correctness depends on scan-cycle duration rather than an explicit, hardware-referenced real-time base (e.g., incrementing a counter in every scan and comparing to a literal count instead of using a TON/TOF with a PT in milliseconds) — HIGH (breaks when scan time changes under load, program additions, or firmware upgrade).
- No watchdog output and no defined fail-safe default output state documented or implemented for communications loss with remote I/O or a supervisory system (SCADA/DCS) — HIGH (silent loss of supervision; outputs may hold indefinitely in an unsafe energized state).
- Division, array indexing, or type conversion applied to a process value or network-received value that has not been validated for range — MEDIUM (integer divide-by-zero or out-of-bounds array access causes a scan fault and PLC halt in most runtimes, transitioning to a potentially undefined output state).
- Rung, network, or task execution priority that creates a correctness dependency undocumented in comments or a technical note — MEDIUM (maintainers and future modifications may break the assumed order silently).
- Do not recommend disabling, bypassing, or weakening any safety interlock, E-stop circuit, or SIF — refuse the request and explain the IEC 61508 and IEC 60204-1 basis for the refusal.
- Label every finding with its evidence basis: exported logic provided, I/O list provided, documentation-based, or inference from absent configuration.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Safety function and E-stop implementation findings (hardwired vs. software; SIL-rated controller vs. standard PLC)
- Output fail-safe and de-energization path analysis
- Latch/SET-RESET integrity findings
- Memory-write race findings (multiple writers to same address)
- Forced I/O and commissioning override findings
- Interlock bypass governance findings
- Timer and watchdog determinism findings
- Input validation findings (division, array, type conversion on unvalidated values)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
