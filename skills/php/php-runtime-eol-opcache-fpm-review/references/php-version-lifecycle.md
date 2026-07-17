# PHP version lifecycle

## Why this matters

A PHP branch that has left php.net's supported window stops receiving fixes of
any kind once it reaches end of life (EOL) — including for actively exploited
vulnerabilities — while a branch in its security-only window still receives
critical security fixes but no bug fixes, and is itself counting down to its
own EOL. A service can look fully healthy at the application-code level (green
tests, current dependencies) while the runtime underneath it has already
crossed one of these boundaries. This file is the sole source the skill uses
for those dates; every version-lifecycle claim in a review must resolve to a
row below, and to no other memory or estimate.

## NORMATIVE: php.net's support policy

Per php.net's Supported Versions page (`documentation-based`):

- Each release branch of PHP is fully supported for **two years** from its
  initial stable release. During this period, reported bugs and security
  issues are fixed and released in regular point releases.
- After that two-year active-support period, each branch is then supported
  for **two additional years** for critical security issues only. Releases
  during this window are made on an as-needed basis — there may be multiple
  point releases, or none, depending on the number of reports.
- After the combined four-year window (two years active plus two years
  security-only), a branch is **end of life (EOL)**: no further fixes of any
  kind are published for it, including for actively exploited
  vulnerabilities.

## NORMATIVE: current per-branch dates

Per php.net's Supported Versions page, the currently listed branches and
their published dates are:

| Branch | Active support until | Security support until (EOL) |
|---|---|---|
| PHP 8.2 | 31 Dec 2024 | 31 Dec 2026 |
| PHP 8.3 | 31 Dec 2025 | 31 Dec 2027 |
| PHP 8.4 | 31 Dec 2026 | 31 Dec 2028 |
| PHP 8.5 | 31 Dec 2027 | 31 Dec 2029 |

Reading this table for a review:

- A branch is in **active support** if the current review's target/committed
  version date is before its "Active support until" date.
- A branch is in **security-only support** if that date is on or after
  "Active support until" but before "Security support until."
- A branch is **EOL** once that date reaches or passes "Security support
  until" — no fixes of any kind are published past that point.

Any PHP version not listed in php.net's current supported-versions table
(either older than the branches php.net still lists, or a malformed/future
version string) has no dates in this file — say the evidence is missing
rather than inferring a status.

## Reviewer evidence criteria

- Resolve the exact PHP version from the strongest available evidence:
  a running-version banner or `phpversion()`/`php -v` output outranks a
  Dockerfile/base-image tag, which outranks a loose Composer `require.php`
  constraint (e.g. `^8.2` does not tell you which point release is actually
  deployed, only the minimum branch). State which evidence tier the version
  claim rests on.
- Match the resolved branch against the table above exactly; do not round a
  point release to a different branch or assume a branch's status from a
  prior review.
- Classify the branch precisely as active support, security-only (cite the
  exact security-support-until date), or EOL (cite the exact date it was
  reached) — never collapse these into an undifferentiated "outdated" label.
- Judge status using the **target or committed PHP version and this file's
  published dates only** — never the wall clock or "today." The same
  repository state must produce the same finding regardless of when the
  review is run, so results stay reproducible and CI-stable.
- For a security-only branch, compare its "Security support until" date
  against the review's stated release/support horizon. If that date falls
  within the horizon with no tracked upgrade plan, this is a blocking
  finding per the skill's decision gates, not merely advisory.
- For an EOL branch, the finding is blocking unconditionally — there is no
  "well-maintained otherwise" exception, since EOL means no fixes of any kind
  are published, including for actively exploited vulnerabilities.

## NORMATIVE vs. RECOMMENDATION

The support-policy structure and the per-branch dates above are NORMATIVE —
sourced directly from php.net and not to be altered, rounded, or
extrapolated. The recommended target version for an upgrade (e.g. "move to
the newest branch still in active support") is a RECOMMENDATION: name a
target that is currently in active support per the table above, but leave the
final choice, timeline, and any compatibility work to the owning engineering
team.

## Sources

- [PHP: Supported Versions](https://www.php.net/supported-versions.php) — supports the two-year active-support / two-year security-only / EOL support-policy structure and the per-branch dates for PHP 8.2 through 8.5 above.

Last verified: 2026-07-16.
