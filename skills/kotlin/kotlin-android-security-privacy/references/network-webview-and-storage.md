# Network, WebView, And Storage

Cleartext/network-security-config, WebView exposure, and secret storage.

- The default cleartextTrafficPermitted value is governed by the app's targetSdkVersion, not the device's OS version — apps targeting API 28+ default cleartext off, but an app targeting API 27 or lower defaults cleartext on regardless of the device's Android version; inspect targetSdkVersion before treating the absence of a permissive network_security_config.xml/manifest entry as evidence cleartext is disabled, and require any permissive config be scoped and justified.
- `addJavascriptInterface` is reachable from all frames and lacks origin control; `setJavaScriptEnabled(true)` with file access or untrusted URLs can exfiltrate the sandbox.
- Plaintext SharedPreferences leaves data readable in the app sandbox and keys must never be hard-coded; recommend a current Android Keystore-backed storage design appropriate to the threat model rather than mandating EncryptedSharedPreferences, since the AndroidX Security Crypto library (and EncryptedSharedPreferences) is deprecated as of Security Crypto 1.1.0.

## Sources

- https://developer.android.com/training/articles/security-config
- https://developer.android.com/reference/android/webkit/WebView
