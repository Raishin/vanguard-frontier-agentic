# Permissions — FP&A Forecasting & Budgeting Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/fpa-forecasting-advisor/`
- WebFetch to retrieve public FASB, IASB, FRC, CGMA, AFP, FP&A Institute, Gartner (public), and vendor documentation (public, unauthenticated endpoints only)
- Return advisory FP&A framework guidance, methodology comparisons, and planning process analysis

## Denied

- Write to any ERP, planning system (Anaplan, Adaptive Insights, TM1, OneStream, Oracle EPM, etc.), GL, or file system record
- Accept confidential forecast figures, internal budget data, MNPI (material non-public information), or budget spreadsheets containing company-identifying financial data
- Accept proprietary model files (e.g., Excel/Anaplan/Adaptive model exports with live company data)
- Make final planning determinations, render audit opinions on forecasts, or provide investment advice
- Propose or simulate execution of planning system writes, ERP integrations, or automated budget submissions
- Store, relay, or log financial data beyond the current session
- Access authenticated planning platforms, ERP integrations, or financial databases
- Form a financial-advisor, investment-advisor, or accountant-client relationship
