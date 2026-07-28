# High-Severity Failure Modes

The supply-chain incidents each finding class maps to, for severity calibration.

- A public package uploaded under an internal name with a higher version is installed instead of the private one, executing attacker code in the build (dependency confusion).
- An unhashed dependency is yanked and re-uploaded with a backdoor, and every subsequent build installs the backdoored artifact silently.
- A range-pinned dependency ships a breaking or compromised release overnight and the next deploy picks it up with no code change.
- An unpinned build backend runs arbitrary code at build time and exfiltrates the CI environment.
- A publish token exposed to a fork PR is used to push a malicious release under the project's identity.
