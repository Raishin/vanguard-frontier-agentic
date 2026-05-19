---
name: dotnet-aspnetcore-identity-authz-review
description: Use this skill when reviewing how an ASP.NET Core application authenticates and authorizes requests — authentication schemes, JWT TokenValidationParameters, cookie and session security, policy-based authorization, authorization handlers, claims trust, role-versus-resource authorization, multi-tenant isolation, privilege-escalation paths, and negative-test coverage. Trigger when a user provides ASP.NET Core authentication or authorization source (Program.cs, JWT bearer or cookie configuration, authorization policies, authorization handlers, controller authorize attributes) or sanitized configuration, asks whether their auth boundary is safe, or wants to know whether a tenant or role check can be bypassed. This skill reviews source and sanitized configuration statically; it never runs the application, mints or inspects tokens, or contacts an identity provider.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-19"
  category: security
  lifecycle: experimental
---

# .NET ASP.NET Core Identity & AuthZ Review

## Purpose
This skill reviews how an ASP.NET Core application authenticates and authorizes requests — the boundary that decides who a caller is and what they may do. An auth boundary is only sound if tokens are fully validated, state-changing endpoints are not anonymous, tenant and organization identity is verified server-side against the authenticated principal rather than trusted from client input, cookies carry the right security flags, authorization on owned resources checks ownership and not just role, and negative tests prove that unauthorized requests are actually rejected. The review catches disabled token validation, anonymous mutating endpoints, client-supplied tenant claims, weak cookie flags, role-only authorization on owned resources, missing negative tests, and hand-rolled token validation. It reads source and sanitized configuration only — it never runs the application, mints or inspects tokens, or contacts an identity provider. Generic middleware order is out of scope (the API agent owns that), and EF Core query-level tenant filters are out of scope (the EF Core agent owns those).

## Trigger conditions
- A user provides ASP.NET Core authentication or authorization source (`Program.cs`, JWT bearer or cookie configuration, authorization policies, authorization handlers, controller `[Authorize]` attributes) or sanitized configuration.
- A user asks whether their authentication or authorization boundary is safe.
- A user asks whether a tenant, organization, or role check can be bypassed or escalated.
- A user wants a pre-merge security review of an ASP.NET Core auth surface.

## Lean operating rules
- CRITICAL: treat `ValidateIssuer`, `ValidateAudience`, `ValidateIssuerSigningKey`, or `ValidateLifetime` set to false — or `RequireHttpsMetadata = false` outside loopback — as CRITICAL: token validation is disabled and forged or expired tokens are accepted.
- CRITICAL: treat `[AllowAnonymous]` on any state-changing endpoint (POST/PUT/PATCH/DELETE or a mutating handler) as CRITICAL — the operation runs with no authenticated caller.
- CRITICAL: treat a tenant or organization identifier taken from a client-supplied claim, header, or query value with no server-side verification against the authenticated principal as a CRITICAL privilege-escalation surface.
- HIGH: treat an authentication cookie missing `Secure`, `HttpOnly`, or an appropriate `SameSite` as HIGH.
- HIGH: treat authorization decided solely by role membership where the operation acts on a resource the caller must own as HIGH — any role-holder can act on another user's resource.
- HIGH: treat the absence of negative authorization tests (a request that must be rejected 401/403) as HIGH — nothing proves the boundary actually denies.
- HIGH: treat hand-rolled token or signature validation as HIGH.
- MEDIUM: treat scattered inline role-string checks instead of named authorization policies as MEDIUM.
- Never recommend `[AllowAnonymous]`, disabling validation, weakening cookie flags, or broad role grants to "unblock" a flow; never recommend disabling a failing gate as the fix.
- Static review only: never run the application, mint or inspect tokens, run builds, tests, or migrations, or contact an identity provider or any live system. Never request secrets, signing keys, client secrets, tokens, connection strings, tenant identifiers, or customer data; ask for sanitized configuration with placeholders.
- Label every finding with an evidence-basis label: `confirmed (config provided)`, `inference (config partial)`, `assumption (config absent)`, or `unknown`.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
1. Verdict (pass / pass-with-conditions / block)
2. Evidence level
3. Findings (severity-labelled: critical / high / medium / low, each with an evidence-basis label)
4. Safe next actions
5. Open questions
