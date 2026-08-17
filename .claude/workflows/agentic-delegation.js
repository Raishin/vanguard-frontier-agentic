export const meta = {
  name: 'agentic-delegation',
  description: "Run this repo's delegation doctrine as a deterministic pipeline: Haiku recon, Context7-grounded verification of version-sensitive APIs, Sonnet spec-driven writing, adversarial verification, and a gate run — orchestrator keeps the commit.",
  whenToUse: "Multi-step work in vanguard-frontier-agentic that needs parallel reconnaissance, primary-source grounding of library/SDK claims via Context7, or bulk file authoring against exact specs. Pass args to scope it; with no args it runs a recon + gate health check.",
  phases: [
    { title: 'Recon', detail: 'Haiku Explore agents, one narrow question each, file:line citations required' },
    { title: 'Context7', detail: 'Resolve + query Context7 per library surface; CONFIRMED / CONTRADICTED / NOT_COVERED / NOT_AVAILABLE' },
    { title: 'Author', detail: 'Sonnet writers against exact file-scoped specs; no commits' },
    { title: 'Verify', detail: 'Independent refuters per authored change' },
    { title: 'Gates', detail: 'Repo gate suite with raw failure output; asset-integrity last' },
    { title: 'Synthesis', detail: 'Orchestrator-facing report' },
  ],
}

// ---------------------------------------------------------------- doctrine
//
// This encodes .claude/skills/agentic-delegation/SKILL.md as an executable
// pipeline. Three rules from that skill are load-bearing here and are repeated
// into every relevant agent prompt rather than assumed:
//
//   1. Haiku explores and runs gates; it never plans, decomposes, or accepts work.
//   2. Delegates write files; ONLY the orchestrator commits. No agent below runs
//      git commit, git push, or npm run validate as a substitute for the real
//      gate phase.
//   3. A delegate's self-report is not verification. Every authored change is
//      independently refuted in the Verify phase, and the caller still reads the
//      diff afterwards.
//
// The Context7 phase exists because this repo has already shipped two wrong
// MLflow API signatures that Databricks' own docs pages did not catch: service
// documentation describes features, library documentation pins call signatures.
// Absence from a Context7 result is NOT_COVERED (uncorroborated), never
// CONTRADICTED (disproven) — conflating the two causes correct content to be
// "fixed" into incorrectness.

const CITATION_RULE =
  'Every finding MUST carry a file:line citation (repo path plus line number) or a URL you actually retrieved. ' +
  'A finding without one is not actionable — omit it rather than reporting it unsourced. ' +
  'Do NOT edit any file. Do NOT run git, npm, or cargo. Read-only.'

const NO_COMMIT_RULE =
  'HARD CONSTRAINTS: touch ONLY the files listed in your spec — nothing else. ' +
  'Never run git add, git commit, git push, or npm run validate. ' +
  'You write files; the orchestrator reviews the diff and commits. ' +
  'If the spec is ambiguous, state the ambiguity in your return value rather than guessing and writing anyway.'

const RECON_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['question', 'findings'],
  properties: {
    question: { type: 'string' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['claim', 'citation'],
        properties: {
          claim: { type: 'string' },
          citation: { type: 'string', description: 'repo path:line, or a URL actually retrieved' },
        },
      },
    },
    gaps: { type: 'array', items: { type: 'string' } },
  },
}

const CONTEXT7_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['library', 'status', 'confirmed', 'contradicted', 'notCovered'],
  properties: {
    library: { type: 'string', description: 'the human name asked about' },
    libraryId: { type: 'string', description: 'the Context7-compatible id actually used, or empty' },
    version: { type: 'string', description: 'version Context7 reports, or empty' },
    status: { enum: ['OK', 'NOT_AVAILABLE'], description: 'NOT_AVAILABLE when the Context7 MCP tools could not be reached at all' },
    confirmed: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['claim', 'snippet'],
        properties: { claim: { type: 'string' }, snippet: { type: 'string' } },
      },
    },
    contradicted: {
      type: 'array',
      description: 'ONLY where Context7 shows something that positively conflicts with the claim. Never use for mere absence.',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['claim', 'actual', 'snippet'],
        properties: { claim: { type: 'string' }, actual: { type: 'string' }, snippet: { type: 'string' } },
      },
    },
    notCovered: {
      type: 'array',
      description: 'Claims Context7 returned no evidence for. Uncorroborated, NOT disproven.',
      items: { type: 'string' },
    },
  },
}

const AUTHOR_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['filesWritten', 'summary'],
  properties: {
    filesWritten: { type: 'array', items: { type: 'string' } },
    summary: { type: 'string' },
    deviations: { type: 'array', items: { type: 'string' }, description: 'anything done differently from the spec, and why' },
    ambiguities: { type: 'array', items: { type: 'string' } },
  },
}

const VERDICT_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['refuted', 'reason'],
  properties: {
    refuted: { type: 'boolean', description: 'true if the change is wrong, incomplete, or violates its spec' },
    reason: { type: 'string' },
    evidence: { type: 'string' },
  },
}

const GATE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['gates', 'allGreen'],
  properties: {
    allGreen: { type: 'boolean' },
    gates: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['name', 'passed'],
        properties: {
          name: { type: 'string' },
          passed: { type: 'boolean' },
          rawFailureOutput: { type: 'string', description: 'verbatim failure text, never a paraphrase' },
        },
      },
    },
  },
}

// ---------------------------------------------------------------- inputs

const a = args || {}
const task = a.task || 'Repo health check: no task supplied.'
const CAPS = { recon: 5, libraries: 5, specs: 6 }

const questionsIn = Array.isArray(a.questions) ? a.questions : []
const librariesIn = Array.isArray(a.libraries) ? a.libraries : []
const specsIn = Array.isArray(a.specs) ? a.specs : []

const questions = questionsIn.slice(0, CAPS.recon)
const libraries = librariesIn.slice(0, CAPS.libraries)
const specs = specsIn.slice(0, CAPS.specs)

// "No silent caps": say out loud what was dropped, so a truncated run is never
// mistaken for full coverage.
if (questionsIn.length > questions.length) log(`CAP: ${questionsIn.length - questions.length} recon question(s) dropped (cap ${CAPS.recon})`)
if (librariesIn.length > libraries.length) log(`CAP: ${librariesIn.length - libraries.length} library surface(s) dropped (cap ${CAPS.libraries})`)
if (specsIn.length > specs.length) log(`CAP: ${specsIn.length - specs.length} spec(s) dropped (cap ${CAPS.specs})`)

const defaultQuestions = [
  'Which validate:* gates does package.json wire into `npm run validate`, and what does each one actually assert? Cite package.json and each tests/ script by path:line.',
  'Which files in this repo are GENERATED and must never be hand-edited, and which script generates each? Cite the generator and its output paths by path:line.',
]
const reconQuestions = questions.length ? questions : defaultQuestions

log(`task: ${task}`)
log(`recon=${reconQuestions.length} context7=${libraries.length} specs=${specs.length}`)

// ---------------------------------------------------------------- 1. Recon
// Haiku, one narrow question each, run concurrently. This is a barrier only in
// the sense that the caller wants the map before authoring; Context7 does not
// depend on it, so the two run as independent branches below.

phase('Recon')
const reconWork = parallel(
  reconQuestions.map((q, i) => () =>
    agent(
      `Read-only reconnaissance in the vanguard-frontier-agentic repository.\n\n` +
        `ONE question, answer only this: ${q}\n\n` +
        `${CITATION_RULE}\n\n` +
        `You are the Haiku recon tier: you locate and summarise, you do not plan or propose changes.`,
      { label: `recon:${i + 1}`, phase: 'Recon', model: 'haiku', effort: 'low', agentType: 'Explore', schema: RECON_SCHEMA },
    ),
  ),
)

// ---------------------------------------------------------------- 2. Context7
// Independent of recon. Each library surface is resolved and queried on its own
// so one unavailable library cannot poison the others.

phase('Context7')
const context7Work = parallel(
  libraries.map((lib, i) => () => {
    const name = typeof lib === 'string' ? lib : lib.name
    const query = (typeof lib === 'object' && lib.query) || `current public API surface of ${name}`
    const claims = (typeof lib === 'object' && Array.isArray(lib.claims) ? lib.claims : [])
    return agent(
      `Verify version-sensitive library API claims using the Context7 MCP.\n\n` +
        `STEP 1 — load the tools. Call ToolSearch with the query "select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs". ` +
        `If those tools cannot be loaded or every call fails, return status "NOT_AVAILABLE" with empty arrays. ` +
        `NEVER fabricate MCP evidence and NEVER substitute WebFetch or your own memory for Context7 in this phase.\n\n` +
        `STEP 2 — resolve the library id for: ${name}\n` +
        `STEP 3 — query it about: ${query}\n\n` +
        (claims.length
          ? `STEP 4 — adjudicate each of these specific claims:\n${claims.map((c, n) => `  ${n + 1}. ${c}`).join('\n')}\n\n`
          : `STEP 4 — report the exact API names, signatures, and import paths Context7 shows for this surface.\n\n`) +
        `ADJUDICATION RULES — these matter more than coverage:\n` +
        `- "confirmed" requires a literal snippet from Context7 output. Quote it. No snippet, no confirmation.\n` +
        `- "contradicted" is ONLY for a claim Context7 positively shows to be different (e.g. it shows a different ` +
        `keyword argument name). Put what Context7 actually shows in "actual".\n` +
        `- "notCovered" is for a claim Context7 returned no evidence about. Context7 returns retrieved snippets, ` +
        `not a complete API inventory, so absence is UNCORROBORATED, never disproven. Misfiling absence as ` +
        `contradiction causes correct content to be "fixed" into incorrectness — do not do it.\n` +
        `- Report the exact library id and any version string Context7 states.\n\n` +
        `Do NOT edit any file. Do NOT run git or npm.`,
      { label: `context7:${name}`, phase: 'Context7', schema: CONTEXT7_SCHEMA },
    )
  }),
)

const [recon, context7] = await parallel([() => reconWork, () => context7Work])
const reconOk = (recon || []).filter(Boolean)
const c7 = (context7 || []).filter(Boolean)

const unavailable = c7.filter((r) => r.status === 'NOT_AVAILABLE').map((r) => r.library)
if (unavailable.length) log(`Context7 UNAVAILABLE for: ${unavailable.join(', ')} — those claims stay labelled unknown`)
const contradictions = c7.flatMap((r) => (r.contradicted || []).map((x) => ({ library: r.library, ...x })))
if (contradictions.length) log(`Context7 CONTRADICTED ${contradictions.length} claim(s) — these are defects, not style notes`)

// ---------------------------------------------------------------- 3/4. Author -> Verify
// pipeline(), not a barrier: spec B's refuter starts as soon as B is written,
// without waiting for spec A. Each authored change is refuted by an agent that
// did not write it.

const evidence =
  (reconOk.length
    ? `REPO EVIDENCE (from recon, each with a citation):\n` +
      reconOk.flatMap((r) => (r.findings || []).map((f) => `- ${f.claim}  [${f.citation}]`)).join('\n') +
      '\n\n'
    : '') +
  (c7.length
    ? `CONTEXT7 EVIDENCE:\n` +
      c7
        .map(
          (r) =>
            `- ${r.library} (${r.libraryId || 'unresolved'}${r.version ? ' ' + r.version : ''}) status=${r.status}\n` +
            (r.confirmed || []).map((x) => `    CONFIRMED: ${x.claim}`).join('\n') +
            (r.contradicted || []).map((x) => `    CONTRADICTED: ${x.claim} -> actually ${x.actual}`).join('\n') +
            (r.notCovered || []).map((x) => `    NOT_COVERED (uncorroborated, not disproven): ${x}`).join('\n'),
        )
        .join('\n') +
      '\n\n'
    : '')

phase('Author')
const authored = await pipeline(
  specs,
  (spec, _orig, i) =>
    agent(
      `Implement exactly this file-scoped spec in the vanguard-frontier-agentic repository.\n\n` +
        `TASK CONTEXT: ${task}\n\n` +
        `${evidence}` +
        `FILES YOU MAY TOUCH (exhaustive):\n${(spec.paths || [spec.path]).filter(Boolean).map((p) => `  - ${p}`).join('\n')}\n\n` +
        `SPEC:\n${spec.instruction || spec}\n\n` +
        `CONVENTIONS TO MIRROR: ${spec.conventions || 'match the surrounding files — frontmatter shape, heading structure, comment density, and tone.'}\n\n` +
        `ACCEPTANCE: ${spec.acceptance || 'the spec is satisfied exactly, and nothing outside the listed files changed.'}\n\n` +
        `${NO_COMMIT_RULE}`,
      { label: `author:${i + 1}`, phase: 'Author', schema: AUTHOR_SCHEMA },
    ),
  (result, spec, i) => {
    if (!result) return null
    return agent(
      `Adversarially verify work you did NOT do. Default to refuted=true when uncertain.\n\n` +
        `The spec was:\n${spec.instruction || spec}\n\n` +
        `The author claims: ${result.summary}\n` +
        `Files it says it wrote: ${(result.filesWritten || []).join(', ')}\n\n` +
        `Read those files yourself and try to REFUTE the claim. Refute if: a listed file was not actually written or ` +
        `does not exist; the content does not satisfy the spec; a file outside the spec's allowed list was modified; ` +
        `the change contradicts the repo conventions in CLAUDE.md; or a generated file was hand-edited instead of ` +
        `being regenerated. Cite file:line for anything you assert.\n\n` +
        `Do NOT fix anything. Do NOT edit files. Do NOT run git or npm. Report only.`,
      { label: `verify:${i + 1}`, phase: 'Verify', effort: 'high', schema: VERDICT_SCHEMA },
    ).then((v) => ({ spec: spec.path || `spec ${i + 1}`, result, verdict: v }))
  },
)

const authoredOk = (authored || []).filter(Boolean)
const refuted = authoredOk.filter((x) => x.verdict && x.verdict.refuted)
if (refuted.length) log(`REFUTED ${refuted.length}/${authoredOk.length} authored change(s) — orchestrator must resolve before committing`)

// ---------------------------------------------------------------- 5. Gates
// Haiku runs them. asset-integrity:write goes LAST and only if everything else
// is green — regenerating it before the other generators settle stales the
// manifest (see the ordering caveat in CLAUDE.md).

phase('Gates')
const gates = a.gates !== false
  ? await agent(
      `Run this repository's gate suite and report results.\n\n` +
        `Work from the repository root — the directory containing package.json and CLAUDE.md, which is your ` +
        `current working directory. Do NOT cd to an absolute path; the checkout location varies by machine.\n\n` +
        `ORDER MATTERS. Generators first, integrity second, validation last. Running \`npm run validate\` before ` +
        `refreshing the integrity manifest makes \`validate:asset-integrity\` fail on the stale manifest for any ` +
        `change that touches a hashed path (agents/, plugins/, .github/plugin/, package.json, or a root file) — ` +
        `a false failure that also blocks the refresh from ever happening. Run:\n\n` +
        `  1. Content generators, but ONLY those matching what changed:\n` +
        `       - catalog assets added/edited -> python3 scripts/update-catalog-new-agents.py [--provider <id>]\n` +
        `       - skills/ or plugin/power manifests -> npm run manifest:write:all\n` +
        `       - catalog counts feeding the docs site -> npm run docs-data:write\n` +
        `       - harness model/effort lines -> npm run model-policy:apply\n` +
        `     Skip any generator whose inputs did not change. Do NOT run npm run maestro-routing:write: it ` +
        `regenerates EVERY provider's fixtures, not just the one you changed.\n` +
        `  2. npm run asset-integrity:write — on its own, after every other generator has finished, so it hashes ` +
        `the settled tree.\n` +
        `  3. npm run validate\n` +
        `  4. npm run lint:spell\n` +
        `  5. npx --yes markdownlint-cli2 "**/*.md" "#node_modules" "#.git" "#CHANGELOG.md"\n` +
        `  6. If and only if tools/vfa-tui changed: cd tools/vfa-tui && cargo fmt --check && ` +
        `cargo clippy --all-targets --locked -- -D warnings && cargo test --locked\n\n` +
        `Report each gate's pass/fail and, for any failure, the RAW verbatim failure output — never a paraphrase ` +
        `like "some tests failed". The orchestrator needs the actual error text to decide the next move.\n\n` +
        `You may write only the files those generators own (catalog/, docs/_data/, plugin and power manifests, ` +
        `harness model lines). Make no hand edits of your own. No git commit. No git push.`,
      { label: 'gates', phase: 'Gates', model: 'haiku', effort: 'low', schema: GATE_SCHEMA },
    )
  : null

// ---------------------------------------------------------------- 6. Synthesis

phase('Synthesis')
const report = {
  task,
  dropped: {
    reconQuestions: Math.max(0, questionsIn.length - questions.length),
    libraries: Math.max(0, librariesIn.length - libraries.length),
    specs: Math.max(0, specsIn.length - specs.length),
  },
  recon: reconOk,
  context7: {
    results: c7,
    unavailable,
    contradictions,
    notCovered: c7.flatMap((r) => (r.notCovered || []).map((x) => ({ library: r.library, claim: x }))),
  },
  authored: authoredOk.map((x) => ({ spec: x.spec, files: x.result.filesWritten, refuted: !!(x.verdict && x.verdict.refuted), reason: x.verdict && x.verdict.reason })),
  refutedCount: refuted.length,
  gates,
  orchestratorMustDo: [
    'Read the actual diff — a delegate self-report and a green gate are both necessary, neither is sufficient.',
    contradictions.length ? `Fix ${contradictions.length} Context7-contradicted API claim(s) before committing.` : null,
    refuted.length ? `Resolve ${refuted.length} refuted change(s).` : null,
    unavailable.length ? `Context7 was unavailable for ${unavailable.join(', ')} — label those claims unknown rather than shipping them.` : null,
    gates && gates.allGreen === false ? 'Gate suite is RED — fix the cause, never bypass the gate.' : null,
    'Commit and push. This workflow never commits.',
  ].filter(Boolean),
}
return report
