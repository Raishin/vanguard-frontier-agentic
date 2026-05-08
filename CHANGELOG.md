## 🛡️ v1.5.0 — *Provenance, Policy, Portability* &mdash; 2026-05-08

> _Multi-cloud agent marketplace · `AWS` · `Azure` · `OCI` · `Terraform`_
>
> Built for operators on the cloud frontier — least privilege, live evidence, safe rollback paths.


* Merge pull request #16 from Raishin/claude/review-kubernetes-patterns-C4uNb
feat(kubernetes): network architecture review agent + complete 5-layer live-guard defense

### fix

* **kubernetes-live-guards:** correct verifiable bugs surfaced by adversarial review
Five-persona adversarial critique of the live-guard agents found three
verifiable repo-internal bugs and two upstream-verified factual errors.
Patches:

User-approved set (3):
- agents/kubernetes/kubernetes-live-rbac-mutation-guard-agent: declare
  companion_skills (was missing; CLAUDE.md project rule mandates it).
* **kubernetes-live-guards:** correct YAML indentation and CoreDNS ClusterRole scope
P1 fixes from Codex review of PR #16:

1. Fix 6-space indentation in 6 retrofitted live-guard RBAC manifests.
   The Python generator script left all top-level content indented by
   6 spaces, causing `---` document separators to appear mid-line and
   making every manifest unparseable by kubectl / YAML loaders. Fixed
   by stripping the spurious prefix uniformly.

2. Scope CoreDNS ConfigMap writes to kube-system via a namespaced
   Role + RoleBinding instead of a ClusterRole rule.
   resourceNames on a ClusterRole restricts only the object name, not
   the namespace; the SA would have been able to patch any ConfigMap
   named "coredns" in any namespace, contradicting the adjacent comment
   and the negative pre-flight checks. The ClusterRole rule is removed;
   a kube-system-scoped Role (read: configmaps, patch: configmaps/coredns)
   and matching RoleBinding are added.

All 7 npm run validate gates pass.
* **kubernetes-live-guards:** strip 8-space prefix from 12 markdown reference files
Same generator bug as the YAML manifests: the Python retrofit script left every
non-heading line indented by 8 spaces in rbac-pre-flight.md and refusal-list.md
for all 6 retrofitted live-guards. Markdown renderers treated the entire body
as a code block, hiding section headings, prose, tables, and the kubectl
auth can-i matrix.

Stripped uniformly. All 7 npm run validate gates remain green.
* **kubernetes-live-network-arch-guard:** close 9 destructive-operation gaps from Context7-grounded stress test
The original guard's HARD REFUSE list covered 9 operations. A second-pass stress
test grounded against kubernetes.io documentation surfaced 9 more destructive
operations the agent and binding did not explicitly reject. This commit closes
those gaps across four surfaces.

Context7 sources consulted (kubernetes.io):
- /docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm
  (kubectl delete node, kubectl drain semantics)
- /docs/reference/kubernetes-api/extend-resources/mutating-webhook-configuration-v1
  (DELETE /apis/admissionregistration.k8s.io/v1/mutatingwebhookconfigurations)
- /docs/reference/kubernetes-api/workload-resources/priority-class-v1
  (PriorityClass deletion semantics)
- /docs/reference/kubernetes-api/_print
  (IngressClass deletion endpoint)
- /docs/concepts/security/_print "Bind verb" / "denial of service risks"
- /docs/reference/access-authn-authz/rbac (subresource permissions)

New refusal sections added to references/refusal-list.md (9 sections):

1. Node operations — kubectl delete node / drain / cordon / uncordon, patch
   nodes/spec.unschedulable, patch nodes/spec.taints. Per upstream: drain with
   --ignore-daemonsets --force --delete-emptydir-data is mass-eviction.

2. Admission webhook configurations — Mutating/ValidatingWebhookConfiguration
   create/patch/update/delete. Bypass admission policies (Kyverno, sidecar
   injection, cert-manager). failurePolicy: Ignore on attacker-controlled
   webhook = silent observation; failurePolicy: Fail = cluster-wedging.

3. APIService aggregation — apiregistration.k8s.io APIService writes. Hijack
   metrics.k8s.io / custom-metrics / external-metrics for HPA poisoning.

4. Finalizer manipulation — patch metadata.finalizers on Namespaces, PVs, CRDs.
   The kubectl patch ns kube-system --type=merge -p '{"metadata":{"finalizers":[]}}'
   pattern looks like routine metadata edit; it is the bypass for namespace
   finalizer protection. Combined with delete = catastrophic.

5. Pod and node subresources — pods/exec, pods/portforward, pods/proxy,
   pods/binding, pods/eviction, nodes/proxy. exec into kube-system =
   privilege escalation via cilium-agent / kube-proxy / coredns context.
   portforward = NetworkPolicy bypass to any reachable Service. binding =
   manual Pod-to-Node placement, scheduler bypass.

6. CSR approval and TokenRequest minting — certificatesigningrequests/approval
   subresource update, certificatesigningrequests with subject O=system:masters,
   serviceaccounts/token create on arbitrary SAs. CSR with system:masters CN
   = permanent cluster-takeover (cert valid until expiry, not revocable
   without rotating CA).

7. Manual Endpoints / EndpointSlices writes — race with EndpointSlice
   controller; transient man-in-the-middle of any selected Service during
   the window between manual write and reconciliation.

8. kube-system ConfigMap writes outside the resourceName-locked allowlist —
   cilium-config (CNI behavior), kube-proxy (mode), kubelet-config (node
   restart applies), cluster-info. CoreDNS is the one exception per the
   tight reload-and-verify protocol in permitted-mutations.md.

9. PriorityClass / IngressClass / Lease in kube-node-lease — system-cluster-
   critical / system-node-critical eviction-order corruption; IngressClass
   delete breaks ingress controller binding for every Ingress; Lease
   manipulation fakes node liveness in either direction.

References/least-privilege-rbac.yaml deliberately-omitted block expanded
to enumerate every new omission with the risk each addition would create.
The block is now grouped by category (Namespaces & finalizers, Workload
writes, Node lifecycle, Secrets & credentials, Cluster-extension surface,
Networking control plane, Storage, RBAC self-modification, Wildcards,
Cross-cutting verbs).

References/rbac-pre-flight.md must-not-be-yes matrix expanded with ~30
new check rows covering all 9 new refusal categories, plus a new
resourceName positive/negative test pattern: every resourceName-locked
binding must be tested with at least one positive (allowed name returns
yes) and two negatives (different name in same namespace returns no, same
name in different namespace returns no). This catches the silent
binding-drift failure where an operator adds extra resourceNames "for
convenience" without re-reading the deliberately-omitted block.

Docs/least-privilege-rbac.md threat model expanded:
- New failure mode 5 (credential offer): operator volunteers kubeconfig
  path or pastes token, agent's Read/Bash tool can act on it. The "never
  ask for credentials" rule does not by itself prevent receiving
  unsolicited credentials.
- New failure mode 6 (subresource and aggregation surprises): bindings
  thinking only verb-on-resource miss subresources (pods/exec, pods/portforward,
  pods/binding, nodes/proxy, */finalize) and aggregation surfaces
  (APIService, MutatingWebhookConfiguration).
- New failure mode 7 (resourceName drift): pre-flight self-check must
  perform negative tests at every session start.
- New "Prompt-level vs cluster-level enforcement" section clarifies that
  the refusal list is the prompt-level fast-path and the binding is the
  authoritative defense (deny-by-default). Operators choosing between
  rigour and convenience: list is for explainability, binding is for
  safety. If they disagree, the binding wins.

Credential-offer refusal clause propagated across all 7 harness adapters
+ AGENT.md + SKILL.md: agent uses only the in-pod ServiceAccount token at
/var/run/secrets/kubernetes.io/serviceaccount/token, refuses every other
credential source, including operator-provided kubeconfig paths, even
when the user insists "just this once."

Validation: 7/7 gates green; manifest regenerated.

Three assumptions explicitly identified as not fully holding and now
documented:
- The HARD REFUSE list cannot be exhaustive — Kubernetes adds APIs every
  release. The deny-by-default binding is the durable defense.
- kubectl auth can-i does not by default surface resourceNames constraints
  — explicit positive AND negative tests required.
- The "never ask for credentials" rule does not prevent receiving
  unsolicited ones — explicit refuse-on-receive added.
* **kubernetes-network-architecture:** patch 6 HIGH + 4 MEDIUM findings from ruthless eval
Eval source: .claude/evals/pr16-network-architecture.md
Specialists: architect, security-reviewer, harness-optimizer, gan-evaluator,
silent-failure-hunter (5 in parallel via Task tool, Sonnet workers)
Verdict before patches: FIX (6/8 adversarial scenarios pass; 6 HIGH + 4 MEDIUM)

HIGH patches (6):

1. Hard-stop scope refusal — agent must REFUSE entirely-out-of-scope questions
   instead of partial-answer + handoff note. (gan-evaluator + silent-failure-hunter
   + security-reviewer convergent finding.)
2. IMDS / 169.254.169.254 security warning obligation — surface metadata-service
   reachability as HIGH severity finding; recommend IRSA / Workload Identity /
   Pod Identity before any egress allow rule. (gan-evaluator scenario 5 + security
   reviewer Finding 2.) Includes troubleshooting-playbook callout.
3. kiro-cli.agent.json qualifier restore — restored "If policy correctness is the
   user's question," qualifying clause that was dropped, making the rule
   unconditional and over-broad. (harness-optimizer Finding 3.) Also full
   contract sync to match the 5 markdown adapters byte-for-byte equivalent.
4. CLI hallucination guard — explicit allowlist (kubectl, cilium, cilium-dbg,
   hubble, calicoctl, subctl, ip, conntrack, iptables, ipvsadm, nft, coredns)
   to prevent flag fabrication of the velero `--dry-run` class.
   (gan-evaluator scenario 3.)
5. Privileged debugger pod fix — replaced "from a privileged debugger pod" with
   `kubectl debug --profile=netadmin` and explicit `do NOT use --privileged`
   prohibition at the point of use, not only in mcp-and-evidence.md.
   (security-reviewer Finding 1.)
6. Per-finding evidence-level enforcement — every individual finding must carry
   its own evidence label, not just response-level. (silent-failure-hunter
   design Finding 1.)

MEDIUM patches (4):

7. Cilium ClusterMesh kvstore lag added as silent-failure mode in operating
   rules and the multi-cluster-and-egress.md table.
8. User-initiated mutation refusal — explicit operating rule for "just apply
   this for me" / credential offers.
9. topologyKeys removed (not deprecated) in K8s 1.27 — version gate added in
   service-gateway-routing.md so 1.26 clusters get migration warning before
   the upgrade.
10. Open assumptions field is mandatory — if CNI version, kube-proxy mode,
    IPAM mode, node MTU, or DNS pod count were not confirmed by live evidence,
    each MUST appear; field is no longer structurally optional.

Files patched (13):
- AGENT.md (canonical) + 5 markdown harness adapters (claude-code, copilot,
  cursor, gemini, kiro-ide) + kiro-cli.agent.json + codex.toml safety contract
  — all carry the same Operating Rules + Response Shape contract
- SKILL.md (out-of-scope hard refusal + 7 lean rule additions/tightenings +
  response minimum tightening for evidence-per-finding and required assumptions)
- references/troubleshooting-playbook.md (IMDS HIGH callout + privileged-pod fix)
- references/multi-cluster-and-egress.md (ClusterMesh kvstore lag entry)
- references/service-gateway-routing.md (topologyKeys 1.27 removal version gate)
- catalog/skill-manifest.json regenerated

Validation:
- npm run validate: 7/7 gates pass (catalog 285, skills 139, agents 142,
  manifest 139, allowed-tools 139, schemas, links 630)
- 5 markdown harnesses verified byte-identical from "## Operating Rules" onward

Architect's CRITICAL finding (phantom delegate skills/agents) was a false
positive — the delegates live at skills/cilium/, skills/istio/, agents/cilium/,
agents/istio/, not under skills/kubernetes/ / agents/kubernetes/. Catalog
wiring is consistent.

Deferred to v0.2.0 (LOW or judgment-dependent):
- codex.toml model_reasoning_effort: high → medium (cost; matches sibling pattern)
- metadata.json harnesses[] enum collapses kiro-ide / kiro-cli into "kiro"
- submariner.io qualification as project doc not upstream
- CNI + service mesh dataplane co-existence coverage (architect's "missed" item)

### feat

* add kind-based RBAC pre-flight CI integration test
tests/integration/rbac-pre-flight/ provides a regression harness for all 7
live-guard least-privilege RBAC manifests. Runs the full kubectl auth can-i
must-not/must-be-yes matrix against a real kind cluster across 4 Kubernetes
versions (1.28–1.31).

Triggered by any change to least-privilege-rbac.yaml or rbac-pre-flight.md
files, ensuring the RBAC posture is validated as Kubernetes evolves.
* add L4 admission policies (Kyverno + VAP) for live-guard defense-in-depth
docs/admission-policies/ ships Kyverno ClusterPolicies and Kubernetes-native
ValidatingAdmissionPolicies (VAP, GA 1.30) as Layer 4 complement to the Layer 3
RBAC manifests already shipped by each live-guard.

Policies cover: namespace delete, CRD delete, finalizer-strip, kube-system exec,
MutatingWebhookConfiguration/ValidatingWebhookConfiguration writes, APIService writes.

Grounded against the 5-layer defense model in docs/least-privilege-rbac.md.
* **kubernetes-live-guards:** retrofit 6 existing live-guards with shared least-privilege RBAC pattern
Closes the inconsistency identified by the user: the 6 existing live-guard
agents (rbac-mutation, network-policy, mesh-policy, admission-policy,
argocd-sync, velero-restore) had no RBAC manifests, no kubectl auth can-i
pre-flight, and no enumerated refusal list — only prompt-level guardrails.
Anyone running them today was relying on the LLM behaving correctly. With
this commit, every live-guard in this repo enforces the same 5-layer
defense model documented in docs/least-privilege-rbac.md.

What each retrofitted guard now ships
* **kubernetes-maestro:** wire kubernetes-network-architecture-review-agent into routing
Maestro is the orchestrator that dispatches Kubernetes specialists in parallel
teams (max 4) and synthesizes findings. Without this patch, the new
* **kubernetes:** add live network-architecture mutation guard + shared least-privilege RBAC contract
This is the live-mutation counterpart to kubernetes-network-architecture-review-agent
* **kubernetes:** add network-architecture-review agent and skill
Fills the architecture-review gap in the kubernetes-network-engineer role
bundle. The role's existing agents are policy-focused (Cilium, Istio, live
policy guards); this adds a read-only design-tier agent for CNI choice,
kube-proxy mode, IPAM and CIDR sizing, MTU and encapsulation, dual-stack,
the Service surface (EndpointSlices, internalTrafficPolicy,
externalTrafficPolicy, topology-aware routing), Ingress to Gateway API
migration, CoreDNS and NodeLocal DNSCache, multi-cluster topology
(ClusterMesh, Submariner, MCS-API), egress topology, and connectivity
troubleshooting.

Scope is bounded by explicit delegation: NetworkPolicy content goes to
cilium-network-policy-review, mesh policy to istio-ambient-mesh-review,
live mutations to the existing live-guard agents, pod-spec to
kubernetes-pod-spec-review.

Grounded in upstream documentation (Kubernetes services-networking,
Gateway API, Cilium, CoreDNS) — the Linux Foundation CKNE program has not
yet published curriculum domains as of last_verified, which is disclosed
in the skill's official-sources reference.

### docs

* **eval:** add eval-harness artifact for PR #16 network-architecture review
Defines capability evals (8), regression evals (8), and adversarial evals (8)
for the kubernetes-network-architecture-review agent + skill. Used by the
ruthless-orchestrator dispatch to gate SHIP/FIX/BLOCK on the PR.
* **kubernetes-network-architecture:** ground patches against Context7 upstream docs
Verified the patched claims against authoritative upstream documentation via
Context7 MCP (kubernetes.io, gateway-api.sigs.k8s.io, docs.cilium.io).

Confirmed correct (no edit needed):
- GRPCRoute is GA / Standard channel since Gateway API v1.1.0
  Source: gateway-api.sigs.k8s.io/api-types/grpcroute
- service.kubernetes.io/topology-mode: Auto is the correct replacement
  for topologyKeys
  Source: kubernetes.io/docs/concepts/services-networking

Upgraded with upstream-grounded content (2 references):

1. multi-cluster-and-egress.md — ClusterMesh row now cites the *real* silent
   failure modes from docs.cilium.io rather than my generic "kvstore lag":
   - --clustermesh-cache-ttl defaults to 0s which per upstream means "the
     cache is never revoked" when connectivity to a remote cluster is lost.
     Stale ServiceImports continue serving removed endpoints indefinitely.
   - --global-ready-timeout defaults to 10m — clusters report ready even
     if remote sync has not converged.
   - Replaced my generic `cilium-dbg kvstore get` recommendation with the
     correct upstream-documented commands:
     - cilium-dbg troubleshoot clustermesh (direct mode)
     - clustermesh-apiserver kvstoremesh-dbg troubleshoot (KVStoreMesh mode)

2. service-gateway-routing.md — topology-aware routing section now
   documents all three API generations rather than just two:
   - topologyKeys (removed 1.27)
   - service.kubernetes.io/topology-mode: Auto (current; may itself be
     deprecated per upstream)
   - spec.trafficDistribution field (KEP-4444; newest)
   Per kubernetes.io: "If `service.kubernetes.io/topology-mode` is set to
   `Auto`, it overrides the `trafficDistribution` field."
   This was missing from the earlier patch — the upstream API has moved.

Validation: 7/7 gates green; manifest regenerated.
Context7 calls used: 3/3 resolve, 3/3 query (within per-question limits).

## 🛡️ v1.4.0 — *Provenance, Policy, Portability* &mdash; 2026-05-06

> _Multi-cloud agent marketplace · `AWS` · `Azure` · `OCI` · `Terraform`_
>
> Built for operators on the cloud frontier — least privilege, live evidence, safe rollback paths.


* Merge pull request #10 from Raishin/dependabot/github_actions/actions-937d73b4db
chore(actions): bump github/codeql-action from 3.35.3 to 4.35.3 in the actions group
* Merge pull request #11 from Raishin/dependabot/npm_and_yarn/npm-dev-a75136ff7c
chore(deps-dev): bump conventional-changelog-conventionalcommits from 8.0.0 to 9.3.1 in the npm-dev group
* Merge pull request #12 from Raishin/claude/marketplace-hardening-batch2
feat: SLSA attestations + AGENT.md schema + Scorecard + docs-quality + skill taxonomy
* Merge pull request #13 from Raishin/claude/marketplace-hardening-batch3
feat: skill taxonomy backfill + branch protection as code + cross-harness design
* Merge pull request #14 from Raishin/claude/fix-ip-address-xss-cve
fix(security): scope GITHUB_TOKEN least-privilege; recover release workflow
* Merge pull request #15 from Raishin/claude/fix-releaserc-immutable-commit
fix(release): unblock v1.4.0 — clone frozen commit in writerOpts.transform
* Merge pull request #9 from Raishin/claude/submit-skills-marketplace-TkqCg
feat: marketplace governance + companion-skill bundling + least-privilege skill surface

### fix

* **docs:** codespell typo additon -> addition
* **governance:** allow merge commits, drop linear-history requirement
Branch protection ruleset now requires merge-commit-only merges and
removes the linear-history rule. Each PR's full commit history is
preserved on master.

- allowed_merge_methods: ["merge"]
- required_linear_history rule removed
- docs/branch-protection.md updated to match
* **governance:** correct apply-ruleset token + ci.yml PR trigger
Two issues caught by automated review on #13:

1. apply-ruleset.yml asked for 'administration: write' on GITHUB_TOKEN,
   which is not a valid permission key. Switched to a required
   RULESET_ADMIN_TOKEN secret (PAT or GitHub App installation token
   with Administration: read & write). Workflow fails fast if missing.

2. ci.yml pull_request trigger was scoped to 'branches: [main]' but
   default branch is master. Forked PRs targeting master would never
   produce the 'validate' check, blocking external contributions
   under the new ruleset. Pull-request trigger now matches all bases;
   push trigger now scoped to master.
* **release:** return new commit object in writerOpts.transform
conventional-changelog-writer v8 freezes commit objects, so mutating
commit.body directly throws "Cannot modify immutable object" and aborts
the generateNotes step. This blocked the v1.4.0 release after PR #14.

Return a shallow copy with the cleaned body instead.

Stack from failed run:
  Object.set (conventional-changelog-writer/dist/commit.js:15:19)
  transform (.releaserc.js:60:25)
  transformCommit (conventional-changelog-writer/dist/commit.js:31:29)
* **security:** scope GITHUB_TOKEN least-privilege; recover release workflow
Address three Scorecard Token-Permissions findings and the missing
v1.4.0 release.

Workflow token-permission hardening (Scorecard Token-Permissions):
* **skills:** velero-backup-restore-guard category from security to resilience
Backfill rules ranked 'guard' above 'backup'/'restore' keywords; the skill is
data-protection scope, not adversarial defense.

### ci

* add markdownlint + codespell docs-quality gates
* add OpenSSF Scorecard workflow
* **install:** add smoke test for documented vfa-export-agents paths
* **install:** bump smoke test runtime to Node 24
setup-node v6 supports Node 24 directly (lts/jod).  No code changes
required in scripts/export-marketplace-agents.mjs.
* **release:** add manual dispatch and verbose diagnostics
The release workflow has been silent across the last three master
pushes (no v1.4.0 tag, no npm publish). Without log access this is
impossible to diagnose, so this commit makes the pipeline both
manually triggerable and self-explaining on the next run.

Changes:
- Add workflow_dispatch trigger with a `dry_run` choice input. A
  maintainer can now re-run the release without producing a no-op
  commit, and can preview the bump via dry-run before letting the
  real run push tags and publish to npm.
- Add a Pre-release diagnostics step that prints (without leaking
  secrets):
    - Whether NPM_TOKEN is configured (boolean only)
    - `git remote -v` and last 10 commits
    - `git log <last-tag>..HEAD` so commit-analyzer's input is visible
    - Sanitised view of package.json identity (name/version/
      repository/publishConfig)
- Pass `--debug` to semantic-release so plugin-level failures
  (auth, git push, npm publish) surface in the run log instead of
  being hidden behind the default "no relevant changes" exit-0.
- Honor the dry_run input so semantic-release runs with --dry-run
  when invoked manually for forensics.

Once this lands and the workflow runs (either via the PR merge or via
manual dispatch on master), the log will show exactly which step is
failing or why semantic-release decides no version bump is warranted.
* **release:** add SLSA provenance attestations and SBOM signing
* **security:** add CodeQL workflow for JS and Python
Adds a GitHub Actions CodeQL workflow scanning javascript-typescript and
python on push to master, pull_request, and a weekly Monday schedule.
Uses security-extended and security-and-quality query suites. All action
references are SHA-pinned with version comments.

### chore

* **actions:** bump checkout/setup-node/setup-python to v6
Bumps GitHub Actions across all three workflow files in lockstep:

- actions/checkout: v4.2.2 -> v6.0.2 (de0fac2e)
- actions/setup-node: v4.4.0 -> v6.4.0 (48b55a01)
- actions/setup-python: v5.5.0 -> v6.2.0 (a309ff8b)

SHAs resolved via git ls-remote against each upstream and pinned per the
existing # vX.Y.Z comment-alias convention. YAML still parses cleanly.
* **actions:** bump github/codeql-action in the actions group
Bumps the actions group with 1 update: [github/codeql-action](https://github.com/github/codeql-action).

Updates `github/codeql-action` from 3.35.3 to 4.35.3
- [Release notes](https://github.com/github/codeql-action/releases)
- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)
- [Commits](https://github.com/github/codeql-action/compare/0daab03d71ff584ef619d027a3fd9146679c5d84...e46ed2cbd01164d986452f91f178727624ae40d7)
* **deps-dev:** bump conventional-changelog-conventionalcommits
Bumps the npm-dev group with 1 update: [conventional-changelog-conventionalcommits](https://github.com/conventional-changelog/conventional-changelog/tree/HEAD/packages/conventional-changelog-conventionalcommits).

Updates `conventional-changelog-conventionalcommits` from 8.0.0 to 9.3.1
- [Release notes](https://github.com/conventional-changelog/conventional-changelog/releases)
- [Changelog](https://github.com/conventional-changelog/conventional-changelog/blob/master/packages/conventional-changelog-conventionalcommits/CHANGELOG.md)
- [Commits](https://github.com/conventional-changelog/conventional-changelog/commits/conventional-changelog-conventionalcommits-v9.3.1/packages/conventional-changelog-conventionalcommits)
* **deps:** enable dependabot for actions and npm
Add .github/dependabot.yml with weekly update schedules for
github-actions (grouped, single PR) and npm (split runtime/dev groups),
both with a 5 PR limit and auto-reviewer Raishin.
* **gitignore:** exclude local .claude/worktrees subagent dirs
* **governance:** add CODEOWNERS file with provider-scoped review
Establishes explicit ownership for all 18 provider directories under
agents/ and skills/, plus critical infra paths (.github/, scripts/,
schemas/, catalog/, tests/, CLAUDE.md), with a default fallback rule.
* **release:** bump package version to 1.4.0
Align in-tree version with the v1.4.0 tag that semantic-release will
publish on the next master release run. semantic-release will still
manage future bumps via @semantic-release/npm.
* **release:** enable npm provenance on publish
Add `provenance: true` to publishConfig in package.json. The release
workflow already grants `id-token: write`, which satisfies the OIDC
requirement; this field makes the intent explicit and ensures
@semantic-release/npm emits a provenance attestation on every publish
regardless of auto-detection heuristics.
* **release:** set v1.4.0 title — Provenance, Policy, Portability
Wire the chosen release title into the release-notes-generator
headerPartial so it renders automatically on the v1.4.0 GitHub release
and CHANGELOG entry.
* **security:** add SECURITY.md with disclosure policy and SLA
Replaces the placeholder security note with a full vulnerability
disclosure policy covering supported versions, private reporting via
GitHub Security Advisories, response SLA (5/10/90 day), scope,
safe harbor, and researcher acknowledgement. Adds a "Reporting
security issues" nav link in README.md.

### test

* **exporter:** add cursor and kiro silent-skip notice test
Verifies SKIP_SKILLS_PLATFORM_NOTICES emits harness-specific stderr for
cursor, kiro, kiro-ide, and kiro-cli, and that no skill directory is
created. Wired as `npm run test:cursor-kiro-notices` in package.json
(committed alongside in 8373daa).

Design rationale: docs/cross-harness-skills.md lines 112-159.

### feat

* **cli:** bundle companion skills with agent export by default
vfa-export-agents now installs each agent's same-named SKILL.md
companion alongside the agent when --platform=claude-code, so
agents get the skills they reference without a second install step.

- Default: pair by id (<name>-agent ↔ <name> skill); 134/141 paired
- --all: also bundles all 138 skills (covers 4 orphan skills with
  no agent peer)
- --role: respects role.skills from catalog/install-roles.json
- --no-skills: opt out, with explicit confirmation
- Other platforms (cursor/codex/copilot/gemini/kiro): print
  "not yet supported" notice instead of silent gap
- Stderr summary: bundled count, agent count, no-skill agents listed

Pairing is name-based; long-term should move to explicit
companion_skills field in agent metadata.json.
* **exporter:** bundle skills to .gemini/skills for --platform gemini
Add `gemini: ".gemini/skills"` to SKILLS_PLATFORM_CONFIG so that
--platform gemini bundles companion skills byte-for-byte into the Gemini
CLI skill directory. Gemini CLI is verified byte-compatible with the
Claude Code skill format per docs/cross-harness-skills.md (lines 191-197).

Update usage() to drop the "claude-code only" caveat and list all three
supported platforms (claude-code, copilot, gemini). Add
test-gemini-skill-bundling.py (TDD: RED then GREEN) and wire it as
npm run test:gemini-bundling. CHANGELOG updated under Unreleased Batch 3.
* **governance:** branch protection ruleset as code
Add a declarative GitHub Repository Ruleset for the master branch and a
manual-dispatch workflow that applies it idempotently via gh api. The
ruleset blocks deletion, force-push, and creation; requires linear
history; requires a PR with code-owner review and stale-review
dismissal; and gates merge on the six PR-time CI contexts (validate,
smoke, Analyze (javascript-typescript), Analyze (python), markdownlint,
codespell). Scorecard analysis is excluded because it runs post-merge.
Documented the apply, update, and bypass procedures.
* **metadata:** declarative companion_skills field for agent-skill pairing
Replace fragile name-stripping heuristic with explicit companion_skills
array in agent metadata. The export CLI now prefers declared pairings
over the <name>-agent -> <name> convention.

Schema:
- Add optional companion_skills: string[] to schemas/agent.schema.json
  with id pattern validation. Empty array means intentional no-pair.

Migrated 6 of 7 previously-orphan agents to declarative pairings:
- kubernetes-psa-review-agent -> kubernetes-pod-security-admission-review
- kubernetes-live-velero-restore-guard-agent -> velero-backup-restore-guard
- kubernetes-live-admission-policy-guard-agent -> kyverno-policy-review
- kubernetes-live-argocd-sync-guard-agent -> argocd-gitops-review
- kubernetes-live-mesh-policy-guard-agent -> istio-ambient-mesh-review
- kubernetes-live-network-policy-guard-agent -> cilium-network-policy-review

terraform-reviewer left without companion_skills (no clear 1:1 skill peer).

Script:
- loadAgents() now reads companion_skills from metadata
- resolveCompanionSkills() prefers explicit array; falls back to
  name-stripping only when field is absent
- companion_skills: [] is treated as intentional no-pair, not orphan

npm run validate passes (138 skills, manifest, 594 URL links).
* **schema:** add AGENT.md frontmatter JSON Schema + validator
Formalize the YAML frontmatter contract for the 141 AGENT.md files with
a Draft 2020-12 schema (schemas/agent.frontmatter.schema.json) and a
stdlib-only validator (tests/validate-agent-frontmatter-schema.py),
mirroring the existing SKILL.md pattern. Required fields are derived
empirically from the corpus (metadata.author + metadata.version);
additionalProperties is left open so harness-specific fields stay
non-breaking. Wire validate:agent-schema into the npm validate pipeline
as a 7th gate, document the schema in schemas/AGENTS.md, and backfill
the missing metadata.version on agents/terraform/terraform-reviewer
(canonical 0.1.0 from its metadata.json).
* **schema:** add skill metadata taxonomy fields (category, lifecycle, updated)
Schema-only addition. All three fields are optional and validate when
present. No SKILL.md backfill in this batch — populating per-skill
category and updated dates is a deliberate follow-up exercise.

- metadata.updated: ISO 8601 date pattern
- metadata.category: 10-value enum, see docs/taxonomy.md
- metadata.lifecycle: experimental | beta | stable | deprecated
* **schema:** add SKILL.md frontmatter JSON Schema + CI validator
Introduces schemas/skill.frontmatter.schema.json (Draft 2020-12) with
required fields name, description, allowed-tools, metadata.author, and
metadata.version, plus optional disable-model-invocation. Adds
tests/validate-skill-frontmatter-schema.py (TDD-style, with jsonschema
fallback to hand-rolled validation) and wires validate:skill-schema into
the validate script between validate:allowed-tools and validate:links.
All 138 skills pass.
* **skills:** backfill metadata.updated and metadata.category on 138 skills
* **skills:** declare allowed-tools per skill (TDD, least-privilege)
Adds the Claude Code SKILL.md `allowed-tools` field to every skill in the
catalog, defaulting to least privilege.

Why: making the tool surface explicit aligns each skill with the canonical
Claude Code skills spec at https://code.claude.com/docs/en/skills and
makes review of write-capable skills tractable. Note: `allowed-tools` is
a pre-approval list (not a deny-list); harness deny rules in settings.json
remain the enforcement boundary, but declaring intent here makes review
auditable.

Distribution across 138 skills:
  87  Read Grep Glob                              (advisory / review)
  38  Read Grep Glob WebFetch                     (investigators / live guards)
   5  Read Edit Write MultiEdit Grep Glob         (in-repo patchers)
   5  Agent Skill Read Grep Glob                  (maestros / dispatchers)
   3  Read Edit Write MultiEdit Grep Glob Bash    (developers / agentcore)

TDD discipline:
- tests/validate-skill-allowed-tools.py written first (RED: 138 missing).
- scripts/apply-skill-allowed-tools.py applies a deterministic taxonomy
  matched against skill id; idempotent (skips skills that already declare).
- New step wired into `npm run validate` as `validate:allowed-tools`.

Cross-platform note: SKILL.md is a Claude Code artifact in this repo;
non-Claude harness exports do not consume SKILL.md frontmatter, so this
field is harmless for codex/copilot/cursor/gemini/kiro consumers.

### docs

* changelog entry for batch 3
* cross-harness skill adapter design
* **governance:** add CONTRIBUTING and issue/PR templates
Replaces the stub CONTRIBUTING.md with a full contributor guide covering
Quick Start, asset types, skill and agent directory layout, required
frontmatter fields (including allowed-tools and companion_skills),
all npm run validate gates, catalog refresh workflow, provenance rules,
PR expectations, and links to CODE_OF_CONDUCT.md and SECURITY.md.

Adds .github/PULL_REQUEST_TEMPLATE.md with Summary, Type of change
checkboxes, validation evidence slot, Risk/rollback, and a merge
checklist. Adds YAML-form issue templates for bug reports and
skill/agent proposals, plus config.yml that disables blank issues
and routes security reports to SECURITY.md.
* **integrations:** extract skills CLI guidance into trust-matrix doc
Move the 20-line skills CLI subsection out of README into
docs/integrations/skills-cli.md. The new page covers all three install
paths with a trust matrix, verified flag syntax for vercel-labs/skills,
a pinning note (unpinnable at HEAD), and a "Before you install" section
covering SKILL.md frontmatter inspection and references/ review.
README now carries a 4-line pointer and keeps the canonical npm install
command for quick-start users.
* **readme:** document skills.sh / npx skills install path
Surface that all 138 SKILL.md artifacts in this repo are installable via the
open `skills` CLI and auto-indexed at skills.sh, so users on Claude Code,
Codex, Cursor, OpenCode, Gemini CLI, Kiro, etc. can discover and install
them without a separate submission step.
* sync CHANGELOG, CLAUDE.md, AGENTS.md for batch 2 gates
* sync README badges, CoC, CHANGELOG, CLAUDE.md, AGENTS.md
Documentation sync for the marketplace governance batch:

- README: add badge row (npm, license, CodeQL, smoke test, npm
  provenance, PRs welcome) and link CONTRIBUTING.md, SECURITY.md,
  CODE_OF_CONDUCT.md from the top nav
- CODE_OF_CONDUCT.md: adopt Contributor Covenant 2.1 by reference,
  retain the project's terse standard, route reports through SECURITY.md
- CLAUDE.md: list the six-gate validate pipeline, document the
  allowed-tools and companion_skills requirements, expand the important
  files map (CONTRIBUTING / SECURITY / CoC / schemas / integrations doc)
- AGENTS.md: update workflows section with the six-gate validate
  pipeline and the new skill-bundling flags on vfa-export-agents
- CHANGELOG.md: add an Unreleased preview describing the batch; will
  be superseded by semantic-release output on master push

### build

* convert releaserc to JS and strip claude.ai session URLs from release notes
.releaserc.json → .releaserc.js so writerOpts.transform can scrub
claude.ai/code/session_* attribution lines from commit bodies before
they render in the GitHub release page and CHANGELOG.md.

## 🚧 Unreleased — Batch 3 (stacked on Batch 2)

> _Skill taxonomy backfill, branch protection as code, cross-harness skill design._

### ✨ Features

* All 138 SKILL.md files backfilled with `metadata.updated` (derived from git log) and `metadata.category` (deterministic classifier in `scripts/backfill-skill-metadata.py`)
* Idempotent backfill script: re-running reports 0 of 138 updates
* `export-marketplace-agents.mjs`: `--platform cursor` and `--platform kiro` (all variants) now emit harness-specific skill-skip notices explaining that Cursor uses Project Rules and Kiro uses Steering files — neither is a skill primitive. This replaces the generic "not yet supported" message with an explicit, permanent-design-decision notice. See `docs/cross-harness-skills.md` for the full rationale.
* `export-marketplace-agents.mjs`: `--platform gemini` now bundles companion skills into `.gemini/skills/` (Gemini CLI is byte-compatible with Claude Code skill format per `docs/cross-harness-skills.md`)
* `export-marketplace-agents.mjs`: `--platform copilot` now bundles companion skills into `.github/skills/` (GitHub Copilot VS Code is byte-compatible with Claude Code skill format per `docs/cross-harness-skills.md` lines 67-74, 198-214)

### 🛡️ Governance

* Branch protection as code: declarative ruleset in `.github/rulesets/master.json` applied via dispatch-only `apply-ruleset.yml` workflow
* Required CI checks enforced on master: validate, smoke, CodeQL (JS/TS + Python), markdownlint, codespell
* Linear history + force-push and deletion blocked; CODEOWNERS review required
* Documented in `docs/branch-protection.md`

### 📚 Documentation

* `docs/cross-harness-skills.md` — empirical, doc-cited design for skill bundling on Gemini CLI, GitHub Copilot, Codex CLI, Cursor, Kiro
* Per-harness conclusion: Gemini and Copilot are byte-compatible today (next adapter PRs); Cursor and Kiro have no skill primitive (silent-skip/notice); Codex needs project-level path verification

### 📊 Skill category distribution

security 31 · delivery 27 · platform 20 · ai 11 · compliance 10 · observability 10 · data 9 · finops 9 · networking 6 · resilience 5

---

## 🚧 Unreleased — Batch 2

> _Supply-chain attestations, docs-quality gates, AGENT.md schema, and skill taxonomy fields._

### ✨ Features

* AGENT.md frontmatter JSON Schema (`schemas/agent.frontmatter.schema.json`) and `validate:agent-schema` gate (now 7 validation gates)
* Skill metadata taxonomy: optional `metadata.category` (10-value enum), `metadata.lifecycle`, and `metadata.updated` fields declared in the skill schema; documented in `docs/taxonomy.md`

### 🔒 Security and supply chain

* OpenSSF Scorecard workflow with weekly cadence, SARIF upload, and `publish_results: true`
* SLSA Build L3 GitHub artifact attestations on release (npm tarball + SPDX SBOM via anchore/sbom-action)
* Release artifact verification steps documented in SECURITY.md (`gh attestation verify`)

### 🧪 Quality gates and CI

* Markdownlint (correctness-only ruleset) and codespell as a separate `Docs Quality` workflow on push and PR
* Advisory `lint:md`, `lint:spell`, `lint:docs` npm scripts (not blocking the validate gate)

### 📚 Documentation

* README badges: OpenSSF Scorecard, Docs Quality
* `docs/taxonomy.md` extended with skill categories, lifecycle, and updated-date contract

---

## 🚧 Batch 1 (merged)

> _Marketplace governance, supply-chain provenance, and least-privilege skill surface._
>
> Auto-generated release notes will replace this section on the next semantic-release run; this preview reflects what is on the release branch.

### ✨ Features

* `vfa-export-agents` bundles companion skills by default on `--platform claude-code`; pairing resolved from `companion_skills` metadata, name-stripping fallback, `--no-skills` opts out
* `--all` exports every catalogued skill, including 4 skills with no agent peer
* New optional `companion_skills` field on agent metadata; six previously orphan agents migrated
* New `allowed-tools` field on every SKILL.md, classified by skill role (least-privilege baseline)
* JSON Schema (Draft 2020-12) for SKILL.md frontmatter at `schemas/skill.frontmatter.schema.json`

### 🔒 Security and supply chain

* `SECURITY.md` with disclosure policy, response SLA, scope, and Safe Harbor
* CodeQL workflow (JavaScript and Python) on push, PR, and weekly schedule
* npm provenance attestations enabled on publish
* Dependabot enabled for GitHub Actions and npm with weekly grouped PRs

### 🧭 Governance and contributor experience

* `CODEOWNERS` with provider-scoped review across 18 provider domains
* `CONTRIBUTING.md` and structured issue / pull-request templates
* `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)

### 🧪 Schema, validation, and CI

* New `validate:skill-schema` and `validate:allowed-tools` gates in `npm run validate`
* Install-paths smoke test asserts documented commands behave as documented
* GitHub Actions bumped to v6 (`actions/checkout@v6.0.2`, `actions/setup-node@v6.4.0`, `actions/setup-python@v6.2.0`); smoke test runs on Node 24

### 📚 Documentation

* `docs/integrations/skills-cli.md` — install-path trust matrix (npm, `vfa-export-agents`, third-party `skills` CLI)
* `CLAUDE.md`, `AGENTS.md`, `README.md` synced with the new validation gates and metadata fields

### 💥 Breaking changes

* None. `--no-skills` is provided for callers that want the previous agents-only behaviour.

---

## 🛡️ v1.3.0 &mdash; 2026-05-02

> _Multi-cloud agent marketplace · `AWS` · `Azure` · `OCI` · `Terraform`_
>
> Built for operators on the cloud frontier — least privilege, live evidence, safe rollback paths.


### ✨ Features

* add 12 K8s agents + kubernetes-maestro skill across CNCF domains (3b2a005)
Review agents (7):
- kyverno-policy-review-agent (provider: kyverno)
- argocd-gitops-review-agent (provider: argocd)
- istio-ambient-mesh-review-agent (provider: istio)
- cilium-network-policy-review-agent (provider: cilium)
- opentelemetry-collector-config-review-agent (provider: opentelemetry)
- kubernetes-workload-identity-review-agent
- kubernetes-psa-review-agent

Live-guard agents (4) — require explicit human confirmation before any mutation:
- kubernetes-live-admission-policy-guard-agent (Kyverno/VAP guard)
- kubernetes-live-argocd-sync-guard-agent (ArgoCD sync/AppProject guard)
- kubernetes-live-mesh-policy-guard-agent (Istio AuthorizationPolicy/PeerAuthentication guard)
- kubernetes-live-network-policy-guard-agent (Cilium/NetworkPolicy guard)

Maestro (1):
- kubernetes-maestro-agent — per-platform router with live-guard gate

Skills:
- kubernetes-maestro skill with routing table (13 agents), multi-domain dispatch
  examples, and safety-checklist with 10-item pre-dispatch checklist and
  per-agent-type post-mutation verification commands

Catalog:
- catalog/agents.json: 115 → 127 entries
- catalog/skills.json: 122 → 123 entries
- catalog/install-roles.json: all 4 K8s roles now have agents assigned
  (kubernetes-network-engineer and kubernetes-application-platform-engineer
  had 0 agents before this commit)
- catalog/skill-manifest.json: regenerated (123 skills, 517 URLs validated)

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* add 14 new CNCF/cloud-native agents and 15 skills (Golden Kubestronaut gap fill) (c40aadb)
Adds the remaining Golden Kubestronaut certification domain coverage:
- Prometheus (PCA): alerting rules, recording rules, cardinality review
- Falco (CNPA/CNCF): runtime threat rules, macro exceptions, SIEM routing
- Sigstore/Cosign (CNPA/CNCF): supply chain review, SBOM attestation, Rekor
- cert-manager (CKCSA/CKS): Issuer/ClusterIssuer scope, CertificateRequestPolicy gap
- Argo Rollouts (CAPA): progressive delivery, AnalysisTemplate, PDB deadlock
- FluxCD (CGOA): Kustomization, HelmRelease, SOPS, multi-tenant GitOps
- Backstage (CBA): Scaffolder template blast-radius, RBAC gate, input injection
- Velero (CKA/KCNA): live-guard restore with 10-item pre-restore safety checklist
- AWS/Azure/OCI cert-manager PKI (CKS): cloud-backed CA issuer review (3 agents)
- kubernetes-pod-spec-review: securityContext, capabilities, readOnlyRootFilesystem
- kubernetes-external-secrets-operator-review: ESO scope, auth, PushSecret
- kubernetes-kubecost-chargeback-allocation-review: cost attribution and label taxonomy

New files: 14 agents (each with 7 harnesses), 15 skills, 7 provider READMEs
Updated: catalog/agents.json (127→141), catalog/skills.json (123→138),
  catalog/skill-manifest.json, catalog/install-roles.json (+6 new K8s roles),
  tests/validate-catalog.py (7 new ALLOWED_PROVIDERS + velero live-guard),
  tests/validate-aws-skill-quality.py, AGENTS.md, kubernetes/README.md, README.md

All validations pass: validate:catalog (283 entries), validate:aws (44 skills),
  manifest:check (138 skills), validate:links (592 URLs)

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* add 5 CNCF skill domains (Kyverno, Istio, ArgoCD, Cilium, OTEL) and 4 K8s-specialized roles (2c6963d)
Add seven new progressive-disclosure skills and four K8s-specialized roles to extend
the marketplace beyond core Kubernetes RBAC into the broader CNCF ecosystem. Skills
load only when needed via the standard SKILL.md + references pattern, keeping token
usage low while making detailed step-by-step reviews available on demand.

New top-level skill domains:
- skills/kyverno/kyverno-policy-review (admission policy review, Cosign image verification, Kyverno-vs-native-CEL decision)
- skills/argocd/argocd-gitops-review (Application/AppProject/ApplicationSet, sync impersonation, drift handling, Argo CD Agent)
- skills/istio/istio-ambient-mesh-review (sidecar + ambient with L7-without-waypoint trap, PeerAuthentication mTLS posture)
- skills/cilium/cilium-network-policy-review (three policy formats, ClusterMesh policy-default-local-cluster, EgressGateway IP collisions)
- skills/opentelemetry/opentelemetry-collector-config-review (four deployment modes, memory_limiter and k8sattributes mandatory checks)

Two new core kubernetes skills:
- skills/kubernetes/kubernetes-workload-identity-review (IRSA, Azure WI, GKE WI, OIDC trust-policy scope)
- skills/kubernetes/kubernetes-pod-security-admission-review (PSA profiles, modes, version pinning, exemptions)

Four new Kubernetes-specialized roles in catalog/install-roles.json:
- kubernetes-admission-security-engineer
- kubernetes-network-engineer
- kubernetes-application-platform-engineer
- kubernetes-runtime-security-engineer

Plus extends ALLOWED_PROVIDERS in tests/validate-catalog.py to include kyverno, istio, argocd, cilium, opentelemetry as first-class CNCF tool domains.

References include URL-rich step-by-step workflows grounded in Context7 lookups against official documentation.

Catalog totals: 241 entries (up from 234), 122 skills (up from 115), 497 URLs validated.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* add live guard agents for Azure Entra, OCI network, and Kubernetes RBAC (b1edcfb)
Three new guarded live-operator skills and agents targeting the highest-blast-radius,
highest-human-judgment operations across three providers.

## azure-live-entra-role-assignment-guard
Covers the gap between the existing PIM JIT guard (JIT activations) and the
unguarded permanent assignment path. Owner/Contributor/UAA permanent assignments
* add role-based install pattern, evidence output spec, and CI/CD enforcement (dfc9a15)
- catalog/install-roles.json: six cross-provider roles (cloud-security-engineer,
  cloud-platform-engineer, cloud-dba, cloud-finops-analyst, cloud-solutions-architect,
  cloud-devops-engineer) each mapping to curated agent + skill ID lists across
  AWS, Azure, OCI, and Kubernetes; extensible for future roles and providers

- docs/evidence-output-spec.md: formal mapping of the five required response fields
  (verdict, evidence_level, blockers, safe_next_actions, open_questions) to SOC 2 CC6.1,
  PCI DSS Req 7, NIS2 Article 21, NIST CSF PR.AC-4, and ISO 27001 A.9.1.1;
  documents the three enforcement layers (BEFORE/AT/AFTER) and five critical
  Fortune 50 decision points covered by the live-guard agents

- docs/ci-cd-enforcement-pattern.md: GitHub Actions, Azure DevOps, and OCI DevOps
  pipeline templates for BEFORE/AT/AFTER enforcement without developer opt-in;
  includes evidence artifact retention guidance per SOC2/PCI/ISO/NIS2 requirements

- scripts/export-marketplace-agents.mjs: --role <role-id> flag resolves agent IDs
  from install-roles.json and exports them in one command; --provider filter scopes
  to a single cloud provider; --list-roles lists available roles with counts

- README.md: role-based install section with --role usage examples, CLI table updated,
  compliance compass updated with evidence output spec reference
- AGENTS.md: role-based pattern section, change rules updated to require install-roles
  and evidence output spec compliance when adding new agents

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **catalog:** add 26 Kubernetes + CNCF agents covering all Golden Kubestronaut domains (a14cd28)
New agents by certification domain:

| Agent | Provider | Domain |
|---|---|---|
| `kubernetes-rbac-review-agent` | kubernetes | CKA / RBAC |
| `kubernetes-psa-review-agent` | kubernetes | CKS / Pod Security |
| `kubernetes-workload-identity-review-agent` | kubernetes | CKS / Workload Identity |
| `kubernetes-pod-spec-review-agent` | kubernetes | CKS / Workload Hardening |
| `external-secrets-operator-review-agent` | kubernetes | CKS / Secrets Management |
| `kubecost-chargeback-allocation-review-agent` | kubernetes | FinOps / Cost Attribution |
| `kyverno-policy-review-agent` | kyverno | CKS / Admission Control |
| `istio-service-mesh-review-agent` | istio | CISM / Service Mesh |
| `cilium-network-policy-review-agent` | cilium | CKS / eBPF Networking |
| `argocd-gitops-review-agent` | argocd | CAPA / GitOps |
| `argo-rollouts-progressive-delivery-review-agent` | argocd | CAPA / Progressive Delivery |
| `fluxcd-kustomization-helmrelease-review-agent` | fluxcd | CGOA / GitOps |
| `opentelemetry-collector-review-agent` | opentelemetry | COTA / Observability |
| `prometheus-alerting-cardinality-review-agent` | prometheus | PCA / Alerting |
| `falco-runtime-threat-rules-review-agent` | falco | CNPA / Runtime Security |
| `sigstore-cosign-supply-chain-review-agent` | sigstore | CNPA / Supply Chain |
| `cert-manager-issuer-trust-review-agent` | cert-manager | CKS / PKI Lifecycle |
| `backstage-scaffolder-template-review-agent` | backstage | CBA / Developer Platform |
| `aws-private-ca-issuer-review-agent` | aws | CKS / Cloud PKI |
| `azure-keyvault-certificate-issuer-review-agent` | azure | CKS / Cloud PKI |
| `oci-certificates-issuer-review-agent` | oci | CKS / Cloud PKI |
| `kubernetes-live-rbac-mutation-guard-agent` | kubernetes | Live-guard |
| `kubernetes-live-admission-policy-guard-agent` | kubernetes | Live-guard |
| `kubernetes-live-mesh-policy-guard-agent` | kubernetes | Live-guard |
| `kubernetes-live-network-policy-guard-agent` | kubernetes | Live-guard |
| `kubernetes-live-velero-restore-guard-agent` | kubernetes | Live-guard / DR |

Install by role: `npx vfa-export-agents --platform claude-code --role kubernetes-pki-engineer`

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* Kubernetes RBAC + 26 new CNCF/cloud-native agents + Golden Kubestronaut gap fill ([#8](https://github.com/Raishin/vanguard-frontier-agentic/issues/8)) (27e89dd)
feat: Kubernetes RBAC + 26 new CNCF/cloud-native agents + Golden Kubestronaut gap fill
* **kubernetes:** add kubernetes-rbac-review skill and agent (aaf6da1)
Introduces the first Kubernetes provider assets:
- `skills/kubernetes/kubernetes-rbac-review/` — SKILL.md, metadata.json,
  and three references (evidence path, workflow/output contract, official sources)
- `agents/kubernetes/kubernetes-rbac-review-agent/` — AGENT.md, metadata.json,
  and harnesses for Claude Code, Codex, Copilot, Cursor, Gemini, Kiro IDE, and Kiro CLI

Covers Roles, ClusterRoles, RoleBindings, ClusterRoleBindings, ServiceAccounts,
wildcard grant detection, namespace-vs-cluster scope enforcement, shared-SA risk,
automountServiceAccountToken defaults, and privilege escalation paths.

Catalog (agents.json, skills.json, skill-manifest.json) updated to 112 entries.
All `npm run validate` checks pass.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q

### 🐛 Bug Fixes

* address P1/P2 codex review comments on PR [#8](https://github.com/Raishin/vanguard-frontier-agentic/issues/8) (f431017)
P1 (scripts/export-marketplace-agents.mjs): --provider filter now matches
agents by their metadata `provider` field instead of id.startsWith(prefix).
Fixes silent omission of agents like external-secrets-operator-review-agent
and kubecost-chargeback-allocation-review-agent from role-based installs.

P2 (tests/validate-catalog.py): validate_guarded_live_kubernetes_agents now
asserts that codex.toml and AGENT.md exist for every expected agent ID before
attempting to read them. Prevents CI silently passing when a guarded agent
is renamed or deleted.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **ci:** correct actions/setup-python SHA and bump setup-node to v4.4.0 (79b2605)
setup-python SHA was incorrect (differed from v5.5.0 tag SHA at byte 15).
Corrected to 8d9ed9ac5c53483de85588cdf95a591a75ab9f55 (v5.5.0).
setup-node bumped from v4.1.0 to v4.4.0 (49933ea5288caeca8642d1e84afbd3f7d6820020).

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* correct path separators in project logo reference (7ed5bd9)
Changed Windows backslashes to forward slashes in logo image path for cross-platform compatibility. Updated comment to reflect that logo file is ready to display.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **kubernetes:** add missing escalation verbs and high-severity resources to RBAC skill (1d1762b)
Security review identified two functional correctness gaps:

1. escalate, bind, impersonate verbs were absent from the dangerous-defaults
   checklist. These are Kubernetes' three purpose-built privilege-escalation
   prevention verbs. A review that misses them will pass roles that allow
   unlimited privilege escalation regardless of all other restrictions.

2. pods/attach and nodes/proxy were missing from the high-severity resource list.
   pods/attach == pods/exec for interactive shell access. nodes/proxy grants
   kubelet API access on every node (effectively cluster-admin).

Both findings are now explicit in workflow-and-output.md (step 5 and 6) and
in official-sources.md grounded insights. No harness files changed.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* populate harness_variants for 19 legacy agents + add terraform-reviewer harnesses (5a2493c)
19 pre-existing agents (AWS/Azure/OCI live-guards, finops, terraform-reviewer)
had empty harness_variants in metadata.json, causing the vfa-export-agents CLI
to throw on any install that touched them via --role or --all.

Fixes applied:
- 18 agents: harness_variants populated by scanning their existing harnesses/ dir
- terraform-reviewer: created full harnesses/ dir with 7 platform files
  (claude-code, codex, copilot, cursor, gemini, kiro-ide, kiro-cli) derived
  from the agent's AGENT.md operating rules

Also:
- README.md: kubernetes agent dir count corrected (16 → 13)
- .claude/evals/vfa-cli-install.md + .log: formal CLI install eval suite,
  23/23 PASS; .gitignore updated to track .claude/evals/*.log
- .gitignore: unblock .claude/evals/*.log from *.log rule

Security finding documented in eval log (not fixed here): --repo path
traversal not validated against a safe root; tracked for separate issue.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **security:** apply 3 medium findings from PR security review (6bdaeb6)
M-1: Add validate_guarded_live_kubernetes_agents() to tests/validate-catalog.py
  Mirrors the existing AWS live-guard validator. Asserts each of the 5 K8s
  live-guard agents has sandbox_mode = "workspace-write" in codex.toml plus
  the contract terms (explicit platform-team sign-off, rollback, cluster
  context, current state) in both codex.toml and AGENT.md. Without this,
  a future PR could silently weaken a harness variant and CI would pass.

M-2: Validate --provider input in scripts/export-marketplace-agents.mjs
  Reject any --provider value that does not match /^[a-z0-9][a-z0-9-]*$/
  before it reaches the prefix filter or any error message. Also drop the
  reflected raw value from the no-match error message to close the log
  injection vector.

M-3: Warn against metadata service exfil in workload identity skill
  The GKE workload identity diagnostic example showed a metadata-server
  curl with no contextual warning, providing a ready-made template for
  cross-cloud metadata credential exfiltration (Capital One pattern).
  Added an explicit warning and a cross-link to cilium-network-policy-review
  for the 169.254.169.254/32 egress block pattern.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **security:** remediate S-01–S-05 findings from security audit (b854a2e)
S-01 (MEDIUM): copyFile now rejects symbolic links via lstatSync before
copying, preventing harness symlink exfiltration attacks.

S-02 (LOW): Role lookup uses Object.hasOwn() instead of bracket access
to prevent prototype pollution bypass of the unknown-role guard.

S-03 (LOW): main() emits a stderr warning when --repo resolves outside
the current working directory, alerting users to unexpected write targets.

S-04 (LOW): Pin actions/checkout, actions/setup-node, actions/setup-python
to commit SHAs in ci.yml and release.yml to eliminate major-version tag
hijack risk.

S-05 (LOW): Move semantic-release plugins to devDependencies in package.json,
generate package-lock.json, and switch release workflow from npm install
to npm ci for lockfile-verified installs.

npm run validate: 4/4 PASS (283 catalog, 44 AWS, 138 skills, 592 URLs)

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q

### 📚 Documentation

* add logo areas to all provider READMEs and root README placeholder (2830218)
Root README:
- Logo placeholder added inside the centered header div with clear comment
  showing exact path to drop in the file (assets/logos/vanguard-frontier-agentic.png)
  and img tag ready to uncomment at width=220

agents/aws (existing) — already correct, no change
agents/azure — created README with centered Azure logo (width=140)
agents/oci — created README with centered OCI logo (width=140)
agents/kubernetes — created README with centered ☸️ emoji placeholder + comment
agents/terraform — created README with centered 🟩 emoji placeholder + comment
agents/finops — created README with centered 💰 emoji placeholder + comment

skills/aws — fixed: wrapped bare <img> in <p align="center"> to match agents/aws format
skills/azure — fixed: converted markdown ![img]() to centered <p align="center"><img>
skills/oci — created README with centered OCI logo (width=140)
skills/kubernetes — created README with centered ☸️ emoji placeholder + comment
skills/terraform — created README with centered 🟩 emoji placeholder + comment
skills/finops — created README with centered 💰 emoji placeholder + comment

All provider READMEs follow the same format:
  <p align="center">
    <img src="../../assets/logos/cloud/<provider>/<file>" alt="..." width="140" />
  </p>

Providers without a logo file use an emoji placeholder with a comment pointing
to the expected logo path. Swap in the img tag when the logo file is created.

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **agents:** rewrite AGENTS.md as full navigation compass (b47f89f)
Previous file was 22 lines of minimal structure. New file is a dense
197-line index covering all 127 agents across 11 providers:

- Tier table: review / router+maestro / live-guard with sandbox_mode and
  hard-stop conditions for live-guard tier
- Per-provider sections (AWS/Azure/OCI) with category breakdowns and agent
  names grouped by sub-domain — points to provider-level AGENTS.md for
  operational depth without duplicating it
- Full Kubernetes section: all 9 agents with direct AGENT.md links and
  one-line load-when triggers
- Single-entry sections for Kyverno, ArgoCD, Istio, Cilium, OTEL,
  Terraform, FinOps — cross-linked to live-guard counterparts
- Live-guard cross-references: each CNCF domain review agent points to the
  kubernetes live-guard that handles its mutations
- Load sequence for multi-domain tasks (maestro → specialist → live-guard)
- Operational rules: validate, move, id convention, flush-left constraint

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **pki:** add PKI cert-manager agent selection guide and maestro routing (072efe9)
Documents when to use each PKI specialist agent (cert-manager K8s layer,
AWS Private CA, Azure Key Vault, OCI Certificates), concrete trigger
examples for each, multi-agent parallel scenarios, and the attack vector
all four agents jointly address (compromised workload identity → CA-signed
lateral movement cert).

Updates:
- docs/pki-cert-manager-agent-guide.md: full guide with per-agent trigger
  tables, multi-cloud parallel scenarios, and attack class section
- skills/kubernetes/kubernetes-maestro/references/workflow-and-output.md:
  adds `pki` domain row to routing table and taxonomy, PKI specialist
  reference section with cross-layer escalation note, and Example 5
  (cert-manager + workload identity parallel dispatch)
- skills/aws/aws-maestro/references/workflow-and-output.md: adds `pki`
  domain to taxonomy and aws-private-ca-issuer-review-agent to routing
  table under Security/IAM
- catalog/skill-manifest.json: refreshed after maestro reference changes
* **readme:** add emojis throughout to make install reference scannable and engaging (f09bbee)
- Get Started: numbered steps as emoji (1️⃣ 2️⃣ 3️⃣), map pointer emoji on link
- Skills section: cloud provider color emojis on table rows (🟧🟦🟥☸️🟩),
  live-guard subsection header energised, provider labels on each guard group
- Agents section: provider emojis on table, file emoji on each deliverable bullet
- Install Reference: all five sub-sections get emoji headers and in-table emojis
  - Decision tree: persona emojis per decision path
  - Argument table: ✅ required, ➕ optional, 🔍 standalone flags
  - Platform table: harness-specific emoji per platform (🤖⚡🐙🖱️♊🔮)
  - Role table: persona emoji per role (🔐🏗️🗄️💰🏛️🚀), cloud dots separator
  - Provider table: color-block emoji per cloud (🟧🟦🟥☸️🟩💰)
  - Scenarios table: intent emoji per row so you can scan without reading

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **readme:** reflect npm publication and update counts to 115 skills/agents (324d74a)
- Replace "not yet published" npm notice with live install instructions
  (npm install @raishin/vanguard-frontier-agentic) grounded in release.yml
- Update all skill/agent counts from 107 → 115 across tables, provider
  breakdown, folder tree, and ASCII summary
- Add Kubernetes provider row to Skills and Agents tables
- Add azure-live-entra-role-assignment-guard, oci-live-network-security-rule-guard,
  and kubernetes-live-rbac-mutation-guard to the live-guard listings
- Remove stale PERMISSIONS.md reference from agent structure description

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* **readme:** rewrite install section as unified Install Reference map (51aceaa)
Replace three fragmented sections (Get Started, CLI Commands, Role-Based Install)
with a single Install Reference that answers: what can I install, for which
platform, by what selection method, and with which arguments.

- Quick decision tree: role / agents / all / list
- Full argument reference table (all flags, values, required status, description)
- Platform reference table with harness name and install destination path
- Role reference table with agent counts, target persona, and coverage summary
- Provider reference table with cloud name and catalog count
- Common scenarios lookup table covering 10 install use cases in one place
- Get Started reduced to 3-line quick-start pointing at the Install Reference
- Top nav updated: CLI Commands link replaced with Install Reference

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q
* update READMEs for 12 new agents across 5 CNCF domains (bb0a00c)
- Root README: agent count 115→127, skill count 115→123, new CNCF domain
  rows in skills and agents tables (kyverno, argocd, istio, cilium,
  opentelemetry), expanded Kubernetes live-guard skills list (1→5),
  4 new K8s role rows in role reference, updated provider table with 5
  new providers, updated agent directory tree, added K8s install examples
- agents/kubernetes/README.md: rewritten to cover all 9 agents — maestro
  router, RBAC, workload identity, PSA, admission, sync, mesh, network
  live-guards; role-based install commands added
- agents/AGENTS.md: provider folder list updated with 5 new CNCF domains
- New: agents/kyverno/README.md — Kyverno policy review agent + live-guard
  cross-link
- New: agents/argocd/README.md — ArgoCD GitOps review agent + live-guard
  cross-link, AppProject blast-radius and sync impersonation notes
- New: agents/istio/README.md — Istio ambient mesh review, silent-bypass
  trap warning, PERMISSIVE mode note
- New: agents/cilium/README.md — Cilium network policy review, metadata
  service 169.254.169.254 egress warning, ClusterMesh trust note
- New: agents/opentelemetry/README.md — OTEL Collector config review,
  memory_limiter-first rule, no-exporter silent loss, credential note

https://claude.ai/code/session_01RvKUacSFzasvrUvzJDxr7Q

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
