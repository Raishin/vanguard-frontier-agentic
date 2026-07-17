# Unserialize object injection

## Why this matters

`unserialize()` does not just decode data — it can rebuild arbitrary objects
and then automatically hand them control. If the serialized string an
attacker controls names a class the application has autoloaded (directly or
transitively), reconstructing that object can invoke its magic methods with
attacker-influenced property values, and one of those methods reaching a
dangerous sink (a file write, a command, a second `unserialize()`, a query)
is enough to turn a data-parsing call into code execution. This is PHP
object injection, and php.net's own manual treats it as unconditional: there
is no safe way to call `unserialize()` on untrusted input, only safer
alternatives to use instead.

## NORMATIVE: php.net's unconditional caution

Per the php.net manual page for `unserialize()`:

- "Do not pass untrusted user input to `unserialize()` regardless of the
  `options` value of `allowed_classes`. Unserialization can result in code
  being loaded and executed due to object instantiation and autoloading, and
  a malicious user may be able to exploit this."
- The manual's stated alternative for untrusted data interchange is "a safe,
  standard data interchange format such as JSON (via `json_decode()` and
  `json_encode()`)."
- After successfully reconstructing an object, PHP "will automatically
  attempt to call the `__unserialize()` or `__wakeup()` methods (if one
  exists)" — this invocation happens as part of `unserialize()` itself, not
  as a separate step a caller can skip.

This is the single controlling fact for this review: the caution is not
scoped to "when `allowed_classes` is absent" — it applies **regardless of**
that option's value. A review must not treat `allowed_classes` as the fix.

## RECOMMENDATION: what `allowed_classes` actually does, and does not do

`allowed_classes` restricts which class names `unserialize()` is willing to
instantiate (e.g. `unserialize($string, ['allowed_classes' => false])`
instantiates no objects at all, converting them to `__PHP_Incomplete_Class`
instead). Use it as defense-in-depth when a design genuinely requires
`unserialize()` over data that cannot be fully untrusted-input-free, but do
not present it to engineers as a fix for untrusted-input reachability — the
manual's caution stands regardless of the option, and per user-contributed
notes on the same manual page, `allowed_classes` does not follow class
inheritance (allowing an interface does not allow its implementers, and
subclasses of an allowed class do not automatically pass).

## The wider magic-method attack surface: `__destruct()` too

Object-injection ("PHP Object Injection" / gadget-chain) exploitation is not
limited to `__wakeup()`/`__unserialize()`. Any class instantiated by the
`unserialize()` call is a live PHP object with its full lifecycle ahead of
it, including destruction. Per the php.net manual for destructors: "The
destructor method will be called as soon as there are no other references to
a particular object, or in any order during the shutdown sequence." This
means a reconstructed object with attacker-controlled properties can trigger
consequential logic in its `__destruct()` even if it has no `__wakeup()` or
`__unserialize()` at all — including at request shutdown, when no other code
runs after it. Two destructor-execution details a reviewer should carry
into any gadget-chain analysis, per the same page:
`__destruct()` calls made during shutdown happen "in any order" (no
guaranteed sequence among unrelated objects), and destructors are called
even when the script stops via `exit()`. When reviewing a class reachable
from `unserialize()`, check `__wakeup()`, `__unserialize()`, and
`__destruct()` — all three are automatic, attacker-triggerable entry points
once the object exists.

## RECOMMENDATION: prefer `json_decode()` for untrusted data

Match the manual's own stated remediation: for any data interchange that
crosses a trust boundary (request body, query string, cookie, cache key
sourced from user input, a queue message an external party can influence),
use `json_decode()`/`json_encode()` rather than `serialize()`/`unserialize()`.
If a legacy format genuinely requires PHP's native serialization for
internal, fully-trusted data only, confirm the value never round-trips
through anything a client can influence, and consider the manual's mention
of `hash_hmac()`-based integrity verification for data an application stores
externally and later reads back, so that tampering can be detected before
`unserialize()` (or `json_decode()`) is ever called on it.

## Reviewer evidence criteria

For each `unserialize()` call site:

- Identify the exact source of the string passed in. Untrusted sources
  include request parameters, headers, cookies, uploaded file contents,
  values read back from a cache/session/queue backend that a client can
  influence, and any database column a client-facing write path can reach.
- If the source is untrusted, this is a blocking finding regardless of
  whether `allowed_classes` is set, per the manual's unconditional caution.
- If `allowed_classes` is present, do not close the finding on that basis;
  note it as partial defense-in-depth only, and still require remediation
  toward `json_decode()` or a fully-trusted data path.
- For any class the call could instantiate (open allowlist, `false`, or a
  named list), check for `__wakeup()`, `__unserialize()`, and `__destruct()`
  definitions in that class and its parents; if any exists, name it as a
  live attack-surface entry point in the finding, not just a theoretical one.
- Confirm the recommended remediation is `json_decode()`/`json_encode()`
  (or an equivalent non-object-instantiating format) unless the review
  independently confirms native PHP serialization is required and the data
  never crosses a trust boundary.

## Applicable versions

The `unserialize()` caution, the automatic `__unserialize()`/`__wakeup()`
invocation, and the `allowed_classes` behavior described above are current
php.net manual guidance as of this review; the destructor shutdown-ordering
and `exit()` behavior is also current manual guidance, with the manual
separately noting that destructors are not run on shutdown caused by a fatal
error as of PHP 5.3.10. Re-verify against the live manual pages before citing
an exact figure for a PHP version outside current support, since documented
behavior can be refined between manual revisions.

## Sources

- [PHP Manual — `unserialize()`](https://www.php.net/manual/en/function.unserialize.php) — supports the unconditional caution against passing untrusted input to `unserialize()` regardless of `allowed_classes`, the automatic `__unserialize()`/`__wakeup()` invocation on reconstructed objects, and the `json_decode()`/`json_encode()` remediation recommendation.
- [PHP Manual — Destructors](https://www.php.net/manual/en/language.oop5.decon.php) — supports the `__destruct()` invocation timing (on last reference removal or in any order during shutdown), invocation even after `exit()`, and the PHP 5.3.10 fatal-error exception to shutdown-time destructor execution.
- [OWASP Top 10:2021 — A08:2021 Software and Data Integrity Failures](https://owasp.org/Top10/2021/A08_2021-Software_and_Data_Integrity_Failures/index.html) — supports the OWASP category mapping for insecure-deserialization findings (CWE-502: Deserialization of Untrusted Data is listed among this category's notable weaknesses). OWASP has since published a Top 10:2025 edition (https://owasp.org/Top10/2025/) that renumbers this category to A08:2025 – Software or Data Integrity Failures and moves Injection to A05:2025; cite the specific edition and number in any finding rather than assuming 2021 numbering is still current.

Last verified: 2026-07-16.
