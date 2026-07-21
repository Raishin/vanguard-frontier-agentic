# Recomposition Stability And Side Effects

How stability annotations and effect APIs determine correct recomposition behavior.

- @Stable promises a type's public properties won't change without notifying Compose, or are immutable, and @Immutable promises full immutability — both let Compose skip recomposition of a composable when such a parameter is unchanged.
- A class exposing `var` properties, or a non-primitive parameter Compose cannot prove stable, is treated as unstable, forcing recomposition of every composable that reads it whenever its parent recomposes.
- LaunchedEffect restarts its coroutine when any of its keys change and cancels it when the effect leaves composition; DisposableEffect requires a trailing onDispose to release whatever it acquired, and omitting it leaks that resource on every recomposition-triggered restart.
- rememberUpdatedState lets a long-lived effect reference the latest value of a parameter without restarting the effect itself, avoiding a stale-closure bug without forcing an unnecessary restart.

## Sources

- https://developer.android.com/develop/ui/compose/performance/stability
- https://developer.android.com/develop/ui/compose/side-effects
