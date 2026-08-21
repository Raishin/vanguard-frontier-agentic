# Workflow And Output

Review sequence and output contract for module-contract review.

## Workflow

1. Establish the module's callers and its published version, or record that neither was supplied and label downstream findings accordingly.
2. Enumerate the input surface and test each variable for a type constraint and a `validation` block that rejects the values the module cannot handle.
3. Enumerate outputs and separate deliberate contract from leaked implementation detail.
4. Classify every input, output, and resource-address change in the diff as breaking or non-breaking for existing callers.
5. Place each invariant in the right construct and flag any that cannot fire where it was written.
6. Ask whether the module should exist: name the platform module it duplicates, or the input that would remove the need for a fork.
7. State a verification-adequacy verdict proportionate to what the module provisions.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the engine and version posture assumed.
- An explicit breaking / non-breaking classification for every input, output, and resource-address change.
- Input-surface, output-contract, and invariant-placement findings, each with an evidence-basis label.
- A verification-adequacy verdict relative to the module's blast radius, and any `moved` blocks the change requires.
