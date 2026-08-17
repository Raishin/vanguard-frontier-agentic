# Authentication and the Strong-Authentication Rollout

How to establish an account's real authentication posture, and why the calendar date never establishes it. Load for any MFA, password, or enforcement question.

## The date is not the state

- Snowflake's strong-authentication programme runs as phased windows, not as a single cutover. A window that has opened does not mean this account, this user type, or this specific user has been affected yet.
- Effect differs by user type: human users (`TYPE = PERSON`) move toward mandatory MFA on password sign-in; non-human users move away from password authentication entirely, with `LEGACY_SERVICE` being the transitional type that is removed.
- Documented exclusions exist — reader accounts, trial accounts, and Snowflake Postgres are called out as outside the timeline. Never generalize the timeline to an account whose class has not been established.
- Therefore: never write 'Phase 3 is active, so your password logins are already blocked.' Establish it from `USERS`, `LOGIN_HISTORY`, and the authentication policies in force, or report `UNKNOWN`.
- The correct recommendation is independent of the rollout state in any case: a non-human identity should not authenticate with a password whether or not the platform still permits it.

## User types decide what is possible

- `PERSON` — a human. MFA is the control that matters, and it is enforced through an authentication policy, not by hoping.
- `SERVICE` — automated applications and services. Cannot use a password; authenticates via key-pair, OAuth, workload identity federation, or a programmatic access token.
- `SERVICE_AGENT` — intended for automated AI agents, distinct from a general service user. Where an AI agent identity is being designed, this is the type to investigate first rather than reusing a human or a generic service user.
- `LEGACY_SERVICE` — the transitional type that still permits password authentication for non-interactive integrations, documented as deprecated with migration to `SERVICE` encouraged. Its presence in an account is a finding with a migration path, not a configuration choice.
- Changing a user's type is a real change with a real blast radius: it can invalidate the authentication method a running integration depends on. Propose it with the integration inventory, not as a one-liner.

## Removing the stored secret entirely

- Workload identity federation lets a `TYPE = SERVICE` user trust an external issuer — a cloud workload identity or an OIDC issuer such as a CI/CD platform — so no Snowflake credential is stored anywhere.
- The `SUBJECT` in a workload identity configuration is the authorization boundary. A subject scoped to an entire repository or organization rather than a specific branch, environment, or service connection is over-broad, and it is over-broad in exactly the way a leaked static credential would be.
- Programmatic access tokens are a credential and are revocable per user and per token name; an authentication policy can require a role restriction on them for service users. Treat a token inventory as part of the credential inventory, not as a separate concern.
- Key-pair authentication remains a stored secret — better than a password, still a secret. Where federation is available it removes the storage problem instead of improving it.
- Published tutorials frequently set `DEFAULT_ROLE = ACCOUNTADMIN` on service users for convenience. Treat that as a documentation artifact, never as a pattern: the default role is what the session holds before it asks for anything, so it sets the floor of a pipeline compromise.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Strong authentication for all users — mandatory MFA for every human user on password authentication, and full deprecation of legacy service users with existing LEGACY_SERVICE users migrated to SERVICE — is scheduled for August 2026 to October 2026. | Phased rollout window, per-account effect | 2026-08-17 via Context7 `/websites/snowflake_en` (security-mfa-rollout) | That the enforcement is scheduled and that its window is open | That this account, this user type, or this user has been enforced yet — that requires account evidence |
| Strong authentication for new users — mandatory MFA for human users created after the enforcement date, and no new legacy service users — is scheduled for May 2026 to July 2026. | Phased rollout window | 2026-08-17 via Context7 `/websites/snowflake_en` | That newly created identities are affected earlier than existing ones | The state of any pre-existing identity in this account |
| The rollout timelines are documented as not applying to reader accounts, trial accounts, or Snowflake Postgres. | Documented exclusions | 2026-08-17 via Context7 `/websites/snowflake_en` | That account class changes whether the timeline applies at all | The class of the account under review |
| `SERVICE_AGENT` exists as a user type alongside `PERSON`, `SERVICE`, and the deprecated `LEGACY_SERVICE`, and is described as being for automated applications or AI agents. | Available as documented — confirm in the target account | 2026-08-17 via Context7 `/websites/snowflake_en` (ALTER USER TYPE property) | That a distinct identity type for AI agents exists and should be considered before reusing a generic service user | That it is enabled, appropriate, or supported by every authentication path in this account |

## Evidence queries

Establish the real authentication posture per user type — never infer it from the date.

```sql
SELECT type                                  AS user_type,
       COUNT(*)                              AS users,
       COUNT_IF(has_password)                AS with_password,
       COUNT_IF(has_rsa_public_key)          AS with_key_pair,
       COUNT_IF(ext_authn_duo)               AS with_mfa_flag,
       COUNT_IF(disabled)                    AS disabled,
       COUNT_IF(last_success_login < DATEADD(day, -90, CURRENT_TIMESTAMP())) AS stale_90d
  FROM SNOWFLAKE.ACCOUNT_USAGE.USERS
 WHERE deleted_on IS NULL
 GROUP BY type
 ORDER BY users DESC;
-- The finding to look for first: any non-PERSON row with with_password > 0.
```

Determine which authentication factor was actually used, rather than which is configured.

```sql
SELECT user_name,
       first_authentication_factor,
       second_authentication_factor,
       COUNT(*)              AS logins,
       MAX(event_timestamp)  AS last_login
  FROM SNOWFLAKE.ACCOUNT_USAGE.LOGIN_HISTORY
 WHERE event_timestamp >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   AND is_success = 'YES'
 GROUP BY 1, 2, 3
 ORDER BY logins DESC;

SHOW AUTHENTICATION POLICIES IN ACCOUNT;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/security-mfa-rollout — The phased enforcement timeline, the human-versus-service-user distinction, and the documented account-class exclusions
- https://docs.snowflake.com/en/user-guide/authentication-policies — How MFA enforcement is expressed as a policy, including scoping a policy to all service users and the ENFORCE_MFA_ON_EXTERNAL_AUTHENTICATION option
- https://docs.snowflake.com/en/sql-reference/sql/alter-user — The TYPE property and the semantics of PERSON, SERVICE, SERVICE_AGENT, and the deprecated LEGACY_SERVICE
- https://docs.snowflake.com/en/user-guide/workload-identity-federation — That a SERVICE user can trust an external workload identity or OIDC issuer, removing the stored Snowflake credential
- https://docs.snowflake.com/en/user-guide/programmatic-access-tokens — That programmatic access tokens are per-user, named, revocable, and constrainable by authentication policy
