# Binary Compatibility Validator And Explicit API Mode

How .api snapshots gate public-surface changes and how Explicit API mode prevents accidental growth.

- The Kotlin binary-compatibility-validator Gradle plugin dumps a library's public ABI to a committed `.api` file; `apiDump` regenerates that snapshot and `apiCheck` fails the build whenever the current compiled public surface no longer matches the committed snapshot.
- Explicit API mode (`explicitApi()` for a hard failure, `explicitApiWarning()` for a warning) requires every public and protected declaration to state its visibility and return type explicitly, so an inferred type or an accidentally-public declaration cannot silently enter the compiled API surface.
- `apiDump` should be run only after a human reviews the diff between the old and new `.api` file and confirms the change is the intended one — running it reflexively to make `apiCheck` pass launders a breaking change into the new baseline.

## Sources

- https://github.com/Kotlin/binary-compatibility-validator
- https://kotlinlang.org/docs/whatsnew14.html#explicit-api-mode-for-library-authors
