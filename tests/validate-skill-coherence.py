#!/usr/bin/env python3
"""Validate that every SKILL.md's shell examples are covered by its allowed-tools.

Every fenced shell command a SKILL.md demonstrates must be covered by its
declared `allowed-tools` frontmatter — least-privilege declarations must
match the skill's own examples. A skill that shows `gcloud container ...`
but only declares `Read Grep Glob` is under-declared (Claude Code would
need a Bash grant it never asked for); a skill that declares bare `Bash`
while only ever running `npm test` is over-declared. This gate catches the
under-declared case: real command examples with no matching `Bash`/
`Bash(pattern)` coverage.

Scope: only fenced code blocks whose info string is EXACTLY one of
`bash`, `sh`, `shell`, or `console` are scanned. Untagged (plain ```)
blocks are deliberately out of scope — they are not reliably shell
examples (could be pseudo-code, transcripts, etc.) and are not a signal
the harness's `Bash(...)` permission grammar can be checked against
anyway. Only `skills/**/SKILL.md` is scanned; `references/*.md` files are
out of scope per the task spec (SKILL.md is the harness-facing contract
that carries `allowed-tools`; references are supporting docs).

Reuses `parse_frontmatter` and `tokenize_allowed_tools` from
validate-skill-allowed-tools.py so the two gates never disagree about
what a token is.

Command extraction (per fenced block):
  - iterate physical lines; strip a leading "$ " prompt
  - join trailing-backslash line continuations into one logical line
  - skip blank lines and lines starting with "#"
  - split compound lines on `&&`, `||`, `;`, `|` into segments, tracking
    single-/double-quote state so an operator character *inside* a quoted
    string (e.g. the `|` in a `jq '.a[] | .b'` filter) is not mistaken for
    a shell pipeline operator — this parser is quote-depth-aware only; it
    does not handle backslash-escaped quotes inside single-quoted spans or
    heredocs, which is a correct simplification for the shell one-liners
    this repo's SKILL.md files demonstrate
  - for each segment, skip leading VAR=value assignment words, then take
    the command token
  - ignore shell builtins/environment ops that never need a Bash grant:
    export cd echo printf set unset source . true false exit pushd popd
  - everything else is a command segment that needs `allowed-tools` coverage

Coverage semantics: a bare `Bash` token in allowed-tools covers every
segment. Each `Bash(inner)` token becomes a regex matcher applied to the
whitespace-normalized full command segment: escape `inner`, turn a
trailing "<literal>:*" suffix into "<literal>(\\s.*)?$" (command-plus-
any-args semantics), turn every other escaped `*` into `.*`, and anchor
at the start. A segment is covered if ANY matcher matches. If a skill
shows command segments but declares no Bash-ish token at all, every
segment is a violation.
"""

from __future__ import annotations

import importlib.util
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS_DIR = ROOT / "skills"

# ── reuse the sibling gate's frontmatter/token parsing (don't fork it) ──────
_vsat_spec = importlib.util.spec_from_file_location(
    "vsat", os.path.join(os.path.dirname(__file__), "validate-skill-allowed-tools.py")
)
assert _vsat_spec is not None and _vsat_spec.loader is not None
vsat = importlib.util.module_from_spec(_vsat_spec)
_vsat_spec.loader.exec_module(vsat)

FENCE_LANGS = {"bash", "sh", "shell", "console"}

# Shell builtins / environment ops that never need a Bash allowed-tools grant.
IGNORE_BUILTINS = {
    "export",
    "cd",
    "echo",
    "printf",
    "set",
    "unset",
    "source",
    ".",
    "true",
    "false",
    "exit",
    "pushd",
    "popd",
}

ASSIGNMENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*=")
FENCE_OPEN_RE = re.compile(r"^(`{3,}|~{3,})\s*([A-Za-z0-9_+-]*)")

# Cap on '*' wildcards per Bash(inner) token. Each '*' becomes a regex `.*`;
# an unbounded chain of `.*` wildcards (e.g. Bash(a*a*a*...*a)) can
# catastrophically backtrack on a near-miss command and hang the gate. 12 is
# generous for any realistic allowed-tools pattern in this repo.
MAX_WILDCARDS = 12


def split_segments(text: str) -> list[str]:
    """Split on &&, ||, ;, | — but never inside a single- or double-quoted span.

    A bare regex split on `|` would fracture a real compound pipeline like
    `sf data query ... | jq '.a[] | .b'` at the `|` *inside* the quoted jq
    filter too, producing a bogus non-command fragment. Shell operators are
    only operators outside quotes, so track quote state while scanning.
    """
    segments: list[str] = []
    buf: list[str] = []
    in_single = False
    in_double = False
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if in_single:
            buf.append(ch)
            if ch == "'":
                in_single = False
            i += 1
            continue
        if in_double:
            buf.append(ch)
            if ch == "\\" and i + 1 < n:
                buf.append(text[i + 1])
                i += 1
            elif ch == '"':
                in_double = False
            i += 1
            continue
        if ch == "'":
            in_single = True
            buf.append(ch)
            i += 1
            continue
        if ch == '"':
            in_double = True
            buf.append(ch)
            i += 1
            continue
        if text[i : i + 2] in ("&&", "||"):
            segments.append("".join(buf))
            buf = []
            i += 2
            continue
        if ch in (";", "|"):
            segments.append("".join(buf))
            buf = []
            i += 1
            continue
        buf.append(ch)
        i += 1
    segments.append("".join(buf))
    return segments


def extract_allowed_tools_tokens(text: str) -> list[str]:
    """Return the raw allowed-tools tokens for a SKILL.md's frontmatter."""
    fm = vsat.parse_frontmatter(text)
    if fm is None:
        return []
    raw = fm.get("allowed-tools", "").strip()
    if not raw:
        return []
    if raw.startswith("[") and raw.endswith("]"):
        # Bracketed-list variant, parsed the same way
        # validate-skill-frontmatter-schema.py parses inline YAML sequences.
        inner = raw[1:-1].strip()
        return [t.strip().strip("'\"") for t in inner.split(",") if t.strip()]
    return vsat.tokenize_allowed_tools(raw)


def compile_bash_matcher(inner: str) -> re.Pattern[str]:
    """Turn a Bash(inner) constraint into a regex matcher on the full segment."""
    if inner.count("*") > MAX_WILDCARDS:
        raise ValueError(
            f"Bash(...) inner pattern has too many '*' wildcards "
            f"(>{MAX_WILDCARDS}), refusing to compile (ReDoS risk): {inner!r}"
        )
    escaped = re.escape(inner)
    for suffix in ("\\:\\*", ":\\*"):
        if escaped.endswith(suffix):
            escaped = escaped[: -len(suffix)] + r"(\s.*)?$"
            break
    escaped = escaped.replace(r"\*", ".*")
    return re.compile("^" + escaped)


def build_bash_matchers(tokens: list[str]) -> tuple[bool, list[re.Pattern[str]]]:
    """Return (has_bare_bash, [compiled Bash(...) matchers])."""
    has_bare_bash = False
    matchers: list[re.Pattern[str]] = []
    for tok in tokens:
        if tok == "Bash":
            has_bare_bash = True
        elif tok.startswith("Bash(") and tok.endswith(")"):
            matchers.append(compile_bash_matcher(tok[len("Bash(") : -1]))
    return has_bare_bash, matchers


def normalize(segment: str) -> str:
    return " ".join(segment.split())


def iter_fenced_blocks(lines: list[str]):
    """Yield (info_string_lower, [(lineno, raw_line), ...]) for each fence."""
    i = 0
    n = len(lines)
    while i < n:
        m = FENCE_OPEN_RE.match(lines[i])
        if not m:
            i += 1
            continue
        fence_char = m.group(1)[0]
        fence_min_len = len(m.group(1))
        info = m.group(2).lower()
        i += 1
        body: list[tuple[int, str]] = []
        while i < n:
            stripped = lines[i].strip()
            if stripped and set(stripped) == {fence_char} and len(stripped) >= fence_min_len:
                i += 1
                break
            body.append((i + 1, lines[i]))
            i += 1
        yield info, body


def extract_command_segments(block_lines: list[tuple[int, str]]) -> list[tuple[int, str]]:
    """Return [(lineno, segment_text), ...] for segments that need coverage."""
    results: list[tuple[int, str]] = []
    i = 0
    n = len(block_lines)
    while i < n:
        start_lineno, first_line = block_lines[i]
        text = first_line.strip()
        if text.startswith("$ "):
            text = text[2:]
        elif text == "$":
            text = ""

        # Join trailing-backslash continuations into one logical line.
        while text.rstrip().endswith("\\") and i + 1 < n:
            text = text.rstrip()[:-1].rstrip()
            i += 1
            _, next_line = block_lines[i]
            text = (text + " " + next_line.strip()).strip()

        i += 1

        if not text or text.startswith("#"):
            continue

        for raw_seg in split_segments(text):
            seg = normalize(raw_seg)
            if not seg:
                continue
            words = seg.split(" ")
            idx = 0
            while idx < len(words) and ASSIGNMENT_RE.match(words[idx]):
                idx += 1
            if idx >= len(words):
                continue  # segment was only VAR=value assignments
            command_token = words[idx]
            if command_token in IGNORE_BUILTINS:
                continue
            results.append((start_lineno, " ".join(words[idx:])))
    return results


def scan_skill(
    skill_md: Path,
) -> tuple[bool, int, list[tuple[int, str]], str | None]:
    """Return (has_shell_block, segment_count, [(lineno, segment), ...] violations, error).

    `error` is None unless the skill's allowed-tools declares a Bash(...)
    token that fails to compile (e.g. too many '*' wildcards — see
    MAX_WILDCARDS). In that case the other fields are unpopulated
    (False, 0, []) and the caller must treat this skill as a hard failure
    rather than silently skipping or crashing on it.
    """
    text = skill_md.read_text(encoding="utf-8")
    lines = text.splitlines()
    tokens = extract_allowed_tools_tokens(text)
    try:
        has_bare_bash, matchers = build_bash_matchers(tokens)
    except ValueError as exc:
        return False, 0, [], f"{skill_md}: {exc}"

    has_shell_block = False
    segment_count = 0
    violations: list[tuple[int, str]] = []

    for info, body in iter_fenced_blocks(lines):
        if info not in FENCE_LANGS:
            continue
        has_shell_block = True
        for lineno, seg in extract_command_segments(body):
            segment_count += 1
            if has_bare_bash:
                continue
            if any(m.match(seg) for m in matchers):
                continue
            violations.append((lineno, seg))

    return has_shell_block, segment_count, violations, None


def main() -> int:
    skill_files = sorted(SKILLS_DIR.glob("*/*/SKILL.md"))
    if not skill_files:
        print("ERROR: no SKILL.md files found", file=sys.stderr)
        return 2

    skills_with_blocks = 0
    total_segments = 0
    total_violations = 0
    skills_with_violations = 0
    violation_lines: list[str] = []
    scan_errors: list[str] = []

    for skill_md in skill_files:
        has_shell_block, segment_count, violations, error = scan_skill(skill_md)
        if error is not None:
            scan_errors.append(error)
            continue
        if has_shell_block:
            skills_with_blocks += 1
        total_segments += segment_count
        if violations:
            skills_with_violations += 1
            total_violations += len(violations)
            for lineno, seg in violations:
                violation_lines.append(
                    f"{skill_md}:{lineno}: command not covered by allowed-tools: {seg}"
                )

    if scan_errors:
        for line in scan_errors:
            print(f"ERROR: {line}", file=sys.stderr)

    if violation_lines:
        for line in violation_lines:
            print(line, file=sys.stderr)
        print(
            f"ERROR: {total_violations} uncovered command(s) in "
            f"{skills_with_violations} skill(s); fix the skill's allowed-tools "
            f"(narrowest matching pattern), never delete the command.",
            file=sys.stderr,
        )

    if scan_errors or violation_lines:
        return 1

    print(
        f"OK: skill coherence — {skills_with_blocks} skill(s) with shell blocks, "
        f"{total_segments} command segment(s), all covered"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
