# OPcache production configuration

## Why this matters

OPcache stores precompiled PHP bytecode in shared memory so subsequent
requests skip parsing and compiling the same script again. In production,
three misconfigurations defeat this: leaving OPcache disabled (paying full
compile cost on every request and gaining nothing), leaving timestamp
validation enabled in a deployment model that expects an immediate cutover
(serving stale bytecode, or paying an unnecessary per-request filesystem stat
cost, depending on which way the mismatch runs), and undersizing the cache
itself (memory or file-count limits below what the codebase actually needs,
so the accelerator evicts and recompiles under normal load instead of
reaching a stable steady state). None of these show up in application-code
review; they live entirely in `php.ini`.

## NORMATIVE: directive behavior per php.net

Per php.net's OPcache configuration reference (`documentation-based`):

- **`opcache.enable`** (default `1`) — enables the opcode cache. When
  disabled, code is not optimized or cached at all. It cannot be enabled at
  runtime via `ini_set()` — only disabled; attempting to enable it in a
  script generates a warning.
- **`opcache.validate_timestamps`** (default `1`) — if enabled, OPcache
  checks for updated scripts every `opcache.revalidate_freq` seconds. When
  disabled, cached scripts are never re-checked against the filesystem;
  `opcache_reset()`, `opcache_invalidate()`, or a web-server restart is
  required for filesystem changes to take effect. OPcache may still validate
  a file's timestamp at compile-time if `opcache.file_update_protection` or
  `opcache.max_file_size` are set to non-zero values.
- **`opcache.memory_consumption`** (default `128`, in megabytes) — the size
  of OPcache's shared-memory storage. The minimum permissible value is `8`;
  a smaller configured value is clamped up to it.
- **`opcache.max_accelerated_files`** (default `10000`) — the maximum number
  of keys (scripts) in OPcache's hash table. The actual value used is the
  first prime from a fixed internal set (`223, 463, 983, 1979, 3907, 7963,
  16229, 32531, 65407, 130987, 262237, 524521, 1048793`) that is greater than
  or equal to the configured value. The minimum is `200`; the maximum is
  `1000000`; out-of-range values are clamped into that range.

## RECOMMENDATION: production posture by deployment model

These are review recommendations, not php.net mandates — they apply the
documented directive behavior above to two common deployment models:

- **Immutable-image / container-per-deploy model** — where a new deploy
  replaces the running container/image wholesale (no in-place file sync to
  a long-running host): `opcache.validate_timestamps=0` is appropriate,
  because every deploy already starts a fresh OPcache with the new code —
  there is no stale-timestamp window to protect against, and skipping the
  per-request timestamp check removes filesystem overhead. `enable=1` is
  required either way.
- **In-place file-sync model** — where code is updated on disk under a
  long-running PHP-FPM/web-server process without a full process restart:
  leaving `validate_timestamps=1` (the default) is the safer choice unless
  the deploy pipeline explicitly calls `opcache_reset()` or
  `opcache_invalidate()` (or restarts the server) as part of every release.
  Setting `validate_timestamps=0` in this model without that compensating
  step means a deploy can silently continue serving stale bytecode.

## Reviewer evidence criteria

- Confirm `opcache.enable=1` in the production `php.ini` (or the effective
  merged configuration, if directives are set across multiple files);
  `opcache.enable=0` or the directive's absence at its non-default state is
  a blocking finding.
- Determine the deployment model in scope (immutable image vs. in-place
  sync) from the required inputs, then check `opcache.validate_timestamps`
  against it per the recommendation above. Flag a mismatch as blocking:
  `1` in an immutable-image model wastes the per-request stat check;
  `0` in an in-place-sync model with no compensating invalidation step in
  the deploy pipeline risks serving stale code.
- If `validate_timestamps=0` is set, look for the compensating step
  (`opcache_reset()`/`opcache_invalidate()` call, or a server restart) in
  the deploy pipeline before treating the configuration as safe.
- Compare `opcache.memory_consumption` and `opcache.max_accelerated_files`
  against the actual number and total size of PHP scripts in the codebase.
  Report undersizing as advisory guidance with a concrete recommended value
  (not a blocking finding on its own), since undersizing degrades
  performance rather than correctness.
- Do not assume a directive's default value applies without checking the
  actual `php.ini` (or merged configuration) in scope — label a stated
  default as `documentation-based` and an observed configuration as
  `repo evidence`.

## Sources

- [PHP: OPcache Configuration](https://www.php.net/manual/en/opcache.configuration.php) — supports the `opcache.enable`, `opcache.validate_timestamps`, `opcache.memory_consumption`, and `opcache.max_accelerated_files` descriptions and default/clamping values above.

Last verified: 2026-07-16.
