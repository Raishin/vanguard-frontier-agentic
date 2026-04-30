# EVAL: azure-oci-live-guard-references

## Scope

12 live-guard skills (6 Azure + 6 OCI), each with 4 reference files:
- `permission-model.md`
- `preflight-commands.md`
- `rollback-playbook.md`
- `official-sources.md`

Total: 48 reference files. Eval validates technical accuracy against current Microsoft Learn and Oracle Cloud documentation, grounded via Context7 MCP.

## Capability Evals

### Azure RBAC action names

- [ ] AKS: `Microsoft.ContainerService/managedClusters/listClusterUserCredential/action` is the documented action for fetching user kubeconfig
- [ ] ARM Deployment Stacks: `Microsoft.Resources/deploymentStacks/{read,write,delete}` are valid resource provider actions
- [ ] App Service slot swap: `Microsoft.Web/sites/slotsswap/action` and `Microsoft.Web/sites/slots/slotsswap/action` are valid actions
- [ ] Cost: `Microsoft.Consumption/budgets/*` and `Microsoft.CostManagement/budgets/*` coexist as valid resource providers
- [ ] Key Vault: `Microsoft.KeyVault/vaults/keys/rotate/action` is the correct DataAction for rotation
- [ ] Key Vault: soft-delete retention window is 7–90 days (default 90)
- [ ] PIM: `Microsoft.Authorization/roleEligibilitySchedules/read` and `roleAssignmentScheduleRequests/write` are documented PIM API actions

### OCI IAM verb hierarchy and resource semantics

- [ ] OCI verb hierarchy is `inspect` ⊂ `read` ⊂ `use` ⊂ `manage`
- [ ] Dynamic groups are tenancy-scoped only (cannot be compartment-scoped)
- [ ] Autonomous Database termination is permanent; no recovery path
- [ ] Autonomous Database storage scale is one-way (grow only)
- [ ] Autonomous Database automatic-backup retention default
- [ ] Vault key deletion minimum window is 7 days
- [ ] Resource Manager: only one job running per stack at a time (platform-enforced)
- [ ] OKE: service principal requires `Allow service OKE` policy

### CLI command shape

- [ ] `az aks get-credentials --overwrite-existing` flag is supported
- [ ] `az deployment-stack group create --deny-settings-mode` accepts `denyDelete` and `denyWriteAndDelete`
- [ ] `oci db autonomous-database restore --timestamp` is the documented invocation
- [ ] `oci kms management key-version cancel-key-version-deletion` is the documented invocation
- [ ] `oci resource-manager job create-plan-job` and `create-apply-job` are documented subcommands

### Documentation URLs

- [ ] All `official-sources.md` URLs return 200 status (or are explicitly canonical Microsoft Learn / Oracle docs paths)

## Regression Evals

- [ ] `tests/validate-azure-oci-live-guards.sh` still passes 253/253
- [ ] `npm run validate` passes
- [ ] No reference file is now empty or contains only the stub pointer

## Graders

- **Code grader**: deterministic checks (file existence, regex matches, validate script exit code)
- **Model grader (Context7)**: query authoritative library docs and compare claimed action names, verbs, and windows
- **Human grader**: required for any FAIL flagged by Context7 grounding — manual verification before fix

## Success criteria

- pass@3 ≥ 0.90 on capability evals
- pass^3 = 1.00 on regression evals
- Zero unresolved CRITICAL findings (incorrect action name in production-ready RBAC role)
