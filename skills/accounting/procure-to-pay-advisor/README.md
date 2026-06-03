# Procure-to-Pay Advisor Skill

Multi-jurisdiction reference framework for procure-to-pay (P2P) accounting across US GAAP, IFRS, German HGB, JGAAP, India GST, and China VAT.

## Coverage

- **PO matching**: 2-way (PO vs. invoice), 3-way (PO vs. GRN vs. invoice), 4-way (adds inspection sign-off); tolerance policies; purchase price variance (PPV); quantity variance handling
- **AP accruals**: GRNI (goods received not invoiced) accrual recognition and reversal; period-end AP cutoff procedures; HGB §249 vs. IAS 37 vs. ASC 450 divergence
- **AP accounting**: invoice validation; early payment discounts (net vs. gross method — ASC 310 / IFRS 9); dynamic discounting; supply chain financing reclassification (IFRS IC Nov 2020; ASC 470 + ASU 2022-04 disclosure)
- **Vendor management**: vendor master controls; duplicate vendor detection; 1099-MISC/NEC and 1042-S (US); GDPR vendor data retention
- **Prepaid assets and commitments**: prepaid insurance/maintenance (ASC 340 / IAS 38 / HGB §250); accrued liabilities vs. AP distinction; purchase commitments and onerous contract loss recognition (ASC 440 / IAS 37)
- **VAT/GST**: input tax credit recovery; blocked input tax (entertainment, passenger vehicles); partial exemption for mixed-use entities; India GST Section 17(5) blocked credits; China VAT fapiao requirements
- **Procurement fraud controls**: SoD (PO creation, approval, receipt, payment, vendor master); three-lines-of-defence; vendor due diligence (sanctions, PEP, adverse media); FCPA / UK Bribery Act interaction

## Companion Agent

`accounting-procure-to-pay-advisor-agent`

## Standards Cited

ASC 210, ASC 310, ASC 330, ASC 340, ASC 420, ASC 440, ASC 450, ASC 470, ASU 2022-04, IAS 2, IAS 37, IAS 38, IFRS 9, IFRS IC Update Nov 2020, German HGB §249/§250/§253/§255/§257, UStG §15, JGAAP ASBJ Statement No. 9, India CGST Act 2017 Section 17, China VAT Reform Caishui [2016] No. 36, FCPA (15 U.S.C. §78dd-1), UK Bribery Act 2010
