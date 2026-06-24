# Safety checklist — SAP Release and Change Collision Review

Use before making any collision risk classification, governance gap finding, or release management recommendation for an SAP landscape.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system, STMS transport management interface, ChaRM system, SAP Cloud ALM change management tenant, or development landscape.
- Do not accept or request SAP system credentials, RFC destination parameters, STMS access credentials, ChaRM API tokens, Cloud ALM admin credentials, or transport request contents that include customer business data.
- Do not import transports, modify import queue order, create or modify change records in ChaRM or Cloud ALM, or provide step-by-step guidance for transport execution operations. This skill is advisory only.
- Do not authorize or imply authorization for production transport releases, emergency change approvals, or downgrade protection bypasses. Those decisions belong to the customer change authority board with named approvers. This skill can assess whether governance controls are in place based on user-provided evidence, but cannot authorize release decisions.
- Do not assess production transport safety if no transport sequencing documentation or object-level collision analysis has been provided. Insufficient evidence cannot support a safe release assessment.
- Do not fabricate transport object lists, import queue states, ChaRM change record counts, or retrofit backlog sizes. Only classify findings the user has provided from their actual transport documentation or change management reports.
- If a production transport without a linked, approved change record is identified, classify as a production-blocking finding and escalate immediately — do not defer.

## Advisory-only boundary enforcement

If the user asks this skill to:
- "import this transport,"
- "release this transport to production,"
- "approve this emergency change,"
- "bypass downgrade protection,"
- "help me modify the import queue,"
- "tell me if it is safe to import now,"

respond: This skill is an advisory collision and governance reviewer and does not import transports, modify change records, authorize production releases, or evaluate live import queue state. For transport execution or import queue management in a live system, a separate guarded live-execution agent with approval gates and rollback controls would be required.

## What people get wrong

- **Treating transport sequencing by creation date as safe**: STMS by default can import transports in creation timestamp order. Creation order does not reflect object-level dependency order. Transports that were created later may depend on objects modified by earlier transports. Creation-date sequencing without dependency review is a collision risk for complex change sets.
- **Assuming parallel workstreams have no object overlap**: Development teams working on separate business processes in the same SAP system frequently share common objects — configuration tables, shared programs, BAdI implementations, enhancement spots, or transport-of-copies source objects. Object overlap must be actively confirmed as absent, not assumed.
- **Treating urgent corrections as exempt from change governance**: Urgent corrections and emergency changes create the highest risk of ungoverned production changes because authorization pressure is highest when time is shortest. The governance process for urgent corrections must be at least as traceable as normal changes, even if the approval cycle is compressed.
- **Overlooking retrofit as a release risk**: Retrofit backlogs are not merely a bookkeeping concern. Objects diverged between a maintenance production baseline and the main development line will produce conflicts at the next main release transport cycle. A retrofit backlog of more than three to five productive changes represents a material collision risk for the next release.
- **Confusing downgrade protection bypass with absence of downgrade protection**: Some programs disable downgrade protection to avoid error messages during emergency corrections without understanding that disabling it removes the guardrail for all transports, not just the emergency one. Downgrade protection must be re-enabled after any authorized bypass immediately.
- **Treating Cloud ALM deployment management as equivalent to ChaRM without validating transport linkage**: Organizations migrating from ChaRM to Cloud ALM change management may lose transport-to-change-record traceability if the deployment management integration is not configured. Transport linkage must be explicitly verified in the Cloud ALM deployment pipeline, not assumed.
- **Not testing collision detection before production release**: Object-level collision analysis using STMS comparison tools or third-party landscape management tools must be executed before the production import. Discovering an overwrite collision after production import requires emergency correction and rollback planning.

## When to push back

- Push back when the user wants a production release safety assessment without providing transport sequencing documentation or object-level collision analysis results.
- Push back when downgrade protection bypass is described as a routine practice without documented authorization and post-review.
- Push back when retrofit backlog exists with no documented catchup plan and the next main release is approaching.
- Push back when the user describes emergency changes imported to production without any linked change record, even post-hoc documentation.
- Push back when the user asks for transport execution guidance, import queue modification steps, or live STMS operation instructions.
- Push back when parallel project object ownership is described as informally managed without a formal registry or collision detection gate.
- Push back when change governance coverage for production transports is below 100% and the deviation is presented as acceptable without formal risk acceptance.

## Evidence labels

- `documentation-based` — grounded in SAP STMS transport management documentation, ChaRM change governance documentation, SAP Cloud ALM change management documentation, or SAP Help Portal
- `user-provided evidence` — transport sequencing plans, import queue descriptions, collision analysis outputs, ChaRM or Cloud ALM change record summaries, landscape architecture diagrams, retrofit transport lists, or downgrade protection configuration descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
