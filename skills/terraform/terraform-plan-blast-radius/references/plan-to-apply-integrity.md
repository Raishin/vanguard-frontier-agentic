# Plan-To-Apply Integrity

Whether the plan under review is the plan that runs, and what narrowing the scope actually costs.

- A plan saved with `-out` records the full configuration, the planned change values, and the plan options, and applying that file applies exactly those changes — this is the only case in which reviewing a plan constrains what apply does.
- Running apply without a saved plan file re-plans against current remote state, so the changes applied can differ from the changes reviewed whenever remote state moved in between; a review of such a plan is advisory and must say so.
- A saved plan file stores sensitive values in cleartext, which makes the plan artifact itself a secret: it must be treated with the same handling as state, and it must not be passed through an artifact store, a CI log, or a chat message that the state itself would not be passed through.
- `-target` is documented for exceptional circumstances only, such as recovering from a mistake or working around an engine limitation; routine use hides drift and leaves state inconsistent with configuration, because everything outside the target is neither planned nor applied.
- A targeted apply produces a state that no unrestricted plan has ever validated, so the next full plan is the first time anyone sees the combined result — the risk of targeting is deferred, not removed.
- `-replace=ADDRESS` forces replacement of a specific instance and is the supported successor to marking a resource tainted; it expresses the intent in the plan rather than as a hidden state flag, which is what makes it reviewable.
- `-refresh-only` produces a plan that reconciles state with remote objects without proposing configuration changes, which makes it the safe way to see drift before deciding what to do about it.
