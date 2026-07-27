# Review Workflow And Output Contract

The developer-tooling review workflow and the required output shape.

## Workflow

1. Identify the linter, type-checker, test runner, tox/nox matrix, build backend, and CI configuration in use.
2. Check whether each gate actually fails the build on a real defect, and flag blanket ignores/exclusions.
3. Check type-checker strictness and linter rule selection for correctness coverage, not just style.
4. Check the CI gate set's coverage and branch/fork scope, and the tox/nox matrix against declared supported versions.
5. Check the build backend and package discovery for layout correctness, and confirm pre-commit mirrors the CI gates; record every claim needing a real run to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the tooling stack assumed.
- Gate-enforcement/strictness, linter-correctness/CI-coverage, and build-backend/feedback-loop findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any actual lint/type/test/CI-behavior claim the user must confirm by running the pipeline.
