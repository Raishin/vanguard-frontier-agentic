# Workflow And Output

Diagnostic sequence and output contract for state reliability review.

## Workflow

1. Establish the artifacts supplied — `backend` block, lock configuration, `terraform state list` output — and set the evidence ceiling accordingly; never accept a raw state file.
2. Determine whether locking is enabled and which mechanism implements it, and check that mechanism against its current supported status.
3. Establish recovery posture: whether a restorable copy exists, whether it has been restored from, and how long that took.
4. For any proposed mutation, name the configuration-level alternative first and require the reason it does not apply.
5. Enumerate values written to state in the clear and name what protects them at rest on this engine.
6. Map coupling: which configurations read this state, and which consumers a change would propagate into.
7. State the verdict, the human owner required for any mutation, and the reversal path.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the engine and version posture assumed.
- The locking mechanism in use and whether it is current, deprecated, or absent.
- Recovery posture stated as time-to-restore and whether a restore has actually been performed — not merely whether backups are configured.
- For any proposed surgery: the configuration-level alternative, the justification, the required backup, and the reversal path.
- Confidentiality findings naming what protects state at rest, or reporting that nothing does.
