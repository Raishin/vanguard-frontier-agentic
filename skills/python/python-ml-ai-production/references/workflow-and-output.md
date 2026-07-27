# Review Workflow And Output Contract

The ML/AI production review workflow and the required output shape.

## Workflow

1. Identify the ML framework, the model-persistence format, and every point where a feature is computed on both the training and serving paths.
2. Check the model artifact's provenance and trust boundary, and confirm training-serving feature parity.
3. Check the preprocessing pipeline for feature/data leakage (fit-on-train-only, no target/future features).
4. Check reproducibility (seed, pinned versions, recorded data snapshot) and that evaluation matches deployment conditions.
5. Check batch-vs-online consistency and model/prompt provenance, and record every metric or drift claim needing a real evaluation to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the ML framework and artifact format assumed.
- Model-artifact/skew, leakage/reproducibility, evaluation/batch-vs-online, and provenance findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any metric or drift claim the user must confirm against a real evaluation run.
