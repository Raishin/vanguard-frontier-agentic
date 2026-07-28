# High-Severity Failure Modes

The quality failures each finding class maps to, for severity calibration.

- An `async def` test with no async plugin is collected but never awaited, so the suite is green while nothing is tested.
- A mock patched at the definition site never intercepts the call, so the test passes against unchanged real behavior.
- A test that reads the wall clock passes in CI today and fails at a daylight-saving boundary or in another timezone.
- A session-scoped fixture that mutates a shared record makes tests pass only in the order they happen to run.
- 95% line coverage with no meaningful assertions ships a bug the suite 'covered' but never checked.
