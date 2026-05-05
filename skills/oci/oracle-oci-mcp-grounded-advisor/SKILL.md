---
name: oracle-oci-mcp-grounded-advisor
description: Use this skill when the user asks about Oracle MCP servers, SQLcl MCP, OCI MCP, Oracle Database agent access, OCI automation, or cloud/database advice that must be grounded in official Oracle sources.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: github: Raishin
  version: 0.1.0
  updated: "2026-05-05"
  category: ai
---

# Oracle and OCI MCP Grounded Advisor

## Purpose

Prevent hallucinated Oracle and OCI guidance by forcing official-source verification before recommendations.

## Workflow

1. Identify whether the question is about Oracle Database, SQLcl MCP, OCI, MySQL, documentation MCP, or general MCP architecture.
2. Verify current behavior against official Oracle pages or repositories when the answer affects installation, authentication, supported versions, or permissions.
3. Distinguish Oracle-official MCP servers from community projects.
4. State authentication and data-access risks clearly.
5. Prefer read-only exploration and documentation lookup before tool execution.

## Output

Return:

- recommendation,
- source-backed facts,
- uncertainty or version caveats,
- setup/security notes,
- next validation command or official document to read.

## Security notes

Database MCP access can expose production data. Do not recommend connecting an agent to a database unless the user has scoped credentials, audit expectations, and read/write boundaries.
