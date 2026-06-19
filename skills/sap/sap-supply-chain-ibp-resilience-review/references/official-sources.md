# Official sources — SAP Supply Chain IBP Resilience Review

Use this reference when grounding demand planning assessment, supply planning design review, S&OP governance evaluation, inventory optimization analysis, forecast accuracy and bias monitoring review, control tower alert design assessment, and scenario planning coverage analysis.

**Evidence level**: documentation-based (SAP Help Portal, SAP Integrated Business Planning documentation). No live-system evidence is collected by this skill.

## SAP IBP — Demand Planning and Statistical Forecasting

- SAP Integrated Business Planning — Demand Planning
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/demand-planning
  source_owner: SAP SE
  topic_supported: Statistical forecasting algorithm configuration (ARIMA, exponential smoothing, Croston for intermittent demand), causal factor modeling, promotions planning, lifecycle planning (new product introduction, end-of-life), consensus demand planning process, forecast override governance
  why_needed: Primary reference for assessing demand planning configuration quality and forecast model selection — defines the IBP statistical forecasting model and consensus planning framework used to classify demand planning findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Supply Planning and Capacity Constraint Modeling

- SAP Integrated Business Planning — Supply Planning
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/supply-planning
  source_owner: SAP SE
  topic_supported: Supply planning heuristic and optimizer configuration, capacity constraint modeling (production, transport, storage), multi-echelon replenishment logic, network design alignment, supply plan exception handling for capacity violations and supply shortfalls, demand-driven MRP (DDMRP) integration
  why_needed: Authoritative reference for assessing supply planning design and capacity modeling completeness — defines the IBP supply planning optimization model and exception handling framework used to classify supply planning findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Sales and Operations Planning (S&OP)

- SAP Integrated Business Planning for Sales and Operations
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/sales-and-operations-planning
  source_owner: SAP SE
  topic_supported: S&OP cycle design, plan version management, collaborative review configuration, S&OP meeting cadence governance, escalation and decision authority configuration, consensus plan definition and version control
  why_needed: Primary reference for assessing S&OP governance design — defines the IBP for S&OP process model, version management framework, and collaborative planning configuration used to classify S&OP governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Inventory Optimization

- SAP Integrated Business Planning — Inventory Optimization
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/inventory-optimization
  source_owner: SAP SE
  topic_supported: Safety stock calculation methods (statistical, demand-driven), service level target configuration by product and location, inventory stratification (ABC/XYZ), slow-moving and excess inventory identification, multi-echelon inventory optimization, inventory target review cadence
  why_needed: Defines the IBP inventory optimization model — required to classify safety stock calculation gaps, service level target misconfiguration, and inventory stratification staleness findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Control Tower and Exception Management

- SAP Integrated Business Planning — Supply Chain Control Tower
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/supply-chain-control-tower
  source_owner: SAP SE
  topic_supported: Control tower alert rule configuration, alert threshold calibration, alert ownership and routing, exception resolution workflow design, alert hierarchy and prioritization, integration with SAP S/4HANA and IBP planning modules for alert triggering
  why_needed: Primary reference for assessing control tower alert design completeness and exception management governance — defines the IBP control tower alert model used to classify alert coverage gaps and alert fatigue findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Scenario Planning and What-If Analysis

- SAP Integrated Business Planning — Scenario Planning
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/scenario-planning
  source_owner: SAP SE
  topic_supported: What-if scenario configuration, scenario comparison and financial impact quantification, scenario version management, integration of scenario outputs into S&OP decision process, disruption scenario design (demand shock, supply disruption, capacity constraint)
  why_needed: Defines the IBP scenario planning model — required to classify scenario planning coverage gaps, missing disruption scenarios, and whether scenario planning is integrated with the S&OP governance process
  evidence_level: primary
  last_verified: 2026-06-19

## SAP IBP — Forecast Accuracy and Error Metrics

- SAP Integrated Business Planning — Forecast Accuracy and Error Metrics
  https://help.sap.com/docs/SAP_IBP/sap-integrated-business-planning/forecast-accuracy-and-error-metrics
  source_owner: SAP SE
  topic_supported: Forecast error metric configuration (MAPE, WMAPE, bias, MAD, RMSE), statistical baseline error benchmarking, forecast value-add analysis, outlier detection and cleansing configuration, error tracking by planning level and time horizon in SAP IBP
  why_needed: Defines the forecast accuracy monitoring framework — required to classify forecast bias accumulation findings, missing error metric coverage, and insufficient forecast review cadence governance
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP IBP documentation describes the designed planning model and configuration options for demand planning, supply planning, S&OP, inventory optimization, control tower, and scenario planning. It does not prove what forecasting algorithm is active in the user's IBP tenant, what safety stock parameters are configured, whether control tower alert thresholds are calibrated, or whether single-source supply dependencies have been modeled in scenario planning. Users must supply planning parameter exports, S&OP process documentation, forecast accuracy metric reports, inventory policy summaries, control tower alert rule descriptions, or written descriptions of their IBP planning landscape for concrete assessment.
