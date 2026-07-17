# PHP-FPM pool tuning

## Why this matters

PHP-FPM's process manager controls how many PHP worker processes exist and
how long each one lives. Two settings dominate the production risk surface:
`pm` together with `pm.max_children` bounds worker concurrency, so an
unbounded or unsized value lets a traffic spike spawn more workers than the
host has memory for, exhausting the host outright; and `pm.max_requests`
bounds a worker's lifetime in requests served, so leaving it unlimited lets a
slow memory leak in application or third-party code degrade a worker (and
eventually the pool) indefinitely instead of being recycled away. Neither
failure is visible in application-code review — both live in the FPM pool
configuration file.

## NORMATIVE: directive behavior per php.net

Per php.net's PHP-FPM configuration reference (`documentation-based`):

- **`pm`** (mandatory, no default) — chooses how the process manager
  controls the number of child processes. Possible values:
  - `static` — the number of child processes is fixed at `pm.max_children`.
  - `dynamic` — the number of child processes varies at runtime between `pm.min_spare_servers`/`pm.max_spare_servers`, starting from `pm.start_servers`, up to a ceiling of `pm.max_children`.
  - `ondemand` — processes spawn only when a request arrives, rather than `pm.start_servers` being started when the service starts.
- **`pm.max_children`** (mandatory, no default) — the number of child
  processes created when `pm=static`, and the *maximum* number of child
  processes created when `pm=dynamic` or `pm=ondemand`. This directive sets
  the hard limit on the number of simultaneous requests the pool will serve
  — equivalent to Apache's `MaxClients` directive under `mpm_prefork`, or the
  `PHP_FCGI_CHILDREN` environment variable in the original PHP FastCGI.
- **`pm.max_requests`** (default `0`, meaning unlimited) — the number of
  requests each child process executes before it is recycled (killed and
  respawned). Documented as useful for working around memory leaks in
  third-party libraries. A value of `0` means a worker serves requests
  indefinitely, with no automatic recycling.

## RECOMMENDATION: sizing and review posture

These are review recommendations applying the documented directives above,
not php.net mandates for a specific numeric value:

- **`pm.max_children` must be evidently bounded by available memory.** As a
  rough check: multiply `pm.max_children` by a representative single-worker
  memory footprint (observed or estimated for the application) and confirm
  the product does not exceed the memory actually available to the pool's
  host or container. A `pm.max_children` value set without any apparent
  relationship to available memory (an arbitrary round number carried over
  from a different host size, or left at a packaging default) is a
  resource-exhaustion risk under a traffic spike.
- **Prefer `dynamic` or `ondemand` over an oversized `static` pool** for
  workloads with variable traffic, so idle capacity is not permanently
  reserved; `static` is reasonable when load is genuinely constant and the
  host is sized for the fixed worker count.
- **`pm.max_requests` should be a nonzero, workload-appropriate value** in
  production, so a worker with a slow leak is recycled before it degrades
  the pool. Leaving it at the default `0` (unlimited) is acceptable only with
  an explicit, documented rationale (e.g. the application and every
  third-party library in its request path are already known leak-free under
  sustained load) — otherwise treat unbounded `pm.max_requests` as a blocking
  finding alongside unbounded `pm.max_children`.

## Reviewer evidence criteria

- Confirm `pm` is explicitly set (it is mandatory with no default) to one of
  `static`, `dynamic`, or `ondemand`, and matches the stated workload shape.
- Confirm `pm.max_children` is set (mandatory, no default) and check it
  against the rough memory-bound calculation above; flag a value with no
  apparent sizing rationale, or one clearly exceeding available host/
  container memory, as a blocking finding.
- If `pm=dynamic`, confirm `pm.start_servers`, `pm.min_spare_servers`, and
  `pm.max_spare_servers` are all set consistently with `pm.max_children`
  (each within a sane range relative to the ceiling), since a missing or
  contradictory value among this triad undermines the sizing already done
  for `pm.max_children`.
- Confirm `pm.max_requests` is set to a nonzero value, or that an explicit
  documented rationale exists for leaving it at `0`; flag an unexplained `0`
  as a blocking finding.
- Label an observed pool configuration `repo evidence`, a php.net-documented
  default or semantic `documentation-based`, and any memory-footprint
  estimate used for the sizing check `inference` — state which is which
  rather than presenting an estimate as a measured fact.

## Sources

- [PHP: PHP-FPM Configuration](https://www.php.net/manual/en/install.fpm.configuration.php) — supports the `pm`, `pm.max_children`, and `pm.max_requests` descriptions, mandatory/default-value status, and the `MaxClients`/`PHP_FCGI_CHILDREN`/`PHP_FCGI_MAX_REQUESTS` equivalences above.

Last verified: 2026-07-16.
