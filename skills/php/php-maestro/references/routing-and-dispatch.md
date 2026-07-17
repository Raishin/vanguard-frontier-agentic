# Routing and Dispatch

## Why this matters

A requester bringing a PHP task rarely knows, up front, which of the PHP board's specialists actually owns it — an insecure `unserialize()` call, an unaudited Composer dependency, an EOL PHP runtime, and an unsanitized WordPress REST route all look like "a PHP bug" from the outside, but each carries a distinct evidence contract and a distinct specialist. Misrouting a task to the wrong specialist, or to only one specialist when a task spans domains, means the wrong (or incomplete) review runs and a real finding never surfaces. This reference is the classification table Maestro uses to route correctly and consistently every time, instead of guessing per request.

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `application-security` | `unserialize()`, object injection, gadget chain, `__wakeup`/`__destruct`/magic-method exploitation, session fixation, session hijacking, `session.cookie_httponly`, `session.cookie_secure`, `session.use_strict_mode`, file upload handling, `$_FILES`, MIME-type trust, upload directory placement |
| `supply-chain` | `composer.json`, `composer.lock`, `composer audit`, Packagist advisory, abandoned package, malware-flagged package, dependency confusion, untrusted VCS repository, install-time script (`post-install-cmd`), minimum-stability |
| `runtime-eol` | PHP branch/version, active support window, security support window, end-of-life (EOL), upgrade readiness, OPcache configuration, `opcache.validate_timestamps`, PHP-FPM pool hardening, `pm` settings, `open_basedir`, `disable_functions`, `expose_php` |
| `wordpress-security` | WordPress plugin, theme, REST API route, permission callback, Gutenberg block, nonce, `check_admin_referer`, `current_user_can`, `sanitize_text_field`, `esc_html`/`esc_attr`/`esc_url`, `$wpdb->prepare`, direct file access (`ABSPATH`) |

## Full routing table

| Agent | Domain | Route when… |
|---|---|---|
| `php-application-security-agent` | `application-security` | The task is about `unserialize()` on untrusted input, PHP object injection/gadget-chain risk, session fixation/hijacking or session cookie hardening, or file-upload handling (MIME trust, filename handling, upload directory placement) |
| `composer-supply-chain-agent` | `supply-chain` | The task is about Composer dependency integrity — `composer.json`/`composer.lock` posture, `composer audit` findings, abandoned or malware-flagged packages, untrusted package sources, or install-time script risk |
| `php-runtime-upgrade-readiness-agent` | `runtime-eol` | The task is about PHP branch/version support status, upgrade planning off an end-of-life or soon-to-be-EOL branch, or OPcache/PHP-FPM hardening configuration |
| `wordpress-security-agent` | `wordpress-security` | The task is about WordPress plugin, theme, REST API, or block security — permission callbacks, nonces, capability checks, output escaping, or input sanitization in WordPress-specific code |

## Narrowest-specialist rule (NORMATIVE)

Route to exactly one specialist when exactly one domain signal is present. Do not dispatch a parallel team "to be safe" when the task is genuinely single-domain — this dilutes the specialist's review with an irrelevant second opinion and slows the requester down for no added signal.

## Parallel dispatch rule (NORMATIVE)

Dispatch two or more specialists in parallel only when the task genuinely spans two or more of the domains above — for example:

- A WordPress plugin that both calls `unserialize()` on request data and depends on a Composer package → `wordpress-security-agent` + `php-application-security-agent` + `composer-supply-chain-agent`.
- An upgrade project moving a WordPress site off an EOL PHP branch → `php-runtime-upgrade-readiness-agent` + `wordpress-security-agent`.

Name every domain the task touches before dispatching; do not silently drop a domain because only one specialist was asked for by name.

## Dispatch modes

**Single specialist** (one domain clearly identified):

```
Route: composer-supply-chain-agent
Reason: User wants composer.lock and composer audit output reviewed — supply-chain domain only.
Mode: single
```

**Parallel team** (two or more domains clearly identified):

```
Route: wordpress-security-agent + php-application-security-agent
Reason: A WordPress plugin's REST callback both needs a capability-check review and calls unserialize() on request data.
Mode: parallel (2)
```

**Refuse-and-ask** (domain ambiguous):

```
Route: none yet
Reason: Task scope is unclear — cannot tell whether this is an application-security or a runtime-EOL concern.
Mode: unclassified — ask for the smallest sufficient artifacts (the relevant PHP source file, composer.json, and the target PHP version)
```

## Response shape

Every Maestro response begins with the routing header:

```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N) | refuse-live-mutation | unclassified>
```

Followed by: dispatched specialist output (summarized, evidence labels preserved), then a handoff note to `php-board-chair-agent`, or to the named owning human if no chair exists yet.

## Evidence criteria

Reviewers verifying a routing decision should confirm:

- The routed agent ID appears literally in the table above and, once the PHP board is cataloged, in `catalog/agents.json`.
- A single-domain task was not fragmented into an unnecessary parallel dispatch.
- A genuinely multi-domain task named every domain it touches, not just the one the requester emphasized.
- The routing basis is labeled `live evidence`, `repo evidence`, `documentation-based`, or `inference`.

## Sources

- https://www.php.net/docs.php — PHP manual entry point; the routing domains above (`unserialize()`, sessions, file upload, Composer, runtime/EOL, WordPress) are each grounded against their own official page, not this landing page.
- https://getcomposer.org/doc/03-cli.md — grounds the `supply-chain` domain: `composer audit` checks installed packages against dependency policies (including Packagist.org security advisories by default) and also flags abandoned or malware-matched packages; `composer.lock` pins exact resolved versions for reproducible installs.
- https://developer.wordpress.org/apis/security/ — grounds the `wordpress-security` domain's validate/sanitize/escape framing used in the keyword table above.

Last verified: 2026-07-16.
