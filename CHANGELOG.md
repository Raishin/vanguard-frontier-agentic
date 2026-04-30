## 🛡️ v1.2.0 &mdash; 2026-04-30

> _Multi-cloud agent marketplace · `AWS` · `Azure` · `OCI` · `Terraform`_
>
> Built for operators on the cloud frontier — least privilege, live evidence, safe rollback paths.


### ✨ Features

* add 12 Azure + OCI live-guard agents with hardened least-privilege permissions ([ab3a156](https://github.com/Raishin/vanguard-frontier-agentic/commit/ab3a156fd24c39f7f13712cb647dc7da595c4099))
* add FinOps Cloud Price Advisor skill and agent ([6cab350](https://github.com/Raishin/vanguard-frontier-agentic/commit/6cab350bb9f46189e4b7b7053c05204ece858e85))
* add per-cloud Maestro router agents for AWS, Azure, and OCI ([ff1480f](https://github.com/Raishin/vanguard-frontier-agentic/commit/ff1480f5694d3db16bcf3558bedc203eb2f0b3cd))
* add Terraform Maestro cross-cloud IaC router agent ([71a6677](https://github.com/Raishin/vanguard-frontier-agentic/commit/71a66772048cfe1281ceaebc72a19faa55eac270))
* **oci:** strengthen policy-based IAM coverage with service principals + tier separation ([c4ce7f3](https://github.com/Raishin/vanguard-frontier-agentic/commit/c4ce7f36cb839e20ceb66a83b3460d7d7397b94a))

### 🐛 Bug Fixes

* **security:** resolve 1 CRIT + 3 HIGH + 1 MED + 1 LOW from PR [#7](https://github.com/Raishin/vanguard-frontier-agentic/issues/7) audit ([08056a5](https://github.com/Raishin/vanguard-frontier-agentic/commit/08056a59802b1a0278d7846210c27d042515cbb6))

### 🔒 Security

* harden Maestro router specs against adversarial eval findings ([cf1ff7c](https://github.com/Raishin/vanguard-frontier-agentic/commit/cf1ff7cc841e3199c2e1c0caf03b7c960802658c))

### 📚 Documentation

* deepen Azure/OCI live-guard skill references and update folder indexes ([b3a1abb](https://github.com/Raishin/vanguard-frontier-agentic/commit/b3a1abba4f11a952c4e8cab54b7b6e017df67169))
* **evals:** add Context7-grounded eval for Azure/OCI live-guard references ([9e1c12e](https://github.com/Raishin/vanguard-frontier-agentic/commit/9e1c12e53b109a5e9c3e0f50e2af69715315619b))
* **evals:** add security audit eval definition for PR [#7](https://github.com/Raishin/vanguard-frontier-agentic/issues/7) ([7b5ce4a](https://github.com/Raishin/vanguard-frontier-agentic/commit/7b5ce4afd3db5bf9fe703c8c2504f1e2e97da747))
* restructure README with get-started, skills/agents tables, FAQ, feedback ([8026fbe](https://github.com/Raishin/vanguard-frontier-agentic/commit/8026fbe69d0dbbc3c08b44439c59cd809c699e69))

# [1.1.0](https://github.com/Raishin/vanguard-frontier-agentic/compare/v1.0.0...v1.1.0) (2026-04-29)


### Bug Fixes

* **aws-agents:** normalize markdown harness templates ([b4d64ec](https://github.com/Raishin/vanguard-frontier-agentic/commit/b4d64ece188b52a59a52e7e8feebd9664fd9412d))


### Features

* **aws-agents:** add AWS role agents and codex harness validation ([9d2a995](https://github.com/Raishin/vanguard-frontier-agentic/commit/9d2a99581975be9b94ace0b1cdfdd4110007fc6b))
* **aws-agents:** add proactive and execution operator tiers ([260a914](https://github.com/Raishin/vanguard-frontier-agentic/commit/260a91405948426e1914682479a6e5b7865d6213))
* **aws-live-agents:** add guarded live operators and iam guidance ([e2e667e](https://github.com/Raishin/vanguard-frontier-agentic/commit/e2e667efe57c8ff71a30eb438aa59274695e25a2))
* **aws-skills:** add role-based portfolio and harden AgentCore guidance ([b953998](https://github.com/Raishin/vanguard-frontier-agentic/commit/b953998ab524e1001e401b3cd08aae02e383a6d4))

# 1.0.0 (2026-04-28)


### Bug Fixes

* **release:** harden npm packaging and runtime ([1f1aa42](https://github.com/Raishin/vanguard-frontier-agentic/commit/1f1aa42975eb4df7846b90026e964a0ca967bedf))
* **release:** validate before installing release dependencies ([b2c30cb](https://github.com/Raishin/vanguard-frontier-agentic/commit/b2c30cb76fc40f31929bb07a88e8e663f158fd3c))


### Features

* **agents/azure:** add cross-platform harness variants ([bdc7513](https://github.com/Raishin/vanguard-frontier-agentic/commit/bdc7513eeef7c3477a7e9ff944d917d6679c4f84))
* **agents/oci:** add cross-platform harness variants ([0b8508d](https://github.com/Raishin/vanguard-frontier-agentic/commit/0b8508dab60607c0245e3e1c2068e22f7ce619be))
* **agents:** add cloud expert agents and provenance rule ([f00a80f](https://github.com/Raishin/vanguard-frontier-agentic/commit/f00a80fa01dace6da259223ea8cfadfc4c6396cd))
* **azure:** add role-based agent portfolio ([279df4f](https://github.com/Raishin/vanguard-frontier-agentic/commit/279df4f5eceef68e9fe34254595bba5465b8df1d))
* **azure:** add role-based skill portfolio with grounded references ([823f31c](https://github.com/Raishin/vanguard-frontier-agentic/commit/823f31ca67fca8b3b06d053165ea6d65b19589ad))
* **marketplace:** add cross-platform agent export workflow ([30a4fe2](https://github.com/Raishin/vanguard-frontier-agentic/commit/30a4fe28c2e02d1df35673e11b95073b492307cb))
* **mcp:** add trusted cloud MCP references ([1d0ae01](https://github.com/Raishin/vanguard-frontier-agentic/commit/1d0ae013b307784410b5102b1bb2ef75b15b01e6))
* **skills/azure:** expand and tighten Azure skill guidance ([364c440](https://github.com/Raishin/vanguard-frontier-agentic/commit/364c440aa14d520975fc33d2ce9f3438a0af5498))
* **skills:** add cloud security workflow catalog ([2f0f2d5](https://github.com/Raishin/vanguard-frontier-agentic/commit/2f0f2d506238e7552bec09fae0cb8535556ec1f4))
