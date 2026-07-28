# Mocks, Fixtures, And Determinism

Correct patch targets, fixture isolation, and controlling time/randomness/environment.

- unittest.mock's `patch` replaces a name in a specific namespace, so it must target where the object is looked up (the module under test importing it), not where it is originally defined — patching the definition site leaves the call unintercepted.
- pytest's `monkeypatch` fixture safely sets and automatically undoes attributes, dict items, and environment variables for the duration of a test, which is the correct way to isolate environment and injected dependencies without leaking into other tests.
- Determinism requires controlling ambient inputs: inject or freeze the clock, fix random seeds, and isolate filesystem/network at a seam, because a test that reads real time, unseeded randomness, or live I/O is inherently flaky.

## Sources

- https://docs.python.org/3/library/unittest.mock.html
- https://docs.pytest.org/en/stable/how-to/monkeypatch.html
