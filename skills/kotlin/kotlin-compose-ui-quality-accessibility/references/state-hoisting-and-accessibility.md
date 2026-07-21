# State Hoisting And Accessibility

How state hoisting and accessibility annotations keep composables reusable and reachable.

- State hoisting moves state and its mutation up to the caller, leaving the child composable stateless and reusable, receiving a value and a change callback instead of owning state internally.
- rememberSaveable persists composable-level state across configuration change and process death by saving it through the same saved-instance-state mechanism, unlike plain remember which survives only recomposition.
- Modifier.semantics and contentDescription describe a composable to accessibility services such as TalkBack; a non-text element with neither is invisible or unlabeled to those services.
- Compose automatically expands a clickable element's touch target toward the accessible minimum, but an explicit smaller size or padding on the visual element does not by itself guarantee the enlarged touch area behaves as intended in every layout, so it must be checked rather than assumed.

## Sources

- https://developer.android.com/develop/ui/compose/state
- https://developer.android.com/develop/ui/compose/accessibility
