#!/usr/bin/env python3
"""Gate: a Copilot agent's tool grant must match its declared execution tier.

The Copilot adapter is the one harness in this repo that carries a per-agent tool
grant (the Markdown-family adapters carry name/description only, and Codex expresses
posture through ``sandbox_mode``). That makes ``copilot.agent.md`` the place where an
agent's declared trust posture either is or is not actually enforced — and it had
drifted both ways:

  * ``static-review`` agents were granted ``execute/runInTerminal``, contradicting the
    tier's own definition ("no Bash", per schemas/agent.schema.json) and the agent's
    own contract.
  * Agents with no ``tools:`` block at all inherit every tool the harness offers, which
    is the opposite of least privilege — an implicit grant strictly wider than anything
    a tier permits.

Rules enforced (only for agents that DECLARE an execution_tier; see the note on
untiered agents below):

  1. An agent with a declared tier must carry an explicit ``tools:`` block. Inheriting
     the harness default is never a deliberate grant.
  2. ``static-review`` must not carry any execution tool. It reads source; it never runs.
  3. ``read-only-runtime`` must not carry an execution tool either, UNLESS the agent is
     in EXEC_ALLOWLIST — a small, explicitly justified set whose documented job is to
     run a command (not to mutate). Routers and advisors do not qualify.
  4. ``mutating-runtime`` may carry execution tools; that is the point of the tier.

Untiered agents are reported but not failed: ``execution_tier`` is optional in
schemas/agent.schema.json and 347 agents (mostly the cloud boards) omit it. Assigning
those tiers is a judgment call per agent and a separate change; this gate deliberately
does not guess a tier in order to police one.

Usage:
    python3 tests/validate-agent-tool-tiers.py            # check (CI)
    python3 tests/validate-agent-tool-tiers.py --write    # apply the minimal fix
    python3 tests/validate-agent-tool-tiers.py --report   # inventory, always exit 0

``--write`` is deliberately surgical: it removes only the offending execution entries
and inserts a tier-appropriate block where one is missing. It never rewrites a tool
grant that already satisfies the rules, so hand-tuned per-agent grants are preserved.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
AGENTS = REPO / "agents"

# Tools that let an agent run a command. Anything matching these is an execution grant.
EXEC_PREFIXES = ("execute/",)
EXEC_EXACT = {"run_terminal_command", "runCommands", "terminal"}

# read-only-runtime agents whose documented purpose is to RUN a read-only command.
# Each entry needs a one-line justification tied to the agent's own contract.
EXEC_ALLOWLIST = {
    # "Execute an existing Playwright E2E suite against an operator-confirmed
    # non-production target"; runtime execution is a documented per-session opt-in.
    "playwright-e2e-execution-run-agent",
}

# Minimal, tier-appropriate default grant used only when a block is missing entirely.
READ_TOOLS = ["read", "search", "search/codebase", "web/fetch"]
EXEC_TOOLS = [
    "read/problems",
    "execute/runInTerminal",
    "execute/getTerminalOutput",
    "read/terminalLastCommand",
]

TOOLS_BLOCK_RE = re.compile(r"^tools:\n((?:[ \t]+- \".*\"\n)+)", re.M)


def is_exec_tool(tool: str) -> bool:
    return tool.startswith(EXEC_PREFIXES) or tool in EXEC_EXACT


def default_tools(tier: str, agent_id: str) -> list[str]:
    if tier == "mutating-runtime" or agent_id in EXEC_ALLOWLIST:
        return READ_TOOLS + EXEC_TOOLS
    return list(READ_TOOLS)


def parse_tools(text: str) -> list[str] | None:
    m = TOOLS_BLOCK_RE.search(text)
    if not m:
        return None
    return re.findall(r'- "([^"]+)"', m.group(1))


def render_block(tools: list[str]) -> str:
    return "tools:\n" + "".join(f'  - "{t}"\n' for t in tools)


def set_tools(text: str, tools: list[str]) -> str:
    """Replace an existing tools block, or insert one before the frontmatter close."""
    if TOOLS_BLOCK_RE.search(text):
        return TOOLS_BLOCK_RE.sub(render_block(tools), text, count=1)
    # Insert before the closing delimiter of the YAML frontmatter (the 2nd '---').
    if not text.startswith("---\n"):
        raise ValueError("copilot adapter does not start with YAML frontmatter")
    close = text.index("\n---", 4)
    return text[: close + 1] + render_block(tools) + text[close + 1 :]


def allowed_violation(tier: str, agent_id: str, tools: list[str]) -> list[str]:
    """Return the execution tools this agent must not have."""
    if tier == "mutating-runtime":
        return []
    if tier == "read-only-runtime" and agent_id in EXEC_ALLOWLIST:
        return []
    return [t for t in tools if is_exec_tool(t)]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true", help="apply the minimal fix")
    ap.add_argument("--report", action="store_true", help="inventory only; exit 0")
    args = ap.parse_args()

    failures: list[str] = []
    fixed: list[str] = []
    untiered = 0
    checked = 0
    allowlisted_seen = 0

    for meta_path in sorted(AGENTS.glob("*/*/metadata.json")):
        meta = json.loads(meta_path.read_text(encoding="utf-8"))
        tier = meta.get("execution_tier")
        agent_id = meta.get("id", meta_path.parent.name)
        adapter = meta_path.parent / "harnesses" / "copilot.agent.md"
        if not adapter.exists():
            continue
        if not tier:
            untiered += 1
            continue

        checked += 1
        if agent_id in EXEC_ALLOWLIST:
            allowlisted_seen += 1
        rel = adapter.relative_to(REPO)
        text = adapter.read_text(encoding="utf-8")
        tools = parse_tools(text)

        # Rule 1 — a declared tier requires an explicit grant.
        if tools is None:
            if args.write:
                adapter.write_text(set_tools(text, default_tools(tier, agent_id)), encoding="utf-8")
                fixed.append(f"{rel}: added explicit {tier} tool grant")
            else:
                failures.append(
                    f"{rel}: tier {tier!r} but no 'tools:' block — inherits every harness "
                    f"tool (implicit grant wider than the tier allows)"
                )
            continue

        # Rules 2-4 — no execution grant outside mutating-runtime / the allowlist.
        offending = allowed_violation(tier, agent_id, tools)
        if offending:
            if args.write:
                kept = [t for t in tools if t not in offending]
                if not kept:
                    kept = list(READ_TOOLS)
                adapter.write_text(set_tools(text, kept), encoding="utf-8")
                fixed.append(f"{rel}: removed {', '.join(offending)} ({tier})")
            else:
                failures.append(
                    f"{rel}: tier {tier!r} must not grant execution tools, but grants "
                    f"{', '.join(offending)}"
                )

    if args.report:
        print(f"agents with a declared execution_tier and a copilot adapter: {checked}")
        print(f"agents without execution_tier (not policed here): {untiered}")
        print(f"violations that would be reported: {len(failures)}")
        for f in failures[:20]:
            print(f"  - {f}")
        return 0

    if args.write:
        for f in fixed:
            print(f"  ~ {f}")
        print(f"OK: applied {len(fixed)} tool-grant fix(es) across {checked} tiered agent(s)")
        return 0

    if failures:
        for f in failures:
            print(f"FAIL {f}")
        print(
            f"\n{len(failures)} agent tool-grant violation(s). "
            f"Run: python3 tests/validate-agent-tool-tiers.py --write",
            file=sys.stderr,
        )
        return 1

    print(
        f"OK: {checked} tiered agent(s) carry an explicit copilot tool grant consistent "
        f"with their execution tier ({untiered} untiered agents not policed; "
        f"EXEC_ALLOWLIST holds {len(EXEC_ALLOWLIST)} read-only executor(s), of which "
        f"{allowlisted_seen} have a copilot adapter)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
