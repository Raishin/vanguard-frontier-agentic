# Failure Modes This Role Prevents

The concrete production incidents this role's change-plan discipline is designed to prevent.

- A change is executed with no diff on record, so no one can confirm afterward what actually changed.
- An approval granted for one target is reused after the target changed, executing an action no one actually reviewed.
- A rollback is improvised during an incident because no rollback procedure was pre-approved in the plan.
- Verification criteria are vague or absent, so a broken change is marked 'done' without anyone confirming it worked.
- A planning tool is given standing production credentials and becomes a privileged target for compromise, when it never needed to execute anything.
