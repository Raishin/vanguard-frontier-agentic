# Input validation, sanitization, and output escaping

## Why this matters

WordPress's own security guidance names a single core model for every place a plugin or theme handles data: validate and sanitize on input, escape on output, and never trust any input regardless of its source. Most plugin and theme vulnerabilities are a violation of exactly this model at one specific point — a value that should have been checked or cleaned before use, or a value that should have been escaped where it was printed, was not. Reviewing against this model directly, rather than against a vague sense of "looks risky," is what makes a finding concrete and actionable.

## NORMATIVE: validate/sanitize input, escape output, never trust input

Per the WordPress Developer Resources security overview (`documentation-based`): "Always make sure to validate and sanitize user input before using it, and to escape on output." The same page states its guiding principles directly: "Never trust user input," "Escape as late as possible," "Escape everything from untrusted sources (e.g., databases and users), third-parties (e.g., Twitter), etc.," and "Sanitation is okay, but validation/rejection is better." These principles apply regardless of the input's apparent trust level — including data read back from the WordPress database or from a third-party API, not only data taken directly from a request.

## NORMATIVE: sanitizing input

Per the WordPress Developer Resources sanitizing-data documentation (`documentation-based`), sanitizing is "the process of securing/cleaning/filtering input data," and "all of it needs to be checked before it's used," including from sources that might otherwise be assumed trusted. Commonly applicable sanitizing functions the same page documents include `sanitize_text_field()`, `sanitize_textarea_field()`, `sanitize_email()`, `sanitize_file_name()`, `sanitize_url()`, and `wp_kses()`/`wp_kses_post()` for input that must retain a restricted set of HTML. The page's own example shows `sanitize_text_field()` performing UTF-8 validation, tag stripping, and removal of line breaks/tabs/extra whitespace — concrete, checkable behavior a reviewer can compare a given sanitization call's expected effect against. The same page restates the preference already noted above: "Validation is preferred over sanitization because validation is more specific" — when a value has a specific expected shape (an integer ID, an email address, an enumerated option), validate and reject rather than merely sanitize.

## NORMATIVE: escaping output, as late as possible

Per the WordPress Developer Resources escaping-data documentation (`documentation-based`), escaping is "the process of securing output data by stripping out unwanted data, like malformed HTML or script tags." The page documents the core escaping functions by output context: `esc_html()` — "Use anytime an HTML element encloses a section of data being displayed. This will remove HTML"; `esc_attr()` — "Use on everything else that's printed into an HTML element's attribute"; `esc_url()` — "Use on all URLs, including those in the `src` and `href` attributes of an HTML element"; and `wp_kses_post()`, described as an "Alternative version of `wp_kses()` that automatically allows all HTML that is permitted in post content." The page states plainly: "It is best to do the output escaping as late as possible, ideally as data is being outputted" — escaping a variable once far from its output point, rather than inline at the point of output, is a documented anti-pattern, not merely a style preference, because it makes the correctness of the escaping harder to verify by inspection and easier to accidentally bypass with a later code change.

## NORMATIVE: nonces and capability checks for state-changing requests

Per the WordPress Developer Resources nonces documentation (`documentation-based`), nonces "help protect URLs and forms from certain types of misuse, malicious or otherwise," and specifically "help protect against several types of attacks including CSRF." WordPress provides `wp_create_nonce()`, `wp_nonce_url()`, and `wp_nonce_field()` to generate nonces, and `check_admin_referer()` (checks the nonce and the referrer, for admin screens), `check_ajax_referer()` (checks the nonce but not the referrer, for AJAX requests), and `wp_verify_nonce()` (general-purpose verification) to check them. For REST requests specifically, the REST API Handbook's authentication documentation states nonces are transmitted via the `_wpnonce` parameter or the `X-WP-Nonce` header under the `wp_rest` action, and that a missing or invalid nonce demotes the request to an unauthenticated one rather than failing the request outright.

Per the WordPress Developer Resources user-roles-and-capabilities documentation (`documentation-based`): "As you build a plugin, make sure to run your code only when the current user has the necessary capabilities." The page's own contrasting example shows code with no capability check allowing "any visitor to the site to trash posts," against a corrected version gated by `current_user_can( 'edit_others_posts' )` — a nonce check alone confirms the request came from an expected context; it does not confirm the requester is authorized. Both checks are required together for a state-changing request: a nonce check without a capability check still permits an authenticated-but-unauthorized user to act, and a capability check without a nonce check is still forgeable via CSRF from an authenticated user's own browser.

## Reviewer evidence criteria

- For every input source (REST params, `$_POST`/`$_GET`/`$_REQUEST`, shortcode attributes, block attributes, imported/uploaded data): confirm a validation or sanitization function is applied before the value is used in a query, a file path, a shell/command context, or output. An input used raw, or checked only with an `isset()`/type check with no actual sanitization/validation function, is a finding.
- Prefer validation (checking the value matches an expected, specific shape and rejecting it otherwise) over sanitization alone wherever the expected shape is specific enough to validate directly (an integer ID via `absint()`, an enumerated option checked against a whitelist).
- For every output point printing a value derived from user input, the database, or a third-party source: confirm the escaping function matches the output context (`esc_html()` for element content, `esc_attr()` for attributes, `esc_url()` for URLs, `wp_kses_post()` for content intentionally retaining limited HTML) and is applied at the point of output, not only earlier in the function.
- For every state-changing request (form submission, admin-ajax action, REST write): confirm both a nonce check (`wp_verify_nonce()`, `check_admin_referer()`, `check_ajax_referer()`, or REST nonce handling) and a `current_user_can()` capability check scoped to the specific action performed are present. Flag the absence of either one independently — do not treat one as substituting for the other.

## RECOMMENDATION: how to phrase a validate/sanitize/escape finding

- Name the exact input source or output point and the file/line involved.
- State which half of the model is missing: input not validated/sanitized before use, or output not escaped at the point of output (or both).
- Name the specific WordPress function that belongs there (e.g. `sanitize_text_field()` on input, `esc_html()` at output, `current_user_can( '<capability>' )` plus a nonce check on a state-changing action) rather than a generic "add validation" instruction.
- Where a nonce or capability check is missing on a state-changing request, state which of the two is absent (or both), since each addresses a different attack (CSRF vs. unauthorized-but-authenticated access).

## Applicable versions

- Guiding principles, sanitizing functions, and escaping functions: current WordPress Developer Resources security documentation. Re-verify against the current pages before relying on details beyond what is quoted here, since the function reference lists are maintained and can change.

## Sources

- [WordPress Developer Resources — Security](https://developer.wordpress.org/apis/security/) — supports the core "validate and sanitize input, escape on output" model and the guiding principles ("Never trust user input," "Escape as late as possible," "Escape everything from untrusted sources," "Sanitation is okay, but validation/rejection is better").
- [WordPress Developer Resources — Sanitizing Data](https://developer.wordpress.org/apis/security/sanitizing/) — supports the sanitizing-function reference (`sanitize_text_field()`, `sanitize_email()`, `sanitize_file_name()`, `sanitize_url()`, `wp_kses()`/`wp_kses_post()`) and the validation-over-sanitization preference.
- [WordPress Developer Resources — Escaping Data](https://developer.wordpress.org/apis/security/escaping/) — supports the escaping-function reference (`esc_html()`, `esc_attr()`, `esc_url()`, `wp_kses_post()`) and the "escape as late as possible" principle.
- [WordPress Developer Resources — Nonces](https://developer.wordpress.org/apis/security/nonces/) — supports nonce creation (`wp_create_nonce()`, `wp_nonce_url()`, `wp_nonce_field()`) and verification (`wp_verify_nonce()`, `check_admin_referer()`, `check_ajax_referer()`) as CSRF protection.
- [WordPress Developer Resources — User Roles and Capabilities](https://developer.wordpress.org/apis/security/user-roles-and-capabilities/) — supports the `current_user_can()` capability-check pattern and the contrast between an unchecked action and one gated by a specific capability.
- [WordPress REST API Handbook — Authentication](https://developer.wordpress.org/rest-api/using-the-rest-api/authentication/) — supports REST-specific nonce transport (`_wpnonce`/`X-WP-Nonce` under the `wp_rest` action) and the requirement for a capability check beyond nonce verification.

Last verified: 2026-07-16.
