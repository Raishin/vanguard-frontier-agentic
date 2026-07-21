# ViewModel Lifecycle And State Persistence

How ViewModel scoping and SavedStateHandle determine what survives configuration change versus process death.

- ViewModel is retained across configuration changes by the ViewModelStore tied to its lifecycle scope, so the same instance survives rotation and is cleared only when that scope finishes for real.
- SavedStateHandle persists small key-value UI state through both configuration change and process death by writing into the saved-instance-state Bundle, unlike plain ViewModel properties which survive only configuration change.
- A ViewModel must never hold a reference to an Activity, Fragment, or View — doing so leaks the destroyed view because the ViewModel outlives it across configuration change.

## Sources

- https://developer.android.com/topic/libraries/architecture/viewmodel
- https://developer.android.com/topic/libraries/architecture/saving-states
