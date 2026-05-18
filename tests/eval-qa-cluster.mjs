#!/usr/bin/env node
// Golden eval grader for the QA skill/agent cluster.
// Code-based grader (deterministic) — verifies reference grounding,
// severity-heuristic wiring, and harness coverage for skills/qa + agents/qa.
// Run: node tests/eval-qa-cluster.mjs

import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { join } from 'node:path';

const SKILLS_DIR = 'skills/qa';
const AGENTS_DIR = 'agents/qa';
const TIER_HARNESS_COUNT = { 'static-review': 7, 'read-only-runtime': 2 };
const STALE_DOC_PATTERN = /\/20(1\d|2[0-4])\.\d{1,2}\//; // dated vendor-doc version pins

const checks = [];
const record = (id, pass, detail) => checks.push({ id, pass, detail });

const skillDirs = readdirSync(SKILLS_DIR).filter((d) =>
  existsSync(join(SKILLS_DIR, d, 'SKILL.md')),
);
const agentDirs = readdirSync(AGENTS_DIR).filter((d) =>
  existsSync(join(AGENTS_DIR, d, 'metadata.json')),
);

for (const skill of skillDirs) {
  const base = join(SKILLS_DIR, skill);
  const meta = JSON.parse(readFileSync(join(base, 'metadata.json'), 'utf8'));
  const skillMd = readFileSync(join(base, 'SKILL.md'), 'utf8');
  const docs = meta.official_docs || [];

  record(`REF-1:${skill}`, docs.length >= 3, `${docs.length} official_docs`);
  record(
    `REF-2:${skill}`,
    !docs.some((u) => STALE_DOC_PATTERN.test(u)),
    'no stale dated version pins in official_docs',
  );
  record(
    `REF-3:${skill}`,
    skillMd.includes('references/workflow-and-output.md'),
    'SKILL.md links its progressive-disclosure reference',
  );
  const refPath = join(base, 'references', 'workflow-and-output.md');
  const refMd = existsSync(refPath) ? readFileSync(refPath, 'utf8') : '';
  const isRuntime = meta.execution_tier === 'read-only-runtime';

  if (isRuntime) {
    // Execution skills emit a pass/fail/manual-review verdict, not severity buckets.
    record(
      `HEUR-1:${skill}`,
      ['pass', 'fail', 'manual-review'].every((v) => skillMd.includes(v)),
      'SKILL.md defines the pass/fail/manual-review verdict shape',
    );
    record(
      `HEUR-2:${skill}`,
      ['pass', 'fail', 'manual-review'].every((v) => refMd.includes(v)),
      'workflow-and-output.md output contract carries the verdict shape',
    );
  } else {
    // Static-review skills enumerate severity-graded findings.
    record(
      `HEUR-1:${skill}`,
      /\bHIGH\b/.test(skillMd) && /\bMEDIUM\b/.test(skillMd),
      'SKILL.md enumerates HIGH/MEDIUM severity heuristics',
    );
    record(
      `HEUR-2:${skill}`,
      ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW'].every((s) => refMd.includes(s)),
      'workflow-and-output.md output contract has all four severity buckets',
    );
  }
}

for (const agent of agentDirs) {
  const base = join(AGENTS_DIR, agent);
  const meta = JSON.parse(readFileSync(join(base, 'metadata.json'), 'utf8'));
  const companions = meta.companion_skills || [];

  record(
    `REF-4:${agent}`,
    companions.length > 0 &&
      companions.every((s) => existsSync(join(SKILLS_DIR, s, 'SKILL.md'))),
    `companion_skills resolve: ${JSON.stringify(companions)}`,
  );

  const variants = meta.harness_variants || {};
  record(
    `HARNESS-1:${agent}`,
    Object.values(variants).every((p) => existsSync(p)),
    `${Object.keys(variants).length} harness_variants files exist`,
  );
  const expected = TIER_HARNESS_COUNT[meta.execution_tier];
  record(
    `HARNESS-2:${agent}`,
    expected !== undefined && Object.keys(variants).length === expected,
    `harness count ${Object.keys(variants).length} matches ${meta.execution_tier} tier (expect ${expected})`,
  );
}

const failed = checks.filter((c) => !c.pass);
for (const c of checks) {
  console.log(`${c.pass ? 'PASS' : 'FAIL'}  [${c.id}] ${c.detail}`);
}
console.log(
  `\nQA cluster eval: ${checks.length - failed.length}/${checks.length} checks passed ` +
    `(${skillDirs.length} skills, ${agentDirs.length} agents)`,
);
if (failed.length > 0) {
  console.error(`\nFAILED: ${failed.map((c) => c.id).join(', ')}`);
  process.exit(1);
}
console.log('OK: QA cluster eval green');
