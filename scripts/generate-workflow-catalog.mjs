#!/usr/bin/env node
/**
 * Generator: catalog/workflows.json from .claude/workflows/*.js
 *
 * Workflow scripts are executable JavaScript whose body calls workflow globals
 * (`phase`, `agent`, `parallel`) that only exist inside the workflow runtime, so the
 * files cannot be imported to read their metadata — evaluating one outside the runtime
 * throws on the first `phase()` call.
 *
 * They can, however, be read statically: the workflow contract requires `meta` to be a
 * PURE LITERAL (no variables, calls, spreads, or interpolation), so the object literal
 * following `export const meta =` is safely evaluable on its own. This extracts exactly
 * that literal by brace matching and evaluates it in isolation.
 *
 * The output exists so `tools/vfa-tui` can display available workflows without
 * re-implementing any of this: the TUI reads generated catalog JSON and never parses
 * JavaScript. Same contract as every other catalog file.
 *
 * Usage:
 *   node scripts/generate-workflow-catalog.mjs           # write
 *   node scripts/generate-workflow-catalog.mjs --check   # verify in sync (CI)
 */
import { readFileSync, writeFileSync, readdirSync, existsSync } from 'node:fs'
import { join, dirname, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)))
const WORKFLOW_DIR = join(ROOT, '.claude', 'workflows')
const OUT = join(ROOT, 'catalog', 'workflows.json')

/**
 * Extract the balanced `{...}` literal following `export const meta =`.
 * Returns null when the marker is absent.
 */
function extractMetaLiteral(source) {
  const marker = /export\s+const\s+meta\s*=\s*/.exec(source)
  if (!marker) return null
  let i = marker.index + marker[0].length
  if (source[i] !== '{') return null

  // Brace matching that respects strings, template literals, and comments — a `{`
  // inside a description string must not be counted as nesting.
  let depth = 0
  let inS = null // "'" | '"' | '`'
  let inLine = false
  let inBlock = false
  for (; i < source.length; i++) {
    const c = source[i]
    const prev = source[i - 1]
    if (inLine) {
      if (c === '\n') inLine = false
      continue
    }
    if (inBlock) {
      if (c === '/' && prev === '*') inBlock = false
      continue
    }
    if (inS) {
      if (c === inS && prev !== '\\') inS = null
      continue
    }
    if (c === '/' && source[i + 1] === '/') { inLine = true; continue }
    if (c === '/' && source[i + 1] === '*') { inBlock = true; continue }
    if (c === "'" || c === '"' || c === '`') { inS = c; continue }
    if (c === '{') depth++
    else if (c === '}') {
      depth--
      if (depth === 0) return source.slice(marker.index + marker[0].length, i + 1)
    }
  }
  return null
}

function readWorkflow(file) {
  const abs = join(WORKFLOW_DIR, file)
  const src = readFileSync(abs, 'utf8')
  const literal = extractMetaLiteral(src)
  if (!literal) {
    throw new Error(`${file}: no 'export const meta = {...}' block found`)
  }
  let meta
  try {
    // Safe: the workflow contract requires meta to be a pure literal, and the literal
    // is evaluated alone — the script body, which needs the workflow runtime, is not.
    meta = new Function(`return (${literal})`)()
  } catch (err) {
    throw new Error(`${file}: meta is not a pure literal (${err.message})`)
  }
  if (!meta || typeof meta.name !== 'string' || typeof meta.description !== 'string') {
    throw new Error(`${file}: meta must declare string 'name' and 'description'`)
  }
  const expected = file.replace(/\.m?js$/, '')
  if (meta.name !== expected) {
    throw new Error(
      `${file}: meta.name '${meta.name}' must match the filename '${expected}' — ` +
      'the workflow is invoked by name, and a mismatch makes it unreachable',
    )
  }
  return {
    id: meta.name,
    name: meta.name,
    path: relative(ROOT, abs),
    description: meta.description,
    when_to_use: typeof meta.whenToUse === 'string' ? meta.whenToUse : '',
    phases: Array.isArray(meta.phases)
      ? meta.phases.map((p) => ({
          title: String(p.title ?? ''),
          detail: typeof p.detail === 'string' ? p.detail : '',
          model: typeof p.model === 'string' ? p.model : '',
        }))
      : [],
  }
}

function build() {
  const files = existsSync(WORKFLOW_DIR)
    ? readdirSync(WORKFLOW_DIR).filter((f) => /\.m?js$/.test(f)).sort()
    : []
  const workflows = files.map(readWorkflow)
  return {
    version: '0.1.0',
    description:
      'Workflow definitions discovered in .claude/workflows/. Generated from each ' +
      "script's meta block by scripts/generate-workflow-catalog.mjs — never hand-edited.",
    workflows,
  }
}

const catalog = build()
const serialized = JSON.stringify(catalog, null, 2) + '\n'

if (process.argv.includes('--check')) {
  const current = existsSync(OUT) ? readFileSync(OUT, 'utf8') : ''
  if (current !== serialized) {
    console.error(
      'FAIL: catalog/workflows.json is out of sync with .claude/workflows/.\n' +
      '      Run: npm run workflow-catalog:write',
    )
    process.exit(1)
  }
  console.log(`OK: workflow catalog in sync (${catalog.workflows.length} workflow(s))`)
} else {
  writeFileSync(OUT, serialized)
  console.log(`OK: wrote catalog/workflows.json (${catalog.workflows.length} workflow(s))`)
}
