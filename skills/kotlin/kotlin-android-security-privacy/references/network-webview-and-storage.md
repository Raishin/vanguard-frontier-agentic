# Network, WebView, And Storage

Cleartext/network-security-config, WebView exposure, and secret storage.

- Android 9+ (API 28+) defaults cleartextTrafficPermitted to false; a permissive network_security_config.xml or usesCleartextTraffic re-opens plaintext HTTP and must be scoped and justified.
- `addJavascriptInterface` is reachable from all frames and lacks origin control; `setJavaScriptEnabled(true)` with file access or untrusted URLs can exfiltrate the sandbox.
- EncryptedSharedPreferences (Jetpack Security) encrypts keys and values with a Keystore-backed master key; plaintext SharedPreferences leaves data readable in the app sandbox, and keys must never be hard-coded.

## Sources

- https://developer.android.com/training/articles/security-config
- https://developer.android.com/reference/android/webkit/WebView
