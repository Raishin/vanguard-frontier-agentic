# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- Improvising a rollback mid-incident because it probably works turns one outage into two.
- Executing a rollback with no captured before-state leaves no reference to confirm the rollback actually restored the prior state.
- Running a rollback against the wrong target because the affected-target fingerprint was never confirmed extends the outage.
- Executing a rollback without confirmed rollback authority bypasses the approval the original change required.
- Self-attesting a rollback's success hides a rollback that did not actually restore service.
