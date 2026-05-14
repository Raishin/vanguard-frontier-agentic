# FOCUS Spec Normalizer

A FinOps skill that normalizes vendor-specific billing rows into FOCUS v1.2 columns from user-pasted CSV or JSON input. Operates on caller-supplied data only; refuses to invent values not present in the input.

## Purpose

Map billing rows from AWS Cost and Usage Report (CUR), Azure Cost Management export, GCP Billing Export, or OCI billing into the FinOps Open Cost and Usage Specification (FOCUS) v1.2 column schema. Each output column is annotated with a provenance label (mapped, derived, or null) and gaps are documented with resolution notes.

## Allowed tools

`Read` `Grep` `Glob`

## Usage

**Single row:** Paste one billing row as CSV (with header) or JSON object. The skill detects the vendor from the column names, applies the appropriate mapping, and returns a FOCUS v1.2 JSON object with per-column provenance labels and a gap summary.

**Multi-row batch:** Paste multiple rows (CSV table or JSON array). The skill maps each row and returns an array of FOCUS objects plus an aggregate gap summary covering all rows.

## Trust posture

Read-only. No cloud credentials, billing account IDs, tenant IDs, or tenant-specific data accepted. All input is user-provided; no live cloud API connections are made. The skill will not invent, default, or infer values for FOCUS columns that are absent from the input — absent columns are set to `null` with a stated reason.

Targets FOCUS v1.2. The FOCUS version is declared in every output.

See [SKILL.md](SKILL.md) for the full mapping behavior, vendor detection logic, and response shape.
