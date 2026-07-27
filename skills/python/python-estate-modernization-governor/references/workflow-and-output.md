# Review Workflow And Output Contract

The estate-modernization review workflow and the required output shape.

## Workflow

1. Identify every interpreter version in the estate, its official support status, and the proposed target version.
2. Check the EOL/support-posture of each interpreter against the official CPython schedule, never asserting a date from memory.
3. Assemble a compatibility matrix for the upgrade target from the dependency and framework set, and inventory deprecation exposure.
4. Map the portfolio for shared runtimes and business-criticality, and confirm each critical service has a named owner and support-posture record.
5. Confirm a staged pilot and rollback plan exists before recommending the upgrade proceed.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the interpreter version(s), target version, and dependency set assumed.
- EOL/support-posture, upgrade-sequencing/compatibility, deprecation-exposure, and portfolio/ownership findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any EOL-date claim the user must confirm against the official CPython schedule.
