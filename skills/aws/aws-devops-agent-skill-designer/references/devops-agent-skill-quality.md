# DevOps Agent Skill Quality Guide

Use this reference when designing or reviewing AWS DevOps Agent skills, learned skills, incident investigation workflows, tool-use best practices, frontmatter descriptions, and operational output contracts.

## What people get wrong

The lazy story is:

> A good skill is a detailed runbook pasted into SKILL.md.

Wrong. A useful DevOps Agent skill triggers correctly, loads progressively, constrains tool use, separates evidence from inference, and has deterministic evaluation criteria.

Common bad assumptions:

- More instructions mean better behavior.
- Broad descriptions improve triggering.
- Learned skills can safely encode environment-specific secrets or account details.
- Tool guidance is safe if it says “read-only” once.
- RCA/mitigation/evaluation can live in one vague workflow.
- A skill is done without tests or example failure cases.

## Skill-design failure modes

- Frontmatter description is too broad and steals unrelated tasks.
- SKILL.md bloats with reference material instead of progressive disclosure.
- Tool instructions allow mutation without approval gates.
- Incident workflow claims root cause from weak evidence.
- Learned skill contains internal role names, customer identifiers, account IDs, or secret-bearing commands.
- Output contract lacks verdict, evidence level, blockers, safe next actions, and open questions.

## Minimum safe workflow

1. Define target agent type, use cases, non-use cases, and trigger boundaries.
2. Keep SKILL.md lean: purpose, when to use, operating rules, references, and response minimum.
3. Move domain depth into references with failure modes, workflow, verification targets, and pushback criteria.
4. Specify allowed tools and mutation boundaries; require approval for live changes.
5. Add eval criteria before editing: trigger fit, safety gates, source grounding, output contract, and no internal identifiers.
6. Validate schema, manifests, links, and examples.
7. Document what evidence the skill can and cannot prove.

## Verification targets

- AWS DevOps Agent skill structure, SKILL.md frontmatter, references, and packaging constraints
- trigger description clarity, anti-triggers, role targeting, and allowed tools
- learned-skill content for topology, tool patterns, automatic updates, activation/deactivation implications
- safety checklist, source grounding, live evidence labels, mutation gates, and final response contract
- eval files, deterministic graders, schema validation, manifest validation, and no-secret/no-internal-identifier scans
- example incident or workflow prompts that prove the skill triggers only where intended

## When to push back

Push back if the user asks to:

- paste a giant runbook into SKILL.md
- include account IDs, internal role names, tokens, or environment-specific secrets
- make a skill trigger for every AWS problem
- allow remediation tools without approval and rollback rules
- skip evals because the skill “reads well”
- mix unrelated domains into one agent skill for convenience
