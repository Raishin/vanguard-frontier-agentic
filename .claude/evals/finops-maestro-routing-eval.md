[CAPABILITY EVAL: maestro-routing-correctness]
Date: 2026-05-13

Grader: tests/validate-maestro-routing.py (Python, provider-agnostic)
Grader output: PASS (exit code 0)
Finops section: "-- [finops] 9 scenarios passed"

Fixture results:
- 001: route=finops-ai-economist-agent mode=single → PASS
- 002: route=finops-kubernetes-rightsizer-agent mode=single → PASS
- 003: route=finops-cloud-price-advisor-agent mode=single → PASS
- 004: route=[finops-ai-economist-agent, finops-kubernetes-rightsizer-agent] mode=parallel (2) → PASS
- 005: route=[finops-cloud-price-advisor-agent, finops-kubernetes-rightsizer-agent] mode=parallel (2) → PASS
- 006: route=finops-kubernetes-rightsizer-agent mode=single → PASS
- 007: route=[] mode=unclassified → PASS
- 008: route=finops-cloud-price-advisor-agent mode=single → PASS
- 009: route=[] mode=unclassified → PASS

Adversarial checks:
- 008-secrets-bait AWS key regex: CLEAN (XKIA prefix, not AKIA; wrapped in <FAKE> markers; _task_has_unmarked_credential() returns False)
- 006-adversarial-instruction-injection: k8s scores 2, other domains 0 → single k8s; live_guard_intent does not match → PASS
- 007-adversarial-persona-replacement: all domains score 0 → unclassified → PASS
- 009-ambiguous: all domains score 0 → unclassified → PASS

Manual keyword scoring (word-boundary, case-insensitive):
- 001: ai-economist=8, k8s=1, price=0 → single ai-economist ✓
- 002: ai-economist=0, k8s=7, price=0 → single k8s ✓
- 003: ai-economist=0, k8s=0, price=5 → single price ✓
- 004: ai-economist=6, k8s=6, price=0; 6 >= 6*0.6 → parallel (2) ✓
- 005: ai-economist=0, k8s=6, price=4; 4 >= 6*0.6=3.6 → parallel (2) ✓
- 006: ai-economist=0, k8s=2, price=0 → single k8s ✓
- 007: all=0 → unclassified ✓
- 008: ai-economist=0, k8s=0, price=1 → single price ✓
- 009: all=0 → unclassified ✓

Taxonomy check:
- Domains: ai-economist, kubernetes-rightsizer, cloud-price-advisor — all 3 PRESENT
- parallel_threshold: 0.6 — PRESENT
- live_guards: [] (empty — correct for v1; no mutating finops specialists)
- ai-economist keywords: 24 entries
- kubernetes-rightsizer keywords: 25 entries
- cloud-price-advisor keywords: 19 entries

Summary: 9/9 fixtures correct
Status: PASS
