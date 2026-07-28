# Test-Quality Review Checklist

The per-concern checklist applied to every test-suite review.

- Assertions: every test asserts an observable outcome; no `assert True`, no assertion only that a mock was called.
- Async: every `async def test_*` carries the async marker/plugin so the body is actually awaited.
- Mocks: patched at the usage site, not the definition; the test verifies behavior, not the mock's own return.
- Determinism: time, randomness, filesystem/network, and environment are injected/frozen/isolated.
- Isolation: fixtures are function-scoped or reset; tests pass in any order.
- Coverage: treated as a floor; risky branches (error paths, edges) are asserted, not merely executed.
