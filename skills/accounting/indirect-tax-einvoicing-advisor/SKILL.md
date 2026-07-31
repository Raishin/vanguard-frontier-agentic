---
name: indirect-tax-einvoicing-advisor
description: Multi-jurisdiction indirect tax and e-invoicing reference framework covering VAT/GST compliance and mandatory electronic invoicing mandates across EU, Brazil, India, Mexico, China, UK, and Australia.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Indirect Tax & E-Invoicing Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for indirect tax (VAT/GST) compliance and mandatory electronic invoicing advisory — from mandate status and technical format requirements through VAT treatment analysis, cross-border supply rules, and common compliance gap patterns.

---

## Part 1: EU VAT Framework and ViDA Reform

### EU VAT Directive 2006/112/EC — Core Structure

The EU VAT Directive (Council Directive 2006/112/EC) is the foundational legal instrument governing VAT across all 27 EU member states.

**Key structural elements:**
- **Taxable persons** (Art. 9): Any person carrying out economic activity independently
- **Taxable transactions** (Art. 2): Supply of goods, supply of services, intra-community acquisition, import
- **Place of supply — goods** (Art. 31–39): Generally where goods located at time of supply; special rules for dispatched/transported goods
- **Place of supply — services** (Art. 43–59b): B2B general rule — where customer established (reverse charge); B2C general rule — where supplier established; special rules for land, events, transport, digital services
- **VAT rates** (Art. 96–106): Standard rate ≥ 15%; reduced rates ≥ 5% for listed categories; zero rates permitted in specific cases
- **Invoice requirements** (Art. 226): 15 mandatory fields including date, sequential number, supplier/customer VAT number, description, amount, VAT rate, VAT amount
- **Reverse charge** (Art. 194–199b): Recipient accounts for VAT instead of supplier; mandatory for cross-border B2B services and optional for domestic supplies in certain sectors

### ViDA — VAT in the Digital Age (Adopted March 2025)

ViDA (Council Directive amending Directive 2006/112/EC) was adopted by the EU Council in March 2025 and introduces three pillars:

**Pillar 1 — Digital Reporting Requirements (DRR) and e-Invoicing:**

| Timeline | Requirement |
|---|---|
| 2024 (member state option) | Member states may introduce domestic mandatory e-invoicing without EU derogation |
| 2028 | Intra-EU B2B transactions: structured e-invoice mandatory (EN 16931 compliant) |
| 2030 | Mandatory real-time digital reporting of intra-EU B2B transaction data to EU central system |
| 2030 | Domestic B2B e-invoicing harmonization with DRR standard |

**E-invoice standard**: EN 16931 (European Standard on electronic invoicing) — defines semantic data model; syntax bindings include UBL 2.1 and UN/CEFACT CII.

**Pillar 2 — Platform Economy:**
- Digital platforms (accommodation, transport) become deemed supplier for VAT purposes from 2030
- VAT collected by platform on services facilitated to non-VAT-registered suppliers

**Pillar 3 — Single VAT Registration:**
- Extension of OSS/IOSS to cover B2B goods transfers
- Removal of need for multiple VAT registrations across EU

### OSS and IOSS

| Scheme | Scope | Registration | VAT Remittance |
|---|---|---|---|
| **OSS (One Stop Shop)** | Cross-border B2C services and distance sales of goods within EU; threshold €10,000 | Single member state of establishment | Quarterly via OSS portal of registration member state |
| **IOSS (Import One Stop Shop)** | Distance sales of goods imported from third countries, value ≤ €150 | Single EU member state or via intermediary | Monthly via IOSS portal |

### Country-Level E-Invoicing Mandates

#### Italy — SDI (Sistema di Interscambio)

**Status**: Live since 1 January 2019 — the first EU member state to introduce mandatory B2B e-invoicing.

| Scope | Requirement |
|---|---|
| B2B and B2C domestic supplies | Mandatory XML e-invoice via SDI hub since Jan 2019 |
| Cross-border supplies | FatturaPA in XML format required since Jan 2022 |
| Format | FatturaPA XML (Agenzia delle Entrate specification) |
| Model | **Clearance model**: invoices transmitted through SDI before or at time of delivery |
| Validation | SDI performs technical and tax number validation; rejected invoices are not legally issued |
| Archiving | 10-year electronic archiving obligation |
| Cancellation | File a nota di credito (credit note) — no direct cancellation mechanism post-SDI acceptance |

Source: Italian Legislative Decree 127/2015 as amended by DL 119/2018; Agenzia delle Entrate — https://www.agenziaentrate.gov.it/portale/web/english/nse/businesses/vat-in-italy

#### France — B2B E-Invoicing Mandate

**Status**: Phased rollout 2024–2026 (delayed from original 2023 target).

| Phase | Date | Scope |
|---|---|---|
| Receive obligation | 1 September 2026 | All VAT-registered businesses must be able to receive e-invoices |
| Issue obligation — large and mid-size | 1 September 2026 | Large enterprises and mid-size companies must issue e-invoices |
| Issue obligation — SMEs and micro | 1 September 2027 | All remaining VAT-registered businesses |

- **Model**: Y-model — invoices transmitted via accredited PDP (Plateforme de Dématérialisation Partenaire) operators to the Chorus Pro government portal (PPF), or directly to PPF
- **Format**: Factur-X (hybrid PDF/XML, subset of EN 16931), UBL 2.1, or UN/CEFACT CII
- **E-reporting**: Suppliers and platforms must transmit transaction data for B2C and cross-border transactions not covered by B2B mandate

Source: French Finance Law 2020 Art. 153; Decree 2022-1299; DGFiP implementation guidance

#### Germany — XRechnung and ZUGFeRD

**Status**: B2B mandatory e-invoicing from 1 January 2025 (receive obligation); issue obligation phased.

| Phase | Date | Scope |
|---|---|---|
| Receive obligation | 1 January 2025 | All German VAT-registered businesses must accept structured e-invoices |
| Issue — large enterprises (revenue > €800K) | 1 January 2027 | Must issue structured e-invoices for domestic B2B |
| Issue — all others | 1 January 2028 | All remaining domestic B2B |

- **Formats**:
  - **XRechnung**: Pure XML (UBL 2.1 or CII), legally preferred format for public sector (since 2020) and now B2B
  - **ZUGFeRD**: Hybrid PDF/A-3 with embedded XML (EN 16931 compliant); common for SMEs
- **Model**: Post-audit (Nachweismodell) — no real-time clearance hub; invoice sent directly to buyer with structured XML
- **Government procurement**: XRechnung mandatory for all public sector since 27 November 2020

Source: Wachstumschancengesetz (Growth Opportunities Act) 2024; UStG §14 as amended; KoSIT XRechnung specification

#### Poland — KSeF (Krajowy System e-Faktur)

**Status**: Mandatory from 1 February 2026 (delayed from July 2024).

- **Model**: Clearance model — all domestic B2B invoices must be issued through KSeF government platform before delivery
- **Format**: FA_VAT XML (Polish Ministry of Finance specification)
- **KSeF number**: Each invoice receives a KSeF reference number that serves as legal proof of issue
- **Archiving**: KSeF stores invoices for 10 years; taxpayers need not maintain separate archive for KSeF-issued invoices
- **Cancellation**: Invoices cannot be deleted from KSeF; corrections via korygujący (corrective invoice) only

Source: Polish Act of 29 October 2021 amending VAT Act (KSeF); Ministry of Finance KSeF specification

#### Romania — RO e-Factura

**Status**: Mandatory B2B from 1 January 2024 for large taxpayers; extended to all taxpayers.

- **Model**: Clearance model via ANAF (National Agency for Fiscal Administration) e-Factura system
- **Format**: UBL 2.1 XML
- **Deadline**: Invoice must be transmitted to e-Factura within 5 calendar days of issue date
- **B2G**: Mandatory since 2022

Source: Romanian Government Emergency Ordinance 120/2021; ANAF e-Factura technical specification

#### Spain — VERI*FACTU

**Status**: Mandatory from 1 July 2025 for large taxpayers; 1 January 2026 for all others (Verifactu regulation).

- **Model**: Post-audit with real-time reporting — each invoice generates a hash chain (registro de facturación) sent immediately to AEAT (Spanish Tax Agency) or stored for later verification
- **Format**: XML with mandatory hash chaining
- **SII (Suministro Inmediato de Información)**: Existing real-time VAT ledger reporting system (since 2017 for large taxpayers) feeds into VERI*FACTU framework
- **Complementary to SII**: VERI*FACTU applies to taxpayers not already on SII

Source: Royal Decree 1007/2023 (VERI*FACTU Reglamento de facturación); AEAT guidance

### Intrastat and EC Sales Lists

- **EC Sales Lists (ESL)**: Required for intra-community supplies of goods and services; being replaced by ViDA DRR digital reporting from 2030 for B2B supplies. Until then, ESL submitted monthly or quarterly depending on member state
- **Intrastat**: Statistical reporting of goods crossing EU borders; thresholds vary by member state (e.g., Germany €800K arrivals, €500K dispatches); ViDA does not eliminate Intrastat

---

## Part 2: Brazil — NF-e and SPED Ecosystem

### NF-e — Nota Fiscal Eletrônica

**Status**: Mandatory for most goods transactions since 2010; real-time clearance model.

| Element | Detail |
|---|---|
| Legal basis | Ajuste SINIEF 07/2005; CONFAZ protocol |
| XML layout | NF-e layout 4.0 (NFe_v4.00.xsd) |
| Authorization | Real-time SEFAZ (Secretaria da Fazenda) of issuing state via SOAP web service |
| Access key (chave de acesso) | 44-digit key; includes CNPJ, model, series, number, ICMS code, issuer type, emission date, random code |
| DANFE | Documento Auxiliar da NF-e — printed representation (A4 or DANFE Simplificado); not the legal document |
| CC-e (Carta de Correção Eletrônica) | Correction letter for minor errors; maximum 3 per NF-e; cannot correct: value, tax base, CFOP, emitter/recipient identity |
| Cancellation | Within 24 hours of authorization (or up to 168 hours with state SEFAZ permission); cancellation event transmitted to SEFAZ |
| Archiving | 5 years (XML files); issuer and recipient both must archive |

### NFS-e — Nota Fiscal de Serviços Eletrônica

- **Scope**: Services (ISS — Imposto Sobre Serviços); regulated at municipal level
- **ISS legal basis**: LC 116/2003 (Lei Complementar 116/2003) — national framework; municipalities set rates 2%–5%
- **NFS-e National Standard**: Federal government launched national NFS-e standard (Nota Fiscal de Serviços Eletrônica Nacional) in 2022; municipalities progressively adopting
- **Model**: Municipal clearance — NFS-e issued and authorized by municipal system; each municipality has own portal

### CT-e and MDF-e

| Document | Purpose |
|---|---|
| CT-e (Conhecimento de Transporte Eletrônico) | Electronic transport document for cargo transport by road, rail, air, water, or pipeline; mandatory for freight |
| MDF-e (Manifesto Eletrônico de Documentos Fiscais) | Groups NF-e and CT-e documents for a single transport route; required for interstate transport |

### SPED — Sistema Público de Escrituração Digital

| Module | Content |
|---|---|
| ECD (Escrituração Contábil Digital) | Digital bookkeeping — trial balance, chart of accounts, general ledger |
| ECF (Escrituração Contábil Fiscal) | Corporate income tax (IRPJ/CSLL) return; derives from ECD |
| EFD-ICMS/IPI | State VAT (ICMS) and federal excise (IPI) fiscal books; filed monthly per state |
| EFD-Contribuições | PIS/COFINS contributions fiscal books; filed monthly |

### ICMS — State VAT

- **Rates**: Interstate rates 4% (industrialized imports), 7% (South/Southeast to North/Northeast/Center-West states), 12% (other routes); internal state rates 17%–20%
- **DIFAL (Diferencial de Alíquota)**: On interstate B2C sales after EC 87/2015; split between origin and destination state; rules revised after STF ruling in 2021/2022
- **Substituição Tributária (ST)**: ICMS collected upfront by the first party in supply chain; applies to specific goods listed in interstate agreements (protocolos/convênios CONFAZ)

### PIS/COFINS

| Regime | PIS Rate | COFINS Rate | Credits |
|---|---|---|---|
| **Cumulative** (Regime Cumulativo) | 0.65% | 3.00% | No input credits |
| **Non-Cumulative** (Regime Não Cumulativo) | 1.65% | 7.60% | Full input credits on qualifying inputs |

- Lucro Real companies: non-cumulative regime
- Lucro Presumido / Simples Nacional companies: cumulative regime
- Legal basis: Laws 10.637/2002 (PIS non-cumulative) and 10.833/2003 (COFINS non-cumulative)

---

## Part 3: India — GST E-Invoice and Compliance Framework

### GST E-Invoice — IRP (Invoice Registration Portal)

**Status**: Mandatory for B2B outward supplies above threshold; clearance model.

| Element | Detail |
|---|---|
| Legal basis | CGST Rules 2017 Rule 48(4); CGST Act 2017 §31 |
| Current threshold | Annual aggregate turnover > ₹5 crore (as of August 2023; threshold progressively reduced since 2020) |
| Scope | B2B outward supplies; debit notes; credit notes; export invoices; supplies to SEZ |
| Excluded | B2C, NIL-rated, exempted supplies; financial institutions, GTA, insurance |
| IRN (Invoice Reference Number) | 64-character hash generated by IRP; unique identifier for each e-invoice |
| QR code | Embedded in invoice; contains IRN, supplier/recipient GSTIN, invoice value, date |
| IRP portals | NIC IRP (einvoice1.gst.gov.in), multiple private IRPs authorized by GSTN |
| Cancellation | Within 24 hours of IRN generation via IRP; after 24h, amendment via credit/debit note only |
| GSTR-1 auto-population | IRP data auto-populates GSTR-1 (outward supply return); reduces manual data entry errors |

Source: GSTN e-Invoice portal — https://einvoice1.gst.gov.in/

### GSTR Filing Obligations

| Return | Frequency | Content |
|---|---|---|
| GSTR-1 | Monthly (or quarterly under QRMP) | Outward supplies — auto-populated from e-invoices and POS data |
| GSTR-3B | Monthly | Summary return — output tax liability, input tax credit (ITC) claimed, net tax payment |
| GSTR-9 | Annual | Annual return reconciling GSTR-1 and GSTR-3B |
| GSTR-9C | Annual (turnover > ₹5 crore) | Reconciliation statement certified by CA/CMA |

### TDS and TCS under GST

- **TDS (§51 CGST Act)**: Government entities and notified persons deduct 2% (1% CGST + 1% SGST) on payments > ₹2.5 lakh to suppliers; supplier claims TDS credit in GSTR-2B
- **TCS (§52 CGST Act)**: E-commerce operators collect 1% (0.5% CGST + 0.5% SGST) on net value of taxable supplies made through platform

### E-Way Bill

- Required for movement of goods valued > ₹50,000 (interstate); intrastate thresholds vary by state
- Generated on e-way bill portal (ewaybillgst.gov.in); valid for distance-based time periods
- Auto-generated from e-invoice for B2B supplies via IRP integration

---

## Part 4: Mexico — CFDI 4.0

### CFDI 4.0 Overview

**Status**: Mandatory since 1 April 2023; supersedes CFDI 3.3.

| Element | Detail |
|---|---|
| Legal basis | Código Fiscal de la Federación (CFF) Art. 29, 29-A; SAT technical specifications |
| Full name | Comprobante Fiscal Digital por Internet versión 4.0 |
| Authorization | Real-time validation and stamping via PAC (Proveedor Autorizado de Certificación) authorized by SAT |
| UUID (folio fiscal) | 36-character unique identifier assigned by PAC at stamping; legal proof of issue |
| XML schema | Namespace cfdi: v4; Anexo 20 technical specification |
| Digital seal | CSD (Certificado de Sello Digital) — issuer's cryptographic signature; PAC adds its own stamp (timbre fiscal digital) |
| Mandatory new fields in v4.0 | Recipient's full legal name; recipient's RFC; recipient's postal code (domicilio fiscal); export indicator; periodicity for global CFDI |
| Archiving | 5 years (SAT may request XML files) |

### CFDI Complementos (Add-ons)

| Complemento | Purpose |
|---|---|
| Nómina (payroll) | Employee payroll payments; required for wage expense deductibility |
| Comercio Exterior | Cross-border goods exports; required for ISR/IVA deduction on exports |
| Carta Porte | Freight and transport; mandatory for road, rail, air, maritime transport within Mexico since 2022 |
| Pagos (payment receipt) | Records actual payment when invoice issued with deferred payment; reconciles accounts receivable |
| Leyendas Fiscales | Tax-required legends for specific industries |

### CFDI Cancellation — Motivos

| Motivo | Reason |
|---|---|
| 01 | Invoice issued with errors with a replacement invoice |
| 02 | Invoice issued with errors without a replacement invoice |
| 03 | Operation was not carried out |
| 04 | Nominative (relates to global or simplified invoice) |

- Recipient has 72 hours to accept or reject a cancellation request (for motivos 01/02/03)
- Automatic acceptance if no response within 72h
- Cancellation request transmitted via PAC to SAT cancellation service

### Global CFDI (POS)

- For B2C sales where individual CFDI is not required (small transactions)
- Issued with RFC XAXX010101000 (domestic) or XEXX010101000 (foreign)
- Issued daily, weekly, biweekly, or monthly depending on taxpayer regime
- Must include all B2C sales for the period not covered by individual CFDIs

---

## Part 5: China — Golden Tax System and Fapiao

### Fapiao System

The fapiao (发票) is China's official invoice and the primary mechanism for VAT input credit and corporate income tax deduction.

| Type | Purpose | VAT Credit |
|---|---|---|
| VAT Special Invoice (增值税专用发票) | B2B supplies; used to claim VAT input credit | Yes — buyer claims input credit |
| VAT Ordinary Invoice (增值税普通发票) | B2C or exempt supplies; documentation only | No input credit |
| Electronic Ordinary Invoice (电子普通发票) | Digital version of ordinary invoice | No input credit |
| Digital e-Fapiao (数电票) | New fully digital format replacing all above | Both special and ordinary versions |

### Golden Tax Phase IV — Digital E-Fapiao (数电票)

- **Status**: Piloted from 2021 (selected regions); national rollout ongoing 2023–2025; replacing paper and legacy electronic fapiao
- **Model**: Taxpayer issues digital fapiao through Golden Tax IV system (税务数字账户); no physical invoice; buyer receives digital copy
- **Format**: XML-based digital record assigned unique electronic invoice number (电子发票号码) by tax authority
- **Input VAT credit**: VAT special e-fapiao eligible for input credit; ordinary e-fapiao not eligible
- **Verification**: Buyer verifies via National VAT Invoice Inspection Platform (全国增值税发票查验平台)

### VAT Rates

| Rate | Applicable Sectors |
|---|---|
| 13% | Standard rate: goods, real estate, construction |
| 9% | Agricultural products, utilities, transportation, basic necessities |
| 6% | Services: financial services, modern services, consumer services, postal, telecoms |
| 0% | Exports (zero-rated with refund mechanism) |
| Exempt | Small-scale taxpayers below threshold; specified exempt services |

### Small-Scale Taxpayer Threshold

- Annual VAT-taxable turnover ≤ RMB 5 million → small-scale taxpayer; VAT collected at 3% (reduced to 1% during COVID relief periods)
- Above RMB 5 million → general taxpayer; eligible for VAT input credit; must use Golden Tax system for fapiao issuance

---

## Part 6: UK — Making Tax Digital

### MTD VAT

**Status**: Mandatory since April 2019 for all VAT-registered businesses (below £85K threshold since April 2022).

| Requirement | Detail |
|---|---|
| Legal basis | Finance Act 2016 §§122–131; VAT Notice 700/22 |
| Scope | All VAT-registered businesses filing UK VAT returns |
| Digital records | VAT records must be kept digitally (accounting software or spreadsheet with MTD-compatible bridging) |
| Digital links | End-to-end digital links required from source records through to VAT return submission; no manual re-keying |
| Submission | Via MTD-compatible software using HMRC API; direct submission to HMRC VAT API |
| Frequency | Quarterly (standard); monthly optional; annual accounting scheme available |

### VAT Return Boxes 1–9

| Box | Content |
|---|---|
| Box 1 | VAT due on sales and other outputs |
| Box 2 | VAT due on acquisitions from EU (post-Brexit: imports under postponed VAT accounting) |
| Box 3 | Total VAT due (Box 1 + Box 2) |
| Box 4 | VAT reclaimed on purchases and other inputs |
| Box 5 | Net VAT to pay to HMRC or reclaim (Box 3 minus Box 4) |
| Box 6 | Total value of sales and outputs (ex. VAT) |
| Box 7 | Total value of purchases and inputs (ex. VAT) |
| Box 8 | Total value of goods supplied to EU countries (post-Brexit: applicable to NI protocol only) |
| Box 9 | Total value of goods acquired from EU countries (post-Brexit: NI protocol) |

### MTD ITSA — Income Tax Self Assessment

**Status**: Effective 6 April 2026 (phased; originally 2023, delayed twice).

| Phase | Date | Scope |
|---|---|---|
| Phase 1 | 6 April 2026 | Self-employed and landlords with annual income > £50,000 |
| Phase 2 | 6 April 2027 | Self-employed and landlords with annual income > £30,000 |
| Phase 3 | TBD | Lower income threshold; general partnerships |

- **Quarterly updates**: 4 quarterly updates per tax year submitted digitally to HMRC via MTD-compatible software; not a full return — income and expenditure summary
- **End of Period Statement (EOPS)**: Annual statement confirming quarterly data; submitted after tax year end
- **Final Declaration**: Replaces Self Assessment tax return; confirms all income sources; filed by 31 January following tax year end

Source: HMRC MTD ITSA guidance — https://www.gov.uk/guidance/using-making-tax-digital-for-income-tax

---

## Part 7: Australia — GST and Peppol E-Invoicing

### GST Framework

| Element | Detail |
|---|---|
| Legal basis | A New Tax System (Goods and Services Tax) Act 1999 (GST Act) |
| Standard rate | 10% on taxable supplies |
| Taxable supply (s9-5) | Supply for consideration; in course of enterprise; connected with Australia; supplier registered or required to be registered |
| GST-free supplies (Div 38) | Basic food, medical services, childcare, education, exports |
| Input taxed supplies (Div 40) | Financial supplies, residential rent, precious metals |
| Registration threshold | Annual turnover ≥ AUD 75,000 (or AUD 150,000 for non-profit) |
| BAS (Business Activity Statement) | Periodic return reporting GST collected and GST credits (input tax credits) |
| Filing frequency | Monthly (turnover > AUD 20M), quarterly (standard), or annual |

### Peppol E-Invoicing

**Status**: Voluntary adoption; government committed to mandate review.

| Element | Detail |
|---|---|
| Standard | A-NZ Peppol BIS Billing 3.0 (based on European Peppol BIS Billing 3.0 with A-NZ extensions) |
| Network | Peppol 4-corner model: supplier → supplier's Access Point → recipient's Access Point → recipient |
| Government adoption | All Australian government agencies required to be Peppol-capable to receive e-invoices (from 1 July 2022) |
| B2B adoption | Voluntary; businesses encouraged but not yet mandated to adopt |
| ATO guidance | ATO supports Peppol as the national e-invoicing standard — https://www.ato.gov.au/business/gst/ |
| Mandatory mandate | Under ongoing review; no confirmed mandatory B2B date as of 2026 |

---

## Part 8: Jurisdiction Comparison Table

| Jurisdiction | Mandate Status | Format/Standard | Model | Cancellation |
|---|---|---|---|---|
| **Italy (SDI)** | Live (Jan 2019) — B2B & B2C | FatturaPA XML | Clearance (pre-delivery via SDI) | Credit note (nota di credito); no direct cancellation |
| **France** | Phased: receive Sep 2026; issue Sep 2026/Sep 2027 | Factur-X, UBL 2.1, CII | Y-model (PDP → PPF) | Amendment via corrective invoice |
| **Germany** | Receive: Jan 2025; Issue: Jan 2027/Jan 2028 | XRechnung (UBL/CII), ZUGFeRD | Post-audit (no central hub) | Correction invoice (Stornorechnung) |
| **Poland (KSeF)** | Mandatory Feb 2026 | FA_VAT XML | Clearance (KSeF platform) | Corrective invoice only; no deletion |
| **Romania (RO e-Factura)** | Mandatory Jan 2024 (all taxpayers) | UBL 2.1 XML | Clearance (ANAF within 5 days) | Storno via corrective invoice |
| **Spain (VERI*FACTU)** | Large: Jul 2025; All: Jan 2026 | XML with hash chain | Post-audit with real-time reporting to AEAT | Cancellation event transmitted to AEAT |
| **Brazil (NF-e)** | Live (mandatory since ~2010) | NF-e XML layout 4.0 | Clearance (real-time SEFAZ) | Cancel within 24h (168h with permission); correction via CC-e |
| **India (IRP)** | Mandatory (threshold ₹5 crore) | JSON → IRP validates; PDF output | Clearance (IRP → IRN+QR) | Cancel within 24h of IRN; after that, credit/debit note |
| **Mexico (CFDI 4.0)** | Live (mandatory Apr 2023) | XML + PAC stamp (UUID) | Clearance (real-time via PAC) | Motivos 01–04; recipient 72h accept/reject |
| **China (Golden Tax IV)** | Rolling out 2023–2025 nationally | Digital e-fapiao XML | Government-issued (tax authority assigns) | No cancellation; red-letter fapiao (冲红) for corrections |
| **UK (MTD VAT)** | Mandatory (all VAT-registered) | API submission via MTD software | Post-audit (HMRC digital records + API) | Amendment via adjusted VAT return |
| **Australia (Peppol)** | Voluntary (government: mandatory receive) | Peppol BIS Billing 3.0 | 4-corner network (post-audit) | Business-level credit note |

---

## Part 9: Common Compliance Gaps and Risk Flags

| Gap | Jurisdiction | Article/Rule Violated | Detection |
|---|---|---|---|
| Missing mandatory invoice fields (e.g., VAT number, sequential number) | EU all | Art. 226 EU VAT Directive 2006/112/EC | Invoice completeness audit against Art. 226 checklist |
| Incorrect place of supply for B2B services | EU | Art. 44 EU VAT Directive | Review service contracts for establishment facts |
| OSS threshold exceeded without registration | EU | Art. 59c EU VAT Directive | Monthly B2C cross-border sales monitoring |
| NF-e XML rejected by SEFAZ, paper note used without authorization | Brazil | Ajuste SINIEF 07/2005 | SEFAZ rejection log review |
| CC-e used to correct value or emitter identity | Brazil | CONFAZ CC-e rules | CC-e field validation |
| Missing IRN in B2B invoice above threshold | India | CGST Rules 2017 Rule 48(4) | Invoice register IRN coverage check |
| IRP cancellation attempted after 24h window | India | CGST e-Invoice FAQ — GSTN | IRN timestamp vs. cancellation request timestamp |
| CFDI issued without valid PAC stamp (UUID missing) | Mexico | CFF Art. 29, 29-A | UUID presence check in XML |
| CFDI carta porte absent for goods transport | Mexico | SAT carta porte regulation (2022) | Transport document review |
| VAT special fapiao issued by non-general taxpayer | China | MOF VAT regulations | Taxpayer classification check |
| Digital link broken — manual re-keying into VAT return | UK | Finance Act 2016 §122; VAT Notice 700/22 | End-to-end data flow audit |
| MTD ITSA quarterly update missed | UK | Finance Act 2021 MTD ITSA provisions | Quarterly calendar compliance check |
| BAS not filed on time | Australia | GST Act s31-10 | ATO lodgment due date tracking |

---

## Part 10: Official Documentation — Publicly Accessible URLs

| Jurisdiction/Standard | Resource | URL |
|---|---|---|
| EU VAT Directive 2006/112/EC | EUR-Lex | https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32006L0112 |
| Italy SDI / FatturaPA | Agenzia delle Entrate (English) | https://www.agenziaentrate.gov.it/portale/web/english/nse/businesses/vat-in-italy |
| India GST e-Invoice IRP | GSTN / NIC IRP | https://einvoice1.gst.gov.in/ |
| Mexico CFDI 4.0 | SAT | https://www.sat.gob.mx/consultas/98850/comprobantes-fiscales-digitales-por-internet |
| UK MTD VAT | HMRC | https://www.gov.uk/government/collections/making-tax-digital-for-vat |
| UK MTD ITSA | HMRC | https://www.gov.uk/guidance/using-making-tax-digital-for-income-tax |
| Australia GST / Peppol | ATO | https://www.ato.gov.au/business/gst/ |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and scenario described. Indirect tax law and e-invoicing mandates change frequently and vary by taxpayer category, transaction type, and registration threshold. Formal compliance filings require qualified local tax advisors and certified software providers (PAC, IRP-registered ASP, or equivalent). This analysis does not constitute a tax opinion in any jurisdiction.
