---
name: programmatic-supply-chain-integrity-review
description: Use this skill when reviewing ads.txt, app-ads.txt, and sellers.json files for a publisher or advertiser's programmatic supply chain to detect unauthorized resellers, domain-spoofing exposure, and SupplyChain Object gaps. Trigger when a user provides an ads.txt file, an app-ads.txt file, a sellers.json endpoint response, or asks whether their programmatic supply chain has unauthorized intermediaries, IVT exposure, domain-spoofing risk, or whether their supply chain declaration satisfies IAB Tech Lab, MRC, or DSP procurement requirements.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: finops
  lifecycle: experimental
---

# Programmatic Supply Chain Integrity Review

## Purpose
This skill reviews ads.txt, app-ads.txt, and sellers.json declarations for a publisher's or advertiser's programmatic supply chain to detect unauthorized resellers, domain-spoofing exposure, SupplyChain Object gaps, and IVT-exposure vectors. Ads.txt (IAB Tech Lab v1.1) and app-ads.txt are the publisher's machine-readable authorization of which exchanges and resellers may sell their inventory; sellers.json (IAB Tech Lab v1.0) is the exchange's machine-readable disclosure of which sellers it represents. When these files are inconsistent — an ads.txt RESELLER entry that no exchange discloses in sellers.json, a DIRECT entry that resolves as `is_confidential:1`, or a whitelisted domain whose ads.txt is absent — the supply chain is opaque to buyers, exposing them to unauthorized intermediary fees and exposing publishers to domain spoofing. The SupplyChain Object (OpenRTB extension) enables bid-time audit of the complete reseller path; gaps in the declared path are treated as invalid traffic by MRC-compliant measurement vendors and many DSP procurement teams. The review works from the raw text of the artifact files pasted as input and produces severity-labelled findings with remediation.

## Lean operating rules
- Treat ads.txt RESELLER entries for exchange accounts that do not appear in any sellers.json file for that exchange as HIGH — these are undisclosed intermediaries whose presence in the resale chain cannot be verified by buyers, constituting unauthorized supply path opacity under IAB Tech Lab ads.txt 1.1.
- Treat a whitelisted publisher domain whose ads.txt file is entirely absent as HIGH — the absence means buyers cannot verify any authorized seller relationship; the domain is categorically IVT-exposed per MRC Invalid Traffic Detection guidelines and most DSP whitelisting criteria.
- Treat a DIRECT entry in ads.txt where the corresponding seller account in sellers.json carries `is_confidential:1` as HIGH — a DIRECT relationship by definition requires transparent publisher identity; confidential resolution contradicts the DIRECT classification and is a domain-spoofing risk vector.
- Treat ads.txt entries that reference exchange account IDs not present in the exchange's sellers.json at all (orphaned account IDs) as HIGH — the account cannot be verified as a legitimate seller, which is a signal of domain spoofing or stale declarations.
- Treat a `seller_type: INTERMEDIARY` entry in sellers.json that has no corresponding ads.txt RESELLER entry on the publisher domain as MEDIUM — the intermediary is declared by the exchange but not authorized by the publisher, creating a supply path discrepancy.
- Treat SupplyChain Object declarations with incomplete node chains (missing `asi`, `sid`, or `rid` fields in intermediate nodes) as MEDIUM — incomplete chains reduce bid-time auditability and may cause DSP procurement filters to reject the bid.
- Flag MEDIUM when the ads.txt file has not been updated within twelve months and active exchange relationships are known to have changed — stale declarations expose revenue to unauthorized resellers who retain old account relationships.
- Flag the absence of app-ads.txt for a mobile app publisher as MEDIUM when the publisher's ads.txt covers only web inventory — app inventory without app-ads.txt is unprotected by IAB Tech Lab supply-chain controls.
- Do not recommend removing a RESELLER entry without first confirming whether it represents a legitimate revenue path that can be replaced with a DIRECT relationship or a disclosed intermediary.
- Label every finding with evidence basis: ads.txt provided, sellers.json provided, documentation-based, or inference from absent file.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- RESELLER-to-sellers.json consistency assessment (unauthorized intermediaries)
- DIRECT-entry confidentiality conflict assessment (domain-spoofing risk)
- Orphaned account ID assessment (account IDs in ads.txt not in sellers.json)
- Absent ads.txt / app-ads.txt assessment for whitelisted domains
- SupplyChain Object completeness assessment
- Stale declaration assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
