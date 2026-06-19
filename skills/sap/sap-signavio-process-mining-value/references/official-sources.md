# Official sources — SAP Signavio Process Mining Value Review

Use this reference when grounding process discovery coverage assessment, conformance checking evaluation, bottleneck and rework analysis review, value realization tracking assessment, and S/4HANA process improvement linkage review.

**Evidence level**: documentation-based (SAP Help Portal, SAP Signavio Process Intelligence documentation). No live-system evidence is collected by this skill.

## SAP Signavio Process Intelligence overview

- What is SAP Signavio Process Intelligence
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/what-is-sap-signavio-process-intelligence
  source_owner: SAP SE
  topic_supported: SAP Signavio Process Intelligence capabilities overview, process mining scope, integration with SAP and non-SAP source systems, difference between Process Intelligence and Process Manager
  why_needed: Primary reference for understanding the Signavio Process Intelligence platform model — defines capabilities, scope boundaries, and integration options used to classify capability coverage findings
  evidence_level: primary
  last_verified: 2026-06-19

## Data source connection and event log

- Connecting Data Sources
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/connecting-data-sources
  source_owner: SAP SE
  topic_supported: Event log extraction from SAP S/4HANA, SAP ECC, Ariba, SuccessFactors, and non-SAP systems; case ID and activity attribute mapping; event log schema requirements; SAP extraction content (pre-built extractors for P2P, O2C, R2R)
  why_needed: Authoritative reference for evaluating event log coverage quality — defines the extraction model and pre-built content used to classify event log coverage gaps and attribute completeness findings
  evidence_level: primary
  last_verified: 2026-06-19

## Investigation and process discovery

- Setting Up Investigations
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/setting-up-investigations
  source_owner: SAP SE
  topic_supported: Investigation configuration in Signavio Process Intelligence, process variant explorer setup, filter and drill-down configuration, KPI definition within investigations, process map generation
  why_needed: Defines the investigation design model — required to assess process discovery coverage quality, variant explorer configuration, and whether the investigation setup enables meaningful root cause analysis
  evidence_level: primary
  last_verified: 2026-06-19

## Process variant analysis

- Process Variant Explorer
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/process-variant-explorer
  source_owner: SAP SE
  topic_supported: Process variant identification, variant frequency and case coverage statistics, variant comparison, outlier variant handling, process map abstraction levels
  why_needed: Primary reference for evaluating process discovery quality — defines the variant coverage model and abstraction options used to classify discovery completeness findings
  evidence_level: primary
  last_verified: 2026-06-19

## Conformance checking

- Conformance Checking
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/conformance-checking
  source_owner: SAP SE
  topic_supported: Conformance checking configuration in Signavio Process Intelligence, reference model definition, deviation classification (missing activity, additional activity, wrong order), conformance score calculation, deviation root cause investigation
  why_needed: Authoritative reference for conformance checking review — defines the target model requirement, deviation taxonomy, and scoring model used to classify conformance analysis gap findings
  evidence_level: primary
  last_verified: 2026-06-19

## KPIs and performance measurement

- Key Performance Indicators
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/key-performance-indicators
  source_owner: SAP SE
  topic_supported: KPI definition in Signavio Process Intelligence, cycle time KPI, throughput time KPI, rework rate KPI, exception rate KPI, KPI threshold configuration and alerting
  why_needed: Primary reference for KPI framework review — defines the KPI definition model, available metric types, and threshold configuration used to classify KPI coverage and threshold gap findings
  evidence_level: primary
  last_verified: 2026-06-19

## Benchmarking

- Benchmarking
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/benchmarking
  source_owner: SAP SE
  topic_supported: SAP Business Process Intelligence benchmarking data integration, industry benchmark comparison for P2P, O2C, and other end-to-end processes, benchmark metric selection and interpretation
  why_needed: Defines the benchmarking capability in Signavio Process Intelligence — required to assess whether benchmark comparison is used to contextualize bottleneck and cycle time findings against industry standards
  evidence_level: primary
  last_verified: 2026-06-19

## Value realization

- Value Realization
  https://help.sap.com/docs/signavio/sap-signavio-process-intelligence/value-realization
  source_owner: SAP SE
  topic_supported: Value realization tracking in Signavio Process Intelligence, improvement initiative registration, pre- and post-implementation KPI comparison, business case outcome measurement, value dashboard
  why_needed: Primary reference for value realization tracking review — defines the improvement initiative tracking model and outcome measurement approach used to classify value realization maturity findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and Signavio Process Intelligence documentation describe the designed platform capabilities, investigation configuration model, conformance checking framework, KPI definition options, and value realization tracking approach. They do not prove what processes are currently mined in the user's Signavio tenant, what conformance scores exist for specific processes, what bottlenecks have been identified, or whether value realization tracking is active. Users must supply mining configuration descriptions, conformance check summaries, bottleneck analysis outputs, value tracking dashboard exports, or written descriptions of their Signavio process mining setup for concrete assessment.
