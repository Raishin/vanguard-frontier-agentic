# Grounding Source Evaluation Reference

This reference covers how to evaluate the quality, freshness, and retrieval
scope of grounding sources used by Agentforce
agents.

---

## What Grounding Means in Agentforce

Grounding connects an agent's reasoning to a curated, org-specific corpus
rather than relying solely on base model knowledge. Grounding sources may be:

- Salesforce Knowledge Articles
- Data Cloud
unified profiles and segments
- External retrieval actions (callouts to vector search endpoints)
- Flow-driven lookup results passed to the agent as context

Without grounding, the agent relies on training priors which may be outdated,
hallucinated, or org-agnostic.

---

## Evaluation Dimensions

### 1. Freshness

| Signal | Acceptable | At Risk |
|--------|-----------|---------|
| Knowledge Article last publish | < 90 days | > 180 days |
| Data Cloud ingestion lag | < 24 hours | > 72 hours |
| External retrieval index reindex | < 7 days | > 30 days |
| Article review owner assigned | Yes | No owner |

**Query to detect stale Knowledge Articles:**
```sql
SELECT Id, Title, LastPublishedDate, ArticleType, LastModifiedById
FROM KnowledgeArticleVersion
WHERE PublishStatus = 'Online'
  AND LastPublishedDate < LAST_N_DAYS:180
ORDER BY LastPublishedDate ASC
LIMIT 200
```

---

### 2. Coverage Gap Analysis

A coverage gap exists when user intents identified in conversation logs cannot
be answered from the grounded corpus. Steps to assess:

1. Export a sample of agent conversation transcripts (minimum 200 sessions).
2. Identify topics where the agent responded with uncertainty language:
   - "I don't have information on..."
   - "You may want to contact support..."
   - Generic deflection without substantive content.
3. Cross-reference those topics against the Knowledge Article corpus.
4. Coverage gap = intents with no matching articles / total identified intents.

**Acceptable threshold:** Coverage gap < 15% for primary topic areas.

---

### 3. Retrieval Scope Review

Retrieval scope determines which records or articles the agent can access when
grounding a response. Overly broad scope exposes sensitive content; overly
narrow scope creates gaps.

**Data Cloud segment scope checklist:**
- [ ] Segment definition excludes sensitive record types (e.g., Health Cloud
  clinical objects, Financial Services Cloud account details not relevant to
  the agent's role).
- [ ] Segment filter applies appropriate consent status (ContactPointConsent
  OptOutIndicator = false).
- [ ] Segment refresh schedule aligns with business change cadence.

**Knowledge Article scope checklist:**
- [ ] Data Categories attached to the agent's grounding source exclude
  internal-only articles.
- [ ] Article visibility is set to "All Users" or the appropriate customer-facing
  channel, not "Internal App".
- [ ] Draft and Archived articles are excluded from retrieval.

---

### 4. Retrieval Quality Scoring

Evaluate retrieval quality using semantic relevance testing:

**Test protocol:**
1. Prepare 20 representative user queries per primary topic.
2. Run each query against the grounding source in isolation (before agent
   reasoning applies).
3. Score each result set:
   - **Rank 1 hit relevant:** 2 points
   - **Relevant result in top 3:** 1 point
   - **No relevant result in top 5:** -1 point
4. Total score / (number of queries x 2) = retrieval precision ratio.

**Acceptable threshold:** Retrieval precision ratio > 0.70.

---

## Grounding Source Configuration Reference

### Knowledge Base Grounding (Native)
```
Agent Builder > Topics > [Topic] > Grounding Sources
  Source Type: Salesforce Knowledge
  Data Categories: [list]
  Article Type filter: [e.g., FAQ, How-To]
  Language filter: [match agent language]
```

### External Retrieval Action (Flow-Based)
```
// Example: Named Credential callout to vector search
Http.sendRequest(
  method: 'POST',
  endpoint: 'callout:VectorSearch_NC/v1/search',
  body: JSON.serialize(new Map<String, Object>{
    'query' => userQuery,
    'max_results' => 5,
    'score_threshold' => 0.72
  })
)
// Parse response, inject top-N chunks as context into agent prompt
```

### Data Cloud Grounding Checklist
- [ ] Unified Individual identified and mapped to org Contact/Lead.
- [ ] Segment used for grounding is published (not draft).
- [ ] Data stream activation status = Active.
- [ ] No PII fields (SSN, full DOB, financial account numbers) included in
  the retrieval payload returned to the agent.

---

## Red Flags

| Finding | Severity |
|---------|---------|
| No grounding source attached to a factual-answer topic | HIGH |
| Grounding source is a sandbox data stream (stale production copy) | HIGH |
| Single article covers 80%+ of topic retrievals (over-reliance) | MEDIUM |
| No human review of grounding source changes in > 6 months | MEDIUM |
| Retrieval returns full contact records including PII | CRITICAL |
| Score threshold for retrieval set to 0.0 (accepts any result) | HIGH |

---

## Freshness SLA Recommendations by Topic Domain

| Domain | Refresh SLA |
|--------|------------|
| Product pricing | Weekly |
| Legal/compliance policies | 30 days |
| How-to and support articles | 90 days |
| Data Cloud customer profiles | Daily |
| External knowledge (callout) | Per vendor SLA, max 7 days |
