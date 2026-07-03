# Slopsquat Dependency Check

Use this reference whenever a diff introduces a new package dependency — a new entry in `package.json`/`requirements.txt`/equivalent manifest, a new lockfile entry, or an import statement pulling from a package not previously present in the project.

## What people get wrong

The common bad assumption is:

> "The model wrote this import, the package name sounds like a real, well-known library, so it must exist and be the right one."

That is the exact mechanism of a slopsquat attack. LLMs, when asked to solve a problem, will sometimes hallucinate a plausible-sounding package name that does not exist — a name that reads as if it *should* be the canonical solution to the stated problem. Attackers monitor this pattern and pre-register those exact hallucinated names on public registries (npm, PyPI, etc.) with malicious payloads, betting that a generated suggestion will eventually get installed verbatim by someone who does not check. A name "sounding right" is therefore not weak evidence of legitimacy — it is close to zero evidence, because it is the precise signal an attacker optimizes for.

This is distinct from classic typosquatting (a deliberate misspelling of a real popular package, e.g. `reactt` or `lodash-es-`) — slopsquatting targets names an AI model is likely to *invent* for a given task, which may not resemble any existing real package at all.

## Non-negotiable check rules

1. **Every new dependency gets a live registry lookup — no exceptions for "obviously real" names.** Resolve the package on its actual public registry (npm registry for JS/TS, PyPI for Python, crates.io for Rust, RubyGems, etc.) via `WebFetch`/`WebSearch`. Do not accept the name on the strength of it sounding familiar or matching a well-known naming convention.
2. **Verify identity, not just existence.** A registry hit is not sufficient — confirm the resolved package's publisher/maintainer, description, weekly download count, repository link, and first-publish date align with what the diff claims the package does. A newly-squatted name can exist on the registry with near-zero downloads and no meaningful history.
3. **Treat a registry miss as a hard blocker, not a note.** If the exact package name does not resolve on the registry at all, this is the strongest possible slopsquat signal — the install must not proceed until the correct real package name is identified and independently confirmed.
4. **Check for confusable near-misses even on a registry hit.** Compare the found package against the likely *intended* real package for the stated purpose (e.g., is there a much more widely-used, near-identically-named alternative that this name is shadowing or bidding to be confused with?).
5. **Do not let version-pinning or lockfile presence substitute for a registry check.** A lockfile entry only proves something was resolved and installed at some point — potentially by an earlier, equally unverified step — not that the package is legitimate.
6. **Escalate, do not silently "fix," a suspicious name.** If a dependency looks slopsquatted, do not unilaterally swap in what you assume is the "real" package — flag it explicitly and require the author to confirm intent, since silently substituting could also introduce the wrong package.

## Verification workflow

1. Diff the manifest/lockfile to enumerate every newly introduced package name and version constraint.
2. For each new name, perform a live registry lookup (`WebFetch`/`WebSearch` against the registry's public package page or API).
3. Record, per package: registry hit/miss, publisher, download volume/popularity signal, repository URL, first-publish date.
4. Cross-reference the package's stated purpose in the diff/commit message against what the registry listing actually describes.
5. Classify each new dependency as:
   - `registry-verified` — resolves to a real, actively-maintained package matching the claimed purpose; cite the registry URL,
   - `registry-verified but low-confidence` — resolves, but with signals worth a human look (very new, near-zero downloads, unrelated description, generic/unmaintained-looking repo),
   - `slopsquat-risk / registry-lookup-failed` — does not resolve, or resolves to something that plausibly is a hallucinated-name squat.

## High-risk assumptions to kill

- "It's a scoped package under a name I recognize (e.g., `@react-...`), so the scope itself vouches for it." Scopes can be squatted or created by unrelated parties; verify the specific scope owner too.
- "The AI tool that generated this surely only suggests real packages." That is precisely the failure mode this check exists to catch — treat every model-suggested package name with the same suspicion regardless of tool confidence.
- "It installed successfully, so it must be real." A successful `npm install`/`pip install` only proves the name resolved on the registry at install time — it says nothing about whether the package is the legitimate, intended one or a malicious squat.

## When to push back

Push back if the user asks you to:

- approve a new dependency without a registry lookup because "we're in a hurry,"
- silently rename a suspicious dependency to what you guess was intended, rather than flagging it for explicit confirmation,
- treat popularity of the *stated* library concept (e.g., "everyone uses a library like this") as a substitute for verifying *this specific* package name resolves to that library.

Those shortcuts reintroduce exactly the supply-chain risk this check exists to close.
