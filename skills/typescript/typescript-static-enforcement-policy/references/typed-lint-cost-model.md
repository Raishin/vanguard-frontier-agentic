# Typed-Lint Cost Model

What type-aware linting costs, Project Service configuration, editor/CI parity, and the supported-version window check.

- Type-aware linting is documented to cost comparably to a full build, because it constructs its own TypeScript program separately from the build's program — a pipeline running both lint and typecheck independently pays that program-construction cost twice.
- typescript-eslint's current supported TypeScript range is `>=4.8.4 <6.1.0`; a project on a compiler version outside that range (including TypeScript 7.0.2) is running the parser outside its supported window, and the documented behavior is a warning, not a hard failure — so a green lint run does not by itself prove compatibility.
- The exact camelCase spellings of typescript-eslint's typed configuration exports are unverified in this skill's evidence base; only the kebab-case documentation identifiers were confirmed, so any exported-config-name claim must be checked against the installed package's actual exports rather than asserted from memory.

## Sources

- https://typescript-eslint.io/packages/parser/
