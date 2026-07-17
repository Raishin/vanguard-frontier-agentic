---
name: php-session-upload-deserialization-review
description: Use this skill to statically review PHP code for object-injection risk from unserialize() on untrusted input, session fixation/hijacking from missing session_regenerate_id() or weak session cookie hardening, and unsafe file-upload handling that trusts the client or stores/executes uploads inside the webroot. Use when reviewing a PHP application for deserialization, authentication-session, or upload-handling security issues. Static review only; it never executes payloads, uploads, or requests against any system, and every unserialize()/session/upload claim is grounded in the current php.net manual rather than memory.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-16"
  category: security
  lifecycle: experimental
---

# PHP Session, Upload, and Deserialization Review

## Purpose

Review PHP source for the three failure modes where the language's own deserialization, session, and file-upload primitives are used in a way php.net's own manual explicitly warns against: `unserialize()` called on untrusted input (object injection, potentially remote code execution), a session that is not regenerated after a privilege change or that ships without cookie hardening (fixation/hijacking), and an upload handler that trusts the client or stores/executes accepted files inside the webroot.

## When to use

Use this skill when the user asks to:

- review any code path that calls `unserialize()`, or a library/cache/session backend that transparently unserializes stored data, for object-injection risk;
- review whether a login, role-elevation, or password-reset path correctly regenerates the session id and whether session cookie hardening (`use_strict_mode`, `httponly`, `secure`, `samesite`) is configured;
- review a file-upload handler for client-trust, storage-location, or size/type/count-limit gaps.

## When not to use

Do not use this skill for:

- designing or approving a backend authorization/permission model — this skill only checks whether a privilege change triggers session regeneration, not whether the privilege model itself is correct; hand that off to the owning backend/platform specialist.
- executing, crafting, or sending any deserialization payload, session-fixation request, or file upload against any live, sandbox, or staging system. This skill is static review only.
- issuing a compliance or audit attestation from an OWASP category mapping; the mapping is a classification aid, never a determination.

## Preconditions

- The code paths that call `unserialize()`, plus the source of the data passed to each call (request body/query/cookie, cache backend, queue message, database column).
- The authentication/session-management code, specifically where `session_start()`, `session_regenerate_id()`, and `$_SESSION` writes occur relative to a login, role change, or password reset.
- The active `session.*` INI settings in scope (`session.use_strict_mode`, `session.cookie_httponly`, `session.cookie_secure`, `session.cookie_samesite`) from `php.ini`, `.htaccess`, or runtime `ini_set()` calls.
- The upload-handling code: how `$_FILES` is validated, where accepted files are stored, and the `upload_max_filesize`/`post_max_size`/`max_file_uploads` configuration in effect.
- The PHP version in scope (`session.cookie_samesite` requires PHP 7.3+; destructor-on-fatal-error behavior changed at PHP 5.3.10).

## Lean operating rules

- Flag any `unserialize()` call on untrusted input regardless of `allowed_classes` — the manual's caution applies unconditionally, and `allowed_classes` narrows instantiable classes without making the call safe.
- Name the reachable magic-method surface (`__wakeup()`, `__unserialize()`, `__destruct()`) for any class an untrusted `unserialize()` call could instantiate; PHP invokes these automatically on the reconstructed object or during the shutdown sequence.
- Recommend `json_decode()`/`json_encode()` as the default remediation for untrusted data interchange unless serialized PHP objects are confirmed necessary.
- Require `session_regenerate_id(true)` before the authenticated flag is written to `$_SESSION` on every login/elevation/reset path; a regeneration call placed after that write is a finding, not a pass.
- Check `session.use_strict_mode`, `session.cookie_httponly`, `session.cookie_secure`, and `session.cookie_samesite` individually and explicitly; never infer hardening from defaults, since `use_strict_mode` is disabled by default.
- Require upload handlers to validate the file's actual content server-side (never the client-supplied `$_FILES[...]['type']` or filename alone), store accepted files outside any web-servable path, and enforce `upload_max_filesize`, `post_max_size`, and `max_file_uploads`.
- Never request, echo, store, or reproduce a secret, credential, session id, token, or PII found in reviewed code; redact-and-flag any such string.
- Label every claim `repo evidence` (seen in the reviewed code) or `documentation-based` (current php.net manual); never assert a documented API behavior from memory.

## Context7 documentation protocol

This skill's `allowed-tools` is limited to `Read`, `Grep`, and `Glob` — it does not include a live documentation-lookup tool, so grounding is done against the bundled, php.net-sourced reference files rather than a runtime Context7 call. `unserialize()`, the `session.*` INI directives, and the upload-handling behaviors reviewed here are core-language documentation, not a third-party library whose API shifts release to release, so the bundled references (each fetched directly from php.net, with its own Sources section and verification date) are the grounding source of record. If a review surfaces a claim about PHP behavior not covered by the bundled references — a different function, a different INI directive, a different PHP version's behavior — do not assert it from memory; say the claim is unverified against a specific php.net page and escalate for direct verification rather than guessing.

## Workflow

1. Enumerate every `unserialize()` call site in scope and trace the data source of its argument; classify each as untrusted-input-reachable or not.
2. For each untrusted-input-reachable `unserialize()` call, identify classes it could instantiate and check for `__wakeup()`, `__unserialize()`, and `__destruct()` magic methods that would fire automatically — see [Unserialize object injection](references/unserialize-object-injection.md).
3. For each authentication/privilege-elevation path, confirm `session_regenerate_id(true)` is called before `$_SESSION` carries the authenticated flag, and check the four session-hardening directives — see [Session security](references/session-security.md).
4. For each upload handler, confirm server-side content validation, non-webroot storage, and enforced size/type/count limits — see [File upload security](references/file-upload-security.md).
5. Emit findings with evidence tiers, exploit/failure narratives, remediation, verification steps, and OWASP category mappings; hand off any authorization-model or infrastructure-scoped issue to the owning specialist.

## Decision gates

- Block only on a demonstrated untrusted-input-reachable `unserialize()` call, a missing/misordered `session_regenerate_id()` call, a missing `session.use_strict_mode`, or an upload handler that trusts the client, stores inside the webroot, or lacks enforced limits.
- Every `unserialize()`, session-directive, or upload-limit claim cites the current php.net manual (`documentation-based`) or the reviewed code (`repo evidence`) — never memory.
- Every OWASP mapping names the specific edition and category cited and is labeled a classification aid, never a compliance determination.
- Every authorization-model-design or infrastructure-scoped finding is handed off, not adjudicated here.

## Evidence classification

Label each finding `repo evidence` (seen directly in the reviewed code or configuration) or `documentation-based` (the current php.net manual). This skill does not use `context7-grounded` (no Context7 tool in scope) and does not use `inference` for a documented API behavior — an undocumented or unverifiable behavior claim must be flagged as unverified rather than guessed.

## Security and privacy constraints

Static review only. Never request, transmit, store, or reproduce a secret, credential, session id, token, or PII found in reviewed code or configuration; treat any such string as a redact-and-flag finding. Never execute a deserialization payload, replay or forge a session id, upload a file, or send any request to a live, sandbox, or staging system. OWASP category mappings are classification aids, never compliance attestations.

## Escalation conditions

Escalate to incident response on any evidence the failure is already live: an object-injection payload observed in logs, a reachable uploaded web shell, or an observed session id reused across a privilege boundary. Escalate backend authorization-model design and infrastructure-scoped findings (webroot layout, TLS termination) to their owning specialist rather than adjudicating them here.

## References

Load these only when needed:

- [Unserialize object injection](references/unserialize-object-injection.md) — why `unserialize()` on untrusted input is unsafe regardless of `allowed_classes`, the magic-method attack surface, and the `json_decode()` remediation.
- [Session security](references/session-security.md) — `session_regenerate_id()` ordering around privilege changes and the four session cookie-hardening directives.
- [File upload security](references/file-upload-security.md) — client-trust, storage-location, and size/type/count-limit review criteria for upload handlers.

Each reference file carries its own Sources section citing the exact php.net (or OWASP) page backing its claims — there is no separate primary-source ledger file in this skill.

## Response minimum

Return, at minimum:

- the call site/path in scope and, per finding, the failure class (`object-injection` / `session-fixation-hijacking` / `unsafe-upload`) and evidence tier;
- the exploit or failure narrative (how the reachable input reaches a wrong outcome);
- concrete remediation and an exact verification step;
- the OWASP category mapping, labeled as a classification aid;
- any authorization-model or infrastructure-scoped handoffs and incident-response escalation.

## Anti-goals

- Do not treat `allowed_classes` as a mitigation for `unserialize()` on untrusted input; the manual's caution is unconditional.
- Do not design or approve a backend authorization/permission model; hand it off.
- Do not execute any payload, upload, or request against any live, sandbox, or staging system.
- Do not assert `unserialize()`, session-directive, or upload-limit behavior from memory; ground every such claim in the current php.net manual.
- Do not echo, store, or reproduce any secret, session id, token, or PII found during review.
