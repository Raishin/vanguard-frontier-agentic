export const meta = {
  name: 'agentic-delegation',
  description: 'Executable form of the agentic-delegation doctrine: Haiku recon sweeps, Context7-backed adversarial verification of external claims, Sonnet spec-driven writing, and a Haiku gate run — with the orchestrator keeping architecture, security edits, and the commit.',
  whenToUse: 'A multi-step change in this repo that decomposes into cheap parallel work plus a little judgment: mapping an unfamiliar area, verifying vendor claims before encoding them, writing bulk assets against a file-scoped spec, then running the gate suite. Pass args as { objective, recon: [{question, where}], claims: [{id, claim, libraryId, encodedAt}], specs: [{label, files, spec, conventions, acceptance}], runGates }. Every field is optional — supply only the phases the task needs.',
  phases: [
    { title: 'Recon', detail: 'parallel Haiku Explore sweeps, file:line citations required' },
    { title: 'Verify', detail: 'Context7 retrieval per claim, then an adversarial refuter' },
    { title: 'Implement', detail: 'Sonnet writes to a file-scoped spec, then a diff reviewer' },
    { title: 'Gates', detail: 'Haiku runs the repo gate suite in dependency order' },
  ],
}

// ---------------------------------------------------------------------------
// Doctrine encoded here (see .claude/skills/agentic-delegation/SKILL.md):
//
//   * Haiku explores and runs gates. It never plans, decomposes, or accepts work.
//   * Sonnet writes bulk assets against an exact file-scoped spec.
//   * The orchestrator keeps architecture, security-sensitive edits, final
//     verification, and the commit — so NO stage here commits anything.
//   * A delegate's self-report is not verification. Every writing stage is
//     followed by a reviewer that reads the diff, and the gate stage reports raw
//     failure output rather than a paraphrase.
//   * Caps are logged, never silent: a truncated work-list says so.
//
// Determinism: no wall-clock, no randomness. Behaviour changes only when the
// committed script or the passed args change.
// ---------------------------------------------------------------------------

const A = args || {}
const OBJECTIVE = A.objective || 'unspecified objective'

// Agent budget. The session guideline for this repo is "keep workflows under 15
// agents"; these caps hold the worst case at 14 and every drop is logged.
const MAX_AGENTS = 14
const CAPS = { recon: 4, claims: 4, refutes: 2, specs: 2 }

let spent = 0
function take(n, what) {
  const room = Math.max(0, MAX_AGENTS - spent)
  const got = Math.min(n, room)
  if (got < n) log(`CAP: ${what} truncated ${n} -> ${got} (agent budget ${MAX_AGENTS} reached)`)
  spent += got
  return got
}

function capped(list, cap, what) {
  const src = Array.isArray(list) ? list : []
  if (src.length > cap) log(`CAP: ${what} limited to ${cap} of ${src.length} supplied`)
  const wanted = src.slice(0, cap)
  return wanted.slice(0, take(wanted.length, what))
}

// The two rules every delegate carries, regardless of stage.
const NO_COMMIT =
  'DO NOT run git commit, git push, git add, or any other git write command. DO NOT open a pull ' +
  'request. The orchestrator owns the commit.'
const NO_SECRETS =
  'DO NOT request, echo, or record any credential, token, private key, account identifier, or ' +
  'customer data. Environment variable NAMES only.'

// ---------------------------------------------------------------------------
// Schemas
// ---------------------------------------------------------------------------

const RECON_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['question', 'findings', 'checkedCount'],
  properties: {
    question: { type: 'string' },
    checkedCount: { type: 'integer', description: 'How many files or symbols were examined' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['path', 'line', 'finding'],
        properties: {
          path: { type: 'string', description: 'Repo-relative file path' },
          line: { type: 'integer' },
          finding: { type: 'string' },
        },
      },
    },
    gaps: {
      type: 'array',
      items: { type: 'string' },
      description: 'What the sweep could NOT establish. Empty array if nothing.',
    },
  },
}

const CLAIM_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['id', 'verdict', 'evidence'],
  properties: {
    id: { type: 'string' },
    verdict: { enum: ['AGREE', 'DISAGREE', 'PARTIAL', 'UNVERIFIABLE'] },
    evidence: { type: 'string', description: 'Quoted doc snippet plus the source URL' },
    discrepancy: { type: 'string', description: '"none" when the verdict is AGREE' },
  },
}

const REFUTE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['id', 'refuted', 'reasoning'],
  properties: {
    id: { type: 'string' },
    refuted: { type: 'boolean', description: 'true if the claim does NOT hold as encoded' },
    reasoning: { type: 'string' },
    missingContext: {
      type: 'string',
      description: 'Documented material the claim omits that would change a reader decision',
    },
  },
}

const REVIEW_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['label', 'accepted', 'issues'],
  properties: {
    label: { type: 'string' },
    accepted: { type: 'boolean' },
    issues: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['path', 'issue'],
        properties: {
          path: { type: 'string' },
          issue: { type: 'string' },
        },
      },
    },
  },
}

const GATE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['allGreen', 'results'],
  properties: {
    allGreen: { type: 'boolean' },
    results: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['command', 'exit', 'pass'],
        properties: {
          command: { type: 'string' },
          exit: { type: 'integer' },
          pass: { type: 'boolean' },
          rawFailure: {
            type: 'string',
            description: 'Verbatim failing lines. Never a paraphrase. Empty when the gate passed.',
          },
        },
      },
    },
  },
}

// ---------------------------------------------------------------------------
// Phase 1 — Recon. Parallel Haiku Explore sweeps, one narrow question each.
// A barrier is correct here: the implementation spec is written against the
// union of what recon found, so nothing downstream can start on a partial map.
// ---------------------------------------------------------------------------

let recon = []
const reconTasks = capped(A.recon, CAPS.recon, 'recon sweeps')

if (reconTasks.length) {
  phase('Recon')
  log(`Recon: ${reconTasks.length} parallel Haiku sweep(s) for "${OBJECTIVE}"`)
  recon = (
    await parallel(
      reconTasks.map((r, i) => () =>
        agent(
          [
            'Read-only reconnaissance. ONE question only — do not broaden it.',
            '',
            `QUESTION: ${r.question}`,
            `WHERE TO LOOK (only these paths): ${r.where || 'the repository root'}`,
            '',
            'ACCEPTANCE CRITERIA: every finding carries a repo-relative path and an exact line',
            'number. A finding you cannot cite is not reported. Report what you checked as a',
            'count, and list anything the sweep could NOT establish under gaps — an empty gaps',
            'array is a claim that you looked and found nothing missing.',
            '',
            'DO NOT edit or write any file. DO NOT run npm, cargo, or any validation command.',
            'DO NOT look outside the paths above.',
            NO_COMMIT,
            NO_SECRETS,
          ].join('\n'),
          {
            label: `recon:${r.label || i + 1}`,
            phase: 'Recon',
            model: 'haiku',
            effort: 'low',
            agentType: 'Explore',
            schema: RECON_SCHEMA,
          },
        ),
      ),
    )
  ).filter(Boolean)

  const cites = recon.reduce((n, x) => n + (x.findings ? x.findings.length : 0), 0)
  const gaps = recon.flatMap((x) => x.gaps || [])
  log(`Recon complete: ${cites} cited finding(s), ${gaps.length} stated gap(s)`)
}

// ---------------------------------------------------------------------------
// Phase 2 — Verify. Context7 retrieval per claim, then an adversarial refuter.
// Pipelined: each claim refutes as soon as its own retrieval lands, so a slow
// claim never blocks a fast one. The refuter is the point of the phase — a
// verifier asked "is this right?" agrees far too easily.
// ---------------------------------------------------------------------------

let verified = []
const claims = capped(A.claims, CAPS.claims, 'claims to verify')

if (claims.length) {
  phase('Verify')
  log(`Verify: ${claims.length} claim(s) against Context7 primary sources`)

  let refuteRoom = take(Math.min(claims.length, CAPS.refutes), 'adversarial refuters')

  verified = (
    await pipeline(
      claims,
      (c) =>
        agent(
          [
            'Independent verification of a claim someone else encoded. Be adversarial: your job',
            'is to find the claims that are WRONG or INCOMPLETE, not to confirm them.',
            '',
            'TOOL SETUP: call ToolSearch with query "select:mcp__Context7__query-docs" to load the',
            'Context7 documentation tool, then query it.',
            `LIBRARY ID: ${c.libraryId || '(resolve the right one with mcp__Context7__resolve-library-id)'}`,
            '',
            `CLAIM ID: ${c.id}`,
            `CLAIM AS ENCODED: ${c.claim}`,
            c.encodedAt ? `ENCODED AT: ${c.encodedAt}` : '',
            '',
            'Run ONE narrowly scoped query, then judge. An AGREE verdict MUST quote the doc',
            'snippet that supports it together with the source URL — an AGREE with no quoted',
            'evidence is downgraded to UNVERIFIABLE. Do not soften a DISAGREE into a PARTIAL to',
            'be agreeable. Report "PARTIAL" when the claim is true but omits documented material',
            'that would change a reader decision.',
            '',
            'DO NOT edit or write any file. DO NOT run more than 3 Context7 queries.',
            NO_COMMIT,
            NO_SECRETS,
          ]
            .filter(Boolean)
            .join('\n'),
          {
            label: `verify:${c.id}`,
            phase: 'Verify',
            model: 'sonnet',
            effort: 'high',
            schema: CLAIM_SCHEMA,
          },
        ),
      async (v, c) => {
        if (!v) return null
        // Refute only the claims that came back clean — a DISAGREE or PARTIAL is
        // already a finding and does not need a second opinion to become one.
        if (v.verdict !== 'AGREE' || refuteRoom <= 0) return { claim: c, verdict: v, refute: null }
        refuteRoom -= 1
        const r = await agent(
          [
            'A previous reviewer judged the claim below as AGREE. Try to REFUTE that judgement.',
            '',
            'TOOL SETUP: call ToolSearch with query "select:mcp__Context7__query-docs", then query',
            `library ${c.libraryId || '(resolve it yourself)'}.`,
            '',
            `CLAIM ID: ${c.id}`,
            `CLAIM: ${c.claim}`,
            `PRIOR EVIDENCE: ${v.evidence}`,
            '',
            'Look specifically for adjacent documentation the first pass missed: a timeline, a',
            'constraint, an exception, a superseding recommendation, or an edition/region/version',
            'gate. Set refuted=true if the claim does not hold AS ENCODED — including when it is',
            'literally true but omits documented material that would change what a reader does.',
            'Default to refuted=false only when you searched and found nothing contradicting it.',
            '',
            'DO NOT edit or write any file. DO NOT run more than 3 Context7 queries.',
            NO_COMMIT,
            NO_SECRETS,
          ].join('\n'),
          {
            label: `refute:${c.id}`,
            phase: 'Verify',
            model: 'sonnet',
            effort: 'high',
            schema: REFUTE_SCHEMA,
          },
        )
        return { claim: c, verdict: v, refute: r }
      },
    )
  ).filter(Boolean)

  const bad = verified.filter(
    (x) => x.verdict.verdict !== 'AGREE' || (x.refute && x.refute.refuted),
  )
  log(`Verify complete: ${verified.length} checked, ${bad.length} need the orchestrator's attention`)
}

// ---------------------------------------------------------------------------
// Phase 3 — Implement. Sonnet writes to an exact file-scoped spec, then a
// reviewer reads the result. Pipelined so each spec reviews as soon as it is
// written. Security-sensitive and load-bearing edits are NOT delegated here —
// the doctrine keeps those with the orchestrator, so this stage is for bulk
// assets only and the spec is expected to say which files it may touch.
// ---------------------------------------------------------------------------

let implemented = []
const specs = capped(A.specs, CAPS.specs, 'implementation specs')

if (specs.length) {
  phase('Implement')
  log(`Implement: ${specs.length} file-scoped spec(s)`)

  const reviewRoom = take(specs.length, 'diff reviewers')
  let reviewsLeft = reviewRoom

  implemented = (
    await pipeline(
      specs,
      (s) =>
        agent(
          [
            'Implement exactly the specification below. Do not improvise scope.',
            '',
            `OBJECTIVE CONTEXT: ${OBJECTIVE}`,
            `SPEC: ${s.spec}`,
            '',
            `FILES YOU MAY TOUCH (exhaustive — nothing outside this list): ${(s.files || []).join(', ')}`,
            s.conventions ? `CONVENTIONS TO MIRROR: ${s.conventions}` : '',
            s.acceptance ? `ACCEPTANCE CRITERIA: ${s.acceptance}` : '',
            '',
            'When done, report the files you wrote and a one-line description of each change.',
            '',
            'DO NOT touch any file outside the list above. DO NOT run npm run validate, cargo',
            'test, or any other repo gate — the orchestrator runs those.',
            NO_COMMIT,
            NO_SECRETS,
          ]
            .filter(Boolean)
            .join('\n'),
          {
            label: `write:${s.label || 'spec'}`,
            phase: 'Implement',
            model: 'sonnet',
            effort: 'medium',
          },
        ),
      async (written, s) => {
        if (!written || reviewsLeft <= 0) return { spec: s, written, review: null }
        reviewsLeft -= 1
        const rev = await agent(
          [
            'Review what another agent just wrote. Its self-report is NOT evidence — read the',
            'actual files.',
            '',
            `SPEC IT WAS GIVEN: ${s.spec}`,
            `FILES IT WAS ALLOWED TO TOUCH: ${(s.files || []).join(', ')}`,
            s.acceptance ? `ACCEPTANCE CRITERIA: ${s.acceptance}` : '',
            '',
            'Check three things: (1) does the content satisfy the spec, (2) did it stay within',
            'the allowed files, (3) does it mirror the conventions of neighbouring files in the',
            'same directory. Set accepted=false if any check fails, and list each problem with',
            'its file path.',
            '',
            'SCOPING THE OUT-OF-SCOPE CHECK — read this carefully. The working tree may contain',
            'unrelated changes made concurrently by the orchestrator or by another delegate. A',
            'bare `git status --short` therefore shows files this writer never touched, and',
            'reporting those as scope violations is a false positive that wastes the',
            "orchestrator's time. Scope the check: run `git status --short -- <the allowed",
            "paths>` to confirm the allowed files changed, and treat a file as out of scope ONLY",
            'when the writer itself reported writing it or its content plainly implements this',
            'spec. When you are unsure whether a changed file belongs to this writer, say so in',
            'the issue text rather than asserting a violation.',
            '',
            'VERIFY THE CONTENT AGAINST REALITY, not against the spec alone. If the writer added',
            'tests, check that the fixtures they assert on actually match the real inputs this',
            'code will see in this repository — a test that passes against an invented fixture',
            'and fails against the committed file is worse than no test. Name any such gap.',
            '',
            'DO NOT fix anything you find — report it. DO NOT run repo gates.',
            NO_COMMIT,
            NO_SECRETS,
          ]
            .filter(Boolean)
            .join('\n'),
          {
            label: `review:${s.label || 'spec'}`,
            phase: 'Implement',
            model: 'sonnet',
            effort: 'medium',
            schema: REVIEW_SCHEMA,
          },
        )
        return { spec: s, written, review: rev }
      },
    )
  ).filter(Boolean)

  const rejected = implemented.filter((x) => x.review && !x.review.accepted)
  log(`Implement complete: ${implemented.length} spec(s), ${rejected.length} rejected by review`)
}

// ---------------------------------------------------------------------------
// Phase 4 — Gates. One Haiku pass over the repo's own suite, in dependency
// order. This is the only stage permitted to write a file, and only
// catalog/asset-integrity.json, and only after everything else is green —
// regenerating integrity before the other generators settle stales the manifest.
// ---------------------------------------------------------------------------

let gates = null

if (A.runGates) {
  phase('Gates')
  take(1, 'gate run')
  log('Gates: running the repo suite in dependency order')

  gates = await agent(
    [
      'Verification-only gate run from the repository root. Report results; fix nothing.',
      '',
      'Run these in EXACTLY this order, capturing each exit code:',
      '  1. npm run validate',
      '  2. npm run lint:spell',
      '  3. npx --yes markdownlint-cli2 "**/*.md" "#node_modules"',
      '',
      'STOP CONDITION: if any of 1-3 exits non-zero, stop immediately and report. Do not',
      'continue. A stale-manifest or model-policy failure is a regeneration decision that',
      'belongs to the orchestrator, not to you.',
      '',
      'ONLY IF 1-3 all exit zero: run `npm run asset-integrity:write`, then re-run',
      '`npm run validate` once more to confirm it still exits zero. Report both.',
      '',
      'For every failure, put the RAW failing lines in rawFailure verbatim. Never paraphrase,',
      'never write "some checks failed" — the orchestrator needs the actual error text to',
      'decide the next move.',
      '',
      'DO NOT edit any file except catalog/asset-integrity.json via the command above, and only',
      'after 1-3 pass. DO NOT attempt to fix a failure.',
      NO_COMMIT,
      NO_SECRETS,
    ].join('\n'),
    { label: 'gates', phase: 'Gates', model: 'haiku', effort: 'low', schema: GATE_SCHEMA },
  )

  log(gates && gates.allGreen ? 'Gates: ALL GREEN' : 'Gates: RED — see rawFailure')
}

// ---------------------------------------------------------------------------
// Return the orchestrator's decision surface, not a narrative. Everything the
// orchestrator must act on is separated from everything that merely passed.
// ---------------------------------------------------------------------------

return {
  objective: OBJECTIVE,
  agentsUsed: spent,
  recon: {
    sweeps: recon.length,
    findings: recon.flatMap((r) => r.findings || []),
    gaps: recon.flatMap((r) => r.gaps || []),
  },
  claims: {
    checked: verified.length,
    // The orchestrator's queue: anything not cleanly AGREE, or refuted on second look.
    needsAttention: verified
      .filter((x) => x.verdict.verdict !== 'AGREE' || (x.refute && x.refute.refuted))
      .map((x) => ({
        id: x.claim.id,
        encodedAt: x.claim.encodedAt || null,
        verdict: x.verdict.verdict,
        evidence: x.verdict.evidence,
        discrepancy: x.verdict.discrepancy || null,
        refuted: x.refute ? x.refute.refuted : null,
        missingContext: x.refute ? x.refute.missingContext || null : null,
      })),
    clean: verified
      .filter((x) => x.verdict.verdict === 'AGREE' && !(x.refute && x.refute.refuted))
      .map((x) => x.claim.id),
  },
  implementation: implemented.map((x) => ({
    label: x.spec.label || null,
    files: x.spec.files || [],
    accepted: x.review ? x.review.accepted : null,
    issues: x.review ? x.review.issues : [],
  })),
  gates,
  // Stated explicitly so a reader never mistakes a green workflow for a finished change.
  orchestratorStillOwns: [
    'reading the actual diff — no reviewer verdict here substitutes for it',
    'architecture decisions, security-sensitive edits, and load-bearing logic',
    'resolving every entry in claims.needsAttention',
    'the commit and the push',
  ],
}
