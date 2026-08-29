# Judges Versus Scorers And LLM Instrument Error

The judge-versus-scorer distinction, the 17 built-in judges, judge error and validation, and regression-detection requirements.

- Judges are LLM-based evaluators; all 17 built-in judges produce Feedback with a value and rationale, carrying instrument error like any LLM. Scorers are the broader category including code-based (deterministic), vector-based, and LLM-based types. Do not conflate them.
- The ten single-turn judges are exactly: RelevanceToQuery, RetrievalRelevance, Safety, RetrievalGroundedness, Correctness, RetrievalSufficiency, Guidelines, ExpectationsGuidelines, ToolCallCorrectness, ToolCallEfficiency. No others.
- The seven multi-turn judges are exactly: ConversationCompleteness, UserFrustration, KnowledgeRetention, ConversationalGuidelines, ConversationalRoleAdherence, ConversationalSafety, ConversationalToolCallEfficiency. No others.
- The Correctness judge requires either `expected_facts` (a list) or `expected_response` in the dataset's expectations dict; without one, Correctness evaluation is not possible. A run comparison where one has expectations and one does not is confounded.
- The Correctness judge accepts an optional `model` parameter formatted `"<provider>:/<model-name>"` to select the judge LLM; two runs using different judge models measure different things and are not comparable for regression detection.
- Every LLM judge carries measurement error; a score movement is evidence of a possible change in the attribute the judge measures, not proof of a regression. A credible regression claim requires either: (a) the judge to be validated against human labels on a holdout set, demonstrating accuracy, or (b) independent corroboration from human feedback or a business metric change.
- Judge validation consists of running the judge on a holdout set of examples where human labels are known, then computing inter-rater agreement (e.g., accuracy, Fleiss' kappa) between judge scores and human labels. A judge with low agreement is not reliable for regression detection.
- `mlflow.genai.scorers.get_all_scorers()` returns every built-in scorer (judge and code-based combined); custom scorers are added via `mlflow.genai.Scorer` class or `mlflow.genai.scorer()` decorator.
