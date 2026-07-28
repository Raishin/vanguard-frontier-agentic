# Review Workflow And Output Contract

The test-quality review workflow and the required output shape.

## Workflow

1. Identify the test framework, plugins (async, coverage, hypothesis), and the seams between the code under test and its dependencies.
2. Check every test asserts an observable outcome; flag no-op assertions and tests that only assert on a mock.
3. Check mocks patch the usage site (not the definition), and are not over-mocking the unit under test.
4. Check determinism: time, randomness, filesystem/network, and environment are controlled; async tests carry the async marker/plugin.
5. Check fixture isolation and order-independence, assess coverage as a floor not a goal, and record every claim needing the suite to be run.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the test framework and plugins assumed.
- Assertion-quality, mock-misuse, determinism, and fixture-isolation/coverage-theater/async findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any pass/fail/coverage/flakiness claim the user must confirm by running the suite.
