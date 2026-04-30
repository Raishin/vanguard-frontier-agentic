# Terraform provider diff analyzer grounding eval

[CAPABILITY EVAL: terraform-provider-diff-analyzer-grounding]
Task: Deepen the AzureRM, OCI, and AWS Terraform diff-analyzer skills with provider-specific grounded references and add an AWS variant.
Success Criteria:
- [ ] AzureRM analyzer references include Terraform core, AzureRM provider, and Azure service-domain grounding for Application Gateway and NSG review.
- [ ] OCI analyzer references include Terraform core, OCI provider, and OCI API/CLI surface grounding for route tables and load balancer rule collections.
- [ ] AWS analyzer skill exists with conservative support for listener rules, WAFv2 web ACLs, route tables, and security groups.
- [ ] Terraform README and validator reflect all provider analyzer variants.
Grader:
- `python3 tests/validate-terraform-agent-quality.py`
- `python3 tests/validate-catalog.py`
Expected Output:
- Terraform specialized skill validation PASS and catalog integrity PASS.

[REGRESSION EVAL: terraform-validation-stack]
Baseline: Terraform tier validators already passing before provider-grounding edits.
Tests:
- `python3 tests/validate-skill-manifest.py`: PASS
- `python3 tests/validate-links.py --offline`: PASS
- `npm run validate`: PASS
Result: PASS

## Verification

- `npm run manifest:write` -> PASS
- `python3 tests/validate-terraform-agent-quality.py` -> PASS
- `python3 tests/validate-catalog.py` -> PASS
- `python3 tests/validate-skill-manifest.py` -> PASS
- `python3 tests/validate-links.py --offline` -> PASS
- `npm run validate` -> PASS

## Status

READY
