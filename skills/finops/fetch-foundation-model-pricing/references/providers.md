# Provider Pricing URLs

Use these URLs with WebFetch to retrieve live pricing. All pages are public and require no authentication.

## Anthropic

| Resource | URL |
|---|---|
| Pricing page | https://docs.anthropic.com/en/docs/about-claude/pricing |
| Model overview | https://docs.anthropic.com/en/docs/about-claude/models/overview |

Pricing dimensions available: input tokens ($/1M), output tokens ($/1M), prompt caching write ($/1M), prompt caching read ($/1M), batch input ($/1M), batch output ($/1M).

Models published as of last verification: Claude Opus 4, Claude Sonnet 4.5, Claude Haiku 3.5, and prior generation models.

## OpenAI

| Resource | URL |
|---|---|
| Pricing page | https://platform.openai.com/docs/pricing |
| Models list | https://platform.openai.com/docs/models |

Pricing dimensions available: input tokens ($/1M), cached input tokens ($/1M), output tokens ($/1M). Batch API discounts published separately on the same page.

Models published: GPT-4o, GPT-4o mini, o1, o3, o4-mini, text-embedding-3-large/small, DALL-E (per image).

## AWS Bedrock

| Resource | URL |
|---|---|
| Pricing page | https://aws.amazon.com/bedrock/pricing/ |
| On-demand pricing table | https://aws.amazon.com/bedrock/pricing/#On-demand |
| Batch inference | https://aws.amazon.com/bedrock/pricing/#Batch_inference |
| Provisioned throughput | https://aws.amazon.com/bedrock/pricing/#Provisioned_throughput |

Pricing dimensions available: on-demand input/output tokens ($/1000 tokens or $/1M tokens depending on model), batch input/output tokens, provisioned throughput model units ($/hour).

Models published: Anthropic Claude family, Amazon Titan, Amazon Nova, Meta Llama, Mistral, Cohere, AI21 Jurassic, Stability AI.

Note: Bedrock token prices are sometimes published per 1,000 tokens rather than per 1M. Normalize to $/1M for comparison by multiplying by 1,000.

## Azure OpenAI Service

| Resource | URL |
|---|---|
| Pricing page | https://azure.microsoft.com/en-us/pricing/details/cognitive-services/openai-service/ |
| Model catalog | https://learn.microsoft.com/en-us/azure/ai-services/openai/concepts/models |

Pricing dimensions available: pay-as-you-go input/output tokens ($/1K tokens), provisioned throughput units (PTU, $/hour/PTU). Prices vary by Azure region.

Models published: GPT-4o, GPT-4o mini, o1, o3, o4-mini, text-embedding, DALL-E 3.

Note: Azure OpenAI prices are often published per 1,000 tokens. Normalize to $/1M by multiplying by 1,000.

## Google Vertex AI

| Resource | URL |
|---|---|
| Generative AI pricing | https://cloud.google.com/vertex-ai/generative-ai/pricing |
| Gemini model pricing | https://cloud.google.com/vertex-ai/generative-ai/pricing#gemini-models |
| Embedding pricing | https://cloud.google.com/vertex-ai/generative-ai/pricing#embedding-models |

Pricing dimensions available: input tokens ($/1M), output tokens ($/1M), context caching ($/1M per hour storage + discounted input tokens), grounding ($/1K queries), image input (per image or per 1K images for multimodal).

Models published: Gemini 2.5 Pro, Gemini 2.5 Flash, Gemini 1.5 Pro/Flash, text-embedding-004, Imagen 3.

## OCI Generative AI

| Resource | URL |
|---|---|
| Service overview | https://www.oracle.com/cloud/ai/generative-ai/ |
| Pricing page | https://www.oracle.com/cloud/ai/generative-ai/pricing/ |

Pricing dimensions available: on-demand token pricing (input and output per 1M tokens), dedicated AI cluster pricing (unit/hour).

Models published: Cohere Command R/R+, Meta Llama 3 family.

Note: OCI Generative AI pricing may not be listed on the main service overview page. If the pricing page URL returns no results, also try https://www.oracle.com/cloud/price-list/ and search for "Generative AI".

## Fetch strategy

When using WebFetch against a pricing page:

1. Fetch the canonical pricing URL listed above.
2. Record the response timestamp in UTC (ISO 8601, e.g., `2026-05-13T14:32Z`).
3. Extract the relevant model row(s) for the requested model name or family.
4. If the page uses JavaScript to render a pricing table, it may return partial or placeholder content — in that case, label the result `documentation-based` and note the URL fetched.
5. If the fetch fails entirely, fall back to the most recent documentation-based price known and label it `documentation-based` with the documentation URL.
