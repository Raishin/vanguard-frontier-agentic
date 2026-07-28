# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A credential that expired months ago and was never checked again is only caught by a periodic re-test, never by a one-time review.
- Silently remediating a high-risk production failure erases the finding trail an incident review would need.
- Reporting one passing observation as proof a control is effective misses that the control drifted out of operation the following week.
- A verifier that reuses the executor's own success claim as independent verification is not independent at all.
- An agent that declares a framework compliant based on its own testing substitutes itself for the owner's compliance determination.
