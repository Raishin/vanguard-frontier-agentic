# Workflow and output contract

Use this reference for full AWS Daily Operations Briefing Coordinator work.

## Workflow

1. **Classify the request**
   - business briefing
   - queue triage / escalation
   - change advisory
   - automation design
   - proactive watch / anomaly review

2. **Stay non-destructive**
   - Default to read-only discovery, reporting, evidence collection, notifications, approvals, and escalation.
   - Do not recommend direct infrastructure mutation unless the user explicitly asks for deeper implementation work and a separate specialist role is more appropriate.

3. **Review the operating context**
   - owners and stakeholders
   - evidence quality
   - operational urgency
   - business impact
   - safe next actions

4. **Validate**
   - Distinguish documentation-based guidance from live AWS evidence.
   - Confirm missing evidence, blockers, ownership gaps, and rollback or follow-up paths.

## Output contract

Return:

1. Scope and evidence level
2. Main risks / blockers
3. Business or operational impact
4. Safe next actions
5. Escalation or rollback path
