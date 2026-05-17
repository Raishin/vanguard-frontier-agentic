---
name: influencer-disclosure-compliance-review
description: Use this skill when reviewing an influencer campaign audit pack — campaign brief, creator agreement excerpt, platform post descriptions or screenshot descriptions, and the disclosure format and placement specification — against FTC Endorsement Guides to identify undisclosed material connections, inadequate disclosure placement, and brand liability exposure. Trigger when a user provides a structured influencer campaign audit pack and asks whether disclosures meet FTC requirements, whether the brief contains problematic instructions, or whether the brand faces liability for creator conduct under 16 CFR Part 255.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Influencer Disclosure Compliance Review

## Purpose
This skill reviews a structured influencer campaign audit pack against the FTC Endorsement Guides (16 CFR Part 255, updated 2023) and FTC Act Section 5 to identify undisclosed material connections, inadequate disclosure placement, brand-instructed deceptive practices, and brand liability exposure. The FTC has consistently held that a "material connection" — payment, gifted product, free service, family or employment relationship, or any other material incentive — must be clearly and conspicuously disclosed so that consumers can weight the endorsement appropriately. A disclosure buried after the "more" fold, placed only in a hashtag crowd, or omitted entirely is a violation. The review assesses the audit pack as a static document: it does not generate new campaign content, draft creator instructions, or approve posts. This skill is scoped strictly to reviewing an existing audit pack; for ad-hoc content generation, use a different skill.

## Lean operating rules
- Treat any post description where a material connection (payment, gifted product, free service, brand affiliation) exists but no disclosure appears in the visible portion of the content — before the "more" or "see more" fold on Instagram, TikTok, or YouTube — as HIGH. FTC Endorsement Guides §255.5 requires clear-and-conspicuous disclosure visible without requiring additional user action.
- Treat gifted product or complimentary service received by the creator, regardless of whether cash payment was made, as a material connection requiring disclosure. Flag the absence of any gifted-product disclosure in the post description as HIGH.
- Treat a campaign brief that instructs creators to "only share positive experiences," suppress honest opinions, or omit negative aspects of the product as HIGH. Instructing suppression of honest opinion is a deceptive practice under 16 CFR §255.5 and creates brand liability.
- Treat disclosure language placed exclusively within a crowd of hashtags (e.g., `#ad` buried among 20 other hashtags) where it is not likely to stand out as HIGH — the FTC Endorsement Guides require disclosures to be clear and conspicuous, not hidden.
- Treat a creator agreement that contains no disclosure obligation clause, or whose clause does not specify placement requirements, as HIGH — the brand bears responsibility for ensuring adequate disclosure and must contractually enforce it.
- Treat verbal or audio-only disclosures in video content without simultaneous on-screen text disclosure as MEDIUM for platforms where superimposed text is technically feasible — the FTC's 2023 updated guides indicate disclosures should be simultaneous with the relevant content.
- Treat the use of platform-native "Paid Partnership" or "Branded Content" labels as a positive control but note it does not eliminate the obligation to make disclosures in the caption or verbally where the connection is not otherwise obvious.
- Flag any disclosure that uses ambiguous language — "collab," "sp," "partner," "ambassador" without context — without a plain-language equivalent as MEDIUM; the FTC guidance indicates that simple terms like "#ad" or "#sponsored" are preferred.
- Flag the absence of a documented disclosure review or approval step in the campaign workflow as MEDIUM — brands bear ongoing liability for non-compliant creator posts.
- Do not recommend suppressing, editing, or withholding any creator's honest opinion. Remediation recommendations must preserve the creator's right to share genuine views.
- Label every finding with evidence basis: brief provided, contract provided, post description provided, disclosure spec provided, or inference from missing document.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Material-connection disclosure assessment (payment, gifted product, free service, other incentives)
- Disclosure placement and conspicuousness assessment (pre-fold visibility, hashtag crowd, verbal vs. on-screen)
- Brief and contract review for problematic instructions (opinion suppression, mandatory positivity)
- Creator agreement disclosure-obligation clause assessment
- Platform-native label usage assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
