# MLflow 3 Registry Defaults And Unity Catalog

The default registry URI on new accounts, model-namespace format, and the legacy Workspace Model Registry status.

- MLflow 3 defaults to `databricks-uc` (Unity Catalog) as the registry URI on new Databricks accounts since April 2024; the legacy Workspace Model Registry is disabled and is accessible only via explicit `mlflow.set_registry_uri("databricks")` on older accounts.
- Model addresses in Unity Catalog follow a three-level namespace: `<catalog>.<schema>.<model>`, not the two-level `<schema>.<model>` of the legacy registry.
- Model URIs in MLflow 3 changed from `runs:/<run_id>/<artifact_path>` to `models:/<model_id>`, and any model registered to MLflow 3's default registry uses the new URI format.
- `MlflowClient.set_registered_model_alias()`, `MlflowClient.get_model_version_by_alias()`, and `mlflow.search_registered_models()` are the primary APIs for alias-based promotion; legacy stage-based promotion is not available in Unity Catalog registries.
- Promotion in Unity Catalog uses custom aliases (e.g., Champion, Challenger, Staging, Production) instead of the fixed stages (None, Archived, Staging, Production) from the legacy registry.
- `mlflow.pyfunc.load_model()` loads a model by alias: `mlflow.pyfunc.load_model('models:/<catalog>.<schema>.<model>@<alias>')`, where the alias resolves to the current version bearing it.
- Cross-registry promotion (model registered to legacy registry, served by a Unity Catalog endpoint) is a configuration mismatch and is not supported.
- Model registration to Unity Catalog requires the caller to have `USE_CATALOG` on the catalog and `USE_SCHEMA` and `CREATE_MODEL` on the schema.
