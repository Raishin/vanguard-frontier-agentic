# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide the influencer campaign audit pack as a structured document. The pack should include:
- **Campaign brief**: objectives, key messages, deliverable specifications, any instructions to creators about tone or content scope
- **Creator agreement excerpt**: compensation terms (cash, gifted product, affiliate commission, free service), disclosure obligation clause, content approval process
- **Post descriptions**: written descriptions of the posts as published or planned, or a text description of screenshot content — including caption text, hashtags used, and where in the caption any disclosure language appears relative to the "more" fold
- **Disclosure format and placement specification**: the brand's stated requirement for how creators must disclose (e.g., "#ad in first line," "verbal disclosure in first 30 seconds," "Instagram Paid Partnership label required")

Note which documents are absent. If the brief is missing, note that brief-level findings cannot be assessed. If post descriptions are missing, note that placement findings are inference only.

### Step 2 — Material connection identification

Before assessing disclosures, identify all material connections present in the campaign:
- **Cash payment**: flat fee, CPM, performance-based commission, affiliate fee
- **Gifted product**: product provided free of charge regardless of whether additional payment was made — the FTC is explicit that product gifts are material connections
- **Complimentary service**: free access, free subscription, free experience
- **Brand affiliation**: creator is an employee, brand ambassador, family member of a brand employee, or investor in the brand
- **Other incentives**: contest entry, travel, accommodation, ticket access

```text
# Example: material connections present in this campaign
- Cash payment: $[AMOUNT_REDACTED] flat fee per post
- Gifted product: one unit of [PRODUCT] provided prior to post
- No affiliate commission structure found in contract excerpt
```

Any combination of the above constitutes a material connection requiring disclosure under 16 CFR §255.5. Document every connection found.

### Step 3 — Brief review for problematic instructions

Review the campaign brief for instructions that could themselves be deceptive practices or brand liability triggers:
- **Opinion suppression**: Instructions to "only share positive experiences," "avoid mentioning [negative aspect]," "only post if you love it," or equivalent. Under 16 CFR §255.5 and FTC Act Section 5, directing a creator to suppress honest opinion is a deceptive practice attributable to the brand.
- **Mandatory positivity**: Instructions requiring the creator to express enthusiasm, use superlatives, or frame the product in exclusively positive terms.
- **Approval-gate bias**: An approval process where the brand reviews content before publication and withholds approval for content that includes balanced or negative views — functionally equivalent to opinion suppression.
- **False authenticity framing**: Instructions to present sponsored content as organic discovery (e.g., "write as if you found this yourself").

```text
# HIGH — brief instructs opinion suppression
Brief language: "Please only share your experience if it is positive. If you have concerns,
reach out to us directly rather than including them in your post."

# COMPLIANT — brief preserves honest opinion
Brief language: "Share your genuine experience with the product. If you have concerns,
you are welcome to include them. We ask only that you disclose the partnership."
```

### Step 4 — Disclosure placement and conspicuousness assessment

For each post description, assess whether disclosure language is clear and conspicuous:

**Pre-fold visibility rule**: On platforms with truncated captions (Instagram, TikTok, Facebook), disclosure language must appear before the "more" or "see more" fold — i.e., in the first approximately 125 characters visible without user interaction. A disclosure that appears after the fold is not clear and conspicuous regardless of its content.

```text
# HIGH — disclosure after the fold
Caption (visible before "more"):
"I've been obsessed with this skincare routine lately — here's what I've been using
every morning to get glowing skin. Products linked below! 🌟 [120 chars so far]"
[...more...]
"#ad #sponsored #gifted"

# COMPLIANT — disclosure in first line, before fold
Caption:
"AD | [Brand] gifted me this skincare set and I've been loving it..."
```

**Hashtag crowd burial**: A disclosure hashtag (`#ad`, `#sponsored`) buried within a group of 15 or more other hashtags at the end of a caption is not clear and conspicuous. Assess whether the disclosure hashtag stands out.

**Video disclosures**: For video content, assess whether disclosure is:
- Verbal: stated clearly and early (within first 30 seconds for videos over 2 minutes)
- On-screen text: present simultaneously with the verbal mention or the first reference to the product
- Not reliant solely on a description-box disclosure that viewers may not see

**Platform-native labels**: Note whether Instagram's "Paid Partnership" label, TikTok's "Branded Content" toggle, or YouTube's paid promotion disclosure checkbox were used. These are positive controls but do not eliminate caption or verbal disclosure obligations where the connection might otherwise not be obvious.

### Step 5 — Creator agreement disclosure-obligation clause assessment

Review the creator agreement excerpt for:
- **Presence of a disclosure clause**: Does the agreement explicitly require the creator to disclose the material connection in every post?
- **Placement specificity**: Does the clause specify where in the post the disclosure must appear (e.g., "in the first line of the caption before any truncation")?
- **Platform coverage**: Does the clause cover all platforms on which the creator will post, including Stories, Reels, TikTok, YouTube Shorts, and any cross-posting?
- **Enforcement mechanism**: Does the agreement give the brand a right to request correction of a non-compliant post?

A creator agreement with no disclosure clause, or with a disclosure clause that does not specify placement, leaves the brand exposed — the FTC holds brands responsible for ensuring disclosures are made even when individual creators are nominally independent.

```text
# HIGH — no disclosure clause
Creator agreement excerpt: [No disclosure obligation language found in provided excerpt]

# COMPLIANT — specific placement requirement
Creator agreement clause: "Creator shall include '#ad' or '#sponsored' as the first
hashtag or in the first line of the caption on each post, before any caption truncation."
```

### Step 6 — Disclosure format adequacy assessment

Assess whether the disclosure language specified in the brief or used in post descriptions meets FTC clarity standards:
- **Acceptable terms**: `#ad`, `#sponsored`, "Advertisement," "Paid partnership with [Brand]," "I received this product for free from [Brand]"
- **Ambiguous terms** (flag as MEDIUM): `#collab`, `#sp`, `#partner`, `#ambassador` without further context — the FTC guidance notes these may not be universally understood
- **Insufficient terms**: `#gifted` alone may not be sufficient if it does not convey the commercial nature of the relationship; `#affiliate` is more specific but may still require context

### Step 7 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<brief provided | contract provided | post descriptions provided | disclosure spec provided | inference>

## Material connections identified
<list all material connections found in the audit pack>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security and scope notes

- This is a static review of a structured influencer campaign audit pack. The skill does not generate new post content, draft creator instructions, or approve posts for publication.
- Never recommend that creators suppress honest opinions, omit negative experiences, or present sponsored content as organic discovery — these are themselves FTC violations and increase brand liability.
- Brand liability under FTC Act Section 5 extends to deceptive acts by creators the brand directed or had reason to know about. A finding of brief-level opinion suppression instructions is a brand liability issue, not only a creator issue.
- The FTC Endorsement Guides were substantially updated in 2023 — verify that any prior campaign documentation was produced with awareness of the updated requirements, particularly regarding disclosure placement and the treatment of gifted product.
- When evidence is partial (e.g., no post descriptions provided), scope placement findings to inference and state assumptions explicitly.
- A serious finding here (e.g., systematic non-disclosure across a campaign) may warrant notification to legal counsel before the campaign continues or expands.
