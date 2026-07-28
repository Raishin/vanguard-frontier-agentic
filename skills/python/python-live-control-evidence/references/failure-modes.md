# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- Sealing evidence to an unapproved, unmanaged destination leaves it outside retention and access controls, undermining any later audit.
- Treating hashed evidence integrity as proof of accuracy hides that the underlying observation itself may have been wrong.
- Asserting a control is effective from sealed evidence alone skips the testing step that would have caught a control that stopped operating.
- Sealing an unredacted secret or personal field into the evidence store creates a new exposure the evidence process was supposed to prevent.
- Missing retention/legal-hold enforcement lets evidence needed for a later investigation expire or be overwritten.
