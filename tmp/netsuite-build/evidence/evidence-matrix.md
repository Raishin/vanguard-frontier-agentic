# NetSuite Enterprise Agent Build — Evidence Matrix

**Compiled:** 2026-06-09  
**Methodology:** All claims verified against Oracle/NetSuite-owned domains only (`docs.oracle.com`, `netsuite.com`, `education.oracle.com`, `mylearn.oracle.com`). Third-party sources not used. Pages returning HTTP 403/503 are noted — those specific pages were inaccessible but the same claim was confirmed from at least one alternate official URL.

---

## Evidence Matrix

| # | Claim | Official Source URL | Evidence Label | Last Verified | Release-Sensitive (Y/N) | Notes |
|---|-------|-------------------|----------------|---------------|------------------------|-------|
| 1a | NetSuite certification program has 5 tracks: Accounting & Finance, BI & Reporting, Consultant & Administrator, Developer, and AI | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Main certification page; netsuite.com returned HTTP 403 to direct fetch but confirmed via search result snippets from the same URL |
| 1b | AI track: AI Foundations Associate — exam page exists at education.oracle.com (N16765GC10); Specialist and Professional "coming soon" | https://education.oracle.com/oracle-netsuite-ai-foundations-associate/pexam_N16765GC10 | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | Exam page returned HTTP 503 on direct fetch; confirmed via Oracle Education search results and mylearn.oracle.com learning-path URL |
| 1c | Accounting & Finance track: Financial Associate (free), AP Specialist, AR Specialist, FP&A Specialist, Accounting Professional, FP&A Professional — all active | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via search snippets from netsuite.com; individual exam N16301GC10 (Accounting Professional) confirmed at education.oracle.com |
| 1d | BI & Reporting track: BI and Reporting Associate (free, N16724GC10), BI and Reporting Specialist (N16740GC10) — both active; Professional level implied but not confirmed available | https://education.oracle.com/oracle-netsuite-bi-and-reporting-associate/pexam_N16724GC10 | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Both exam pages confirmed present in Oracle Education search results |
| 1e | Consultant & Administrator track: SuiteFoundation Specialist (N16300GC10), Administrator Professional (N16291GC10), ERP Consultant Professional (N16302GC10) — all active | https://education.oracle.com/oracle-netsuite-erp-consultant-professional/pexam_N16302GC10 | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | All three exam pages confirmed in Oracle Education search results; individual pages returned HTTP 503 but search results confirm existence |
| 1f | Developer track: Application Developer Professional (N16304GC10) active; Web Services Developer and SuiteCloud Developer recognition also referenced; additional developer certifications described as "coming soon" | https://education.oracle.com/oracle-netsuite-application-developer-professional/pexam_N16304GC10 | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via search snippets from education.oracle.com |
| 1g | SuiteFoundation: cross-track prerequisite for Admin, ERP Consultant, SuiteCloud Developer; intended for power users, admins, recent implementation team members | https://learn.oracle.com/ols/module/netsuite-certification-suitefoundation-exam-preparation/85171/86403 | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via Oracle MyLearn search snippet |
| 1h | Financial User exam (N16599GC10) exists as a separate foundational certification | https://education.oracle.com/oracle-netsuite-financial-user/pexam_N16599GC10 | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via Oracle Education search results |
| 2a | Starting with the 2026.1 release, all **newly built** integrations should use REST web services with OAuth 2.0; NetSuite will no longer provide a new SOAP endpoint with each release | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_2104046421.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | Direct fetch of SOAP Removal Plans FAQ confirmed verbatim: "Starting with the 2026.1 NetSuite release, all newly built integrations should use REST web services with OAuth 2.0." |
| 2b | With the 2027.1 release, it will no longer be possible to build **any** new integrations using SOAP web services | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_2104046421.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | Verbatim: "With the 2027.1 release, you will not be able to build any new integrations using SOAP web services." Also: no new TBA integrations for SOAP as of 2027.1 |
| 2c | 2025.2 SOAP endpoint is the last planned SOAP endpoint; from 2027.2 only that endpoint is supported and may receive bug fixes | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_2104046421.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | "The 2025.2 SOAP endpoint is the last planned SOAP endpoint and any later SOAP endpoints would be released only as necessary." |
| 2d | With the 2028.2 release, **all** SOAP endpoints are disabled and SOAP-based integrations stop working entirely | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_2104046421.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | Final end-of-life milestone confirmed via direct fetch |
| 3a | OAuth 2.0 is supported for REST web services | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157780312610.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "OAuth 2.0 is only available for REST web services and RESTlets." |
| 3b | OAuth 2.0 is supported for RESTlets | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_158263562006.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Dedicated page "Setting up OAuth 2.0 for a RESTlet Integration" confirms support |
| 3c | OAuth 2.0 is supported for SuiteAnalytics Connect (NetSuite2.com data source) | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_1011040638.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Dedicated page "OAuth 2.0 for SuiteAnalytics Connect" confirms support; also requires "Log in using OAuth 2.0 Access Tokens" permission |
| 3d | OAuth 2.0 is NOT supported for SOAP web services | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157780312610.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "SOAP web services don't support OAuth 2.0." |
| 4a | TBA (Token-Based Authentication) is supported for SOAP web services (existing integrations), REST web services, and RESTlets | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_4381113277.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | Confirmed for SOAP via "Token-based Authentication and Web Services" page; note: new TBA integrations for SOAP end at 2027.1 |
| 4b | User credentials (NLAuth / Passport) are NOT supported for new RESTlets (deprecated as of 2021) | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N2971402.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "NetSuite doesn't support user credentials authentication for new RESTlets." Session cookies still valid for same-account internal calls |
| 4c | User credentials (Passport complex type / request-level credentials) are NOT supported for SOAP web services using endpoint 2020.2 or later | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N3445710.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "As of the 2020.2 SOAP web services endpoint, authentication through request-level credentials is not supported." TBA is required for 2020.2+ SOAP endpoints |
| 4d | As of 2027.1, no new TBA integrations can be created for SOAP, REST, or RESTlets; existing TBA integrations continue working | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/chapter_4247329078.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | Y | OAuth 2.0 is the recommended path for all new integrations post-2027.1 |
| 5a | 2FA is required for the Administrator role in all NetSuite accounts (production, sandbox, development, Release Preview) | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_1532968056.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "NetSuite requires two-factor authentication (2FA) for all Administrator and other highly privileged roles when logging to any NetSuite account." |
| 5b | 2FA is mandatory for other highly privileged roles; this requirement is set by default and cannot be removed | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N328126.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed: Administrator + roles with highly privileged permissions are set as 2FA-required by default; full list visible only inside account at Setup > Users/Roles > Two-Factor Authentication Roles |
| 5c | Specific permissions that trigger mandatory 2FA include: Access Token Management, OAuth 2.0 Authorized Applications Management, Core Administration Permissions, View Unencrypted Credit Cards, View Unencrypted ACH Account Numbers, and several SSO/OIDC setup permissions | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_1515446005.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via "Permissions Requiring Two-Factor Authentication (2FA)" page |
| 5d | 2FA-required designation can be applied to Employee Center, Partner Center, and Vendor Center roles, but NOT to Customer Center roles | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N328126.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via "Designate Two-Factor Authentication Roles" page |
| 6a | The NetSuite AI Connector Service does NOT support the Administrator role or any role with full permissions to access NetSuite features | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "it is not possible to run the NetSuite AI Connector Service if you've logged in to NetSuite with the Administrator role or roles that have full permissions to access NetSuite features." |
| 6b | Required permission: **MCP Server Connection** | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed verbatim on the "Required Features and Permissions" page |
| 6c | Required permission: **Log in using OAuth 2.0 Access Tokens** (distinct from "Log in using Access Tokens") | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "Do not confuse it with the Log in using Access Tokens permission." Also confirmed at FAQ page (article_4160616848.html) |
| 6d | Required features: Server SuiteScript and OAuth 2.0 must be enabled; REST Web Services also required if using MCP Standard Tools SuiteApp | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Also cross-confirmed at https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_4160616848.html |
| 6e | AI Connector cannot be activated for healthcare customers with a signed BAA (HIPAA/BAA restriction) | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Explicitly stated on Required Features and Permissions page |
| 7a | You cannot modify standard roles directly; best practice is to create a custom copy of a standard role, then modify it | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N285436.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "It's best to start with a copy of the standard roles built into NetSuite before you customize them." |
| 7b | The least privilege principle is explicitly stated: "Giving users only the access they need helps avoid showing restricted pages, records, and data." | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N295396.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed via "Standard Roles Permissions Table" page, which restates the least-privilege principle |
| 7c | Users should not be given responsibility for more than one related function (separation of duties principle also documented) | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N285436.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim from NetSuite Roles Overview page |
| 8a | OAuth 2.0 authorized applications in a production account are **not** copied to Release Preview or sandbox accounts; users must explicitly authorize in each environment | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157771979135.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "Applications authorized using the OAuth 2.0 feature in your NetSuite production account aren't copied to your Release Preview or to your sandbox accounts. Users must authorize applications explicitly in Release Preview or in a sandbox to test OAuth 2.0 feature in these accounts." |
| 8b | Each time a sandbox is refreshed, users must re-authorize OAuth 2.0 applications explicitly in that sandbox | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157771979135.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "Each time the sandbox is refreshed, users must authorize applications explicitly in the sandbox." |
| 8c | OAuth 2.0 client credentials flow setup is also not copied to other production accounts, Release Preview, or sandbox accounts; cleared on sandbox refresh | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_162686838198.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Verbatim: "The client credentials flow setup in your NetSuite production account isn't copied to any other production account, Release Preview account, or sandbox account." |
| 8d | TBA tokens created in production are not copied to sandbox or Release Preview; new tokens must be created in those environments | https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_4254801119.html | OFFICIAL_DOCUMENTATION | 2026-06-09 | N | Confirmed: same sandbox isolation applies to TBA tokens, not just OAuth 2.0 |

---

## Certification Matrix

### Track → Cert → Status → Source

#### Accounting & Finance Track
| Certification | Level | Status | Intended Profile | Source |
|--------------|-------|--------|-----------------|--------|
| Financial User | Entry/Associate | Available (free for Pass holders) | End users of financial modules | https://education.oracle.com/oracle-netsuite-financial-user/pexam_N16599GC10 |
| Financial Associate | Associate | Available (free for Pass holders) | Finance end users, AP/AR/FP&A roles | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| AP Specialist | Specialist | Available | Accounts Payable specialists | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| AR Specialist | Specialist | Available | Accounts Receivable specialists | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| FP&A Specialist | Specialist | Available | Financial Planning & Analysis roles | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| Accounting Professional | Professional | Available (N16301GC10) | Senior accountants, controllers | https://education.oracle.com/oracle-netsuite-accounting-professional/pexam_N16301GC10 |
| FP&A Professional | Professional | Available | Senior FP&A professionals | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |

**Note:** Individual exam pages for Specialist-level certs (AP, AR, FP&A Specialist) returned HTTP 503 during this research session; existence confirmed via search snippets from netsuite.com.

#### BI & Reporting Track
| Certification | Level | Status | Intended Profile | Source |
|--------------|-------|--------|-----------------|--------|
| BI and Reporting Associate | Associate | Available (free for Pass holders) | Report builders, dashboard creators | https://education.oracle.com/oracle-netsuite-bi-and-reporting-associate/pexam_N16724GC10 |
| BI and Reporting Specialist | Specialist | Available (N16740GC10) | Advanced analytics users | https://education.oracle.com/oracle-netsuite-bi-and-reporting-specialist/pexam_N16740GC10 |
| BI and Reporting Professional | Professional | Status UNVERIFIED — not confirmed available or "coming soon" in fetched pages | Senior BI practitioners | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |

**Note:** BI Professional level is referenced in track descriptions but its availability status was not confirmed from a directly fetchable official page during this research session.

#### Consultant & Administrator Track
| Certification | Level | Status | Intended Profile | Source |
|--------------|-------|--------|-----------------|--------|
| SuiteFoundation Specialist | Specialist/Foundation | Available (N16300GC10) | Power users, admins, implementation team members; prerequisite for Admin/ERP Consultant/SuiteCloud Developer | https://education.oracle.com/oracle-netsuite-suitefoundation-specialist/pexam_N16300GC10 |
| Administrator Professional | Professional | Available (N16291GC10) | NetSuite administrators with 1+ year hands-on experience; requires SuiteFoundation | https://education.oracle.com/oracle-netsuite-administrator-professional/pexam_N16291GC10 |
| ERP Consultant Professional | Professional | Available (N16302GC10) | Implementation consultants; highest technical expertise level | https://education.oracle.com/oracle-netsuite-erp-consultant-professional/pexam_N16302GC10 |

**Note:** Additional Consultant & Administrator certifications described as "coming soon" on main certification page, but no specific exam names were confirmed.

#### Developer Track
| Certification | Level | Status | Intended Profile | Source |
|--------------|-------|--------|-----------------|--------|
| Application Developer Professional | Professional | Available (N16304GC10) | Developers proficient in SuiteScript, SuiteFlow, SuiteBuilder, SuiteCloud Development Framework | https://education.oracle.com/oracle-netsuite-application-developer-professional/pexam_N16304GC10 |
| Web Services Developer | Specialist/Professional | Status UNVERIFIED — referenced but specific exam page not confirmed fetchable | Integration developers, SOAP/REST API developers | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| SuiteCloud Developer | Recognition/Advanced | Status UNVERIFIED — referenced as a "recognition" credential; no exam page confirmed | Expert-level SuiteCloud platform developers | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |

**Note:** Main certification page states "additional developer certifications coming soon" without specifying which ones.

#### AI Track
| Certification | Level | Status | Intended Profile | Source |
|--------------|-------|--------|-----------------|--------|
| AI Foundations Associate | Associate | Available (free for Pass holders; N16765GC10) | All NetSuite users; introduces foundational AI concepts aligned to platform capabilities | https://education.oracle.com/oracle-netsuite-ai-foundations-associate/pexam_N16765GC10 |
| AI Specialist | Specialist | Coming Soon (referenced on main certification page, no exam page confirmed) | TBD | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |
| AI Professional | Professional | Coming Soon (referenced on main certification page, no exam page confirmed) | TBD | https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml |

---

## Key Corrections / Nuances vs. Original Claims

1. **SOAP deprecation releases** — The original claims of "2026.1" and "2027.1" are **both confirmed correct**. 2026.1 = recommendation to stop new SOAP integrations; 2027.1 = hard block on new SOAP integrations. Full sunset is 2028.2.
2. **OAuth 2.0 + SOAP** — Confirmed NOT supported. SOAP requires TBA (for existing integrations until 2027.1).
3. **TBA + user credentials for SOAP** — User credentials were removed from SOAP at the 2020.2 endpoint, not recently. TBA is still valid for existing SOAP integrations but cannot be used for new ones from 2027.1.
4. **AI Connector permission name** — The precise permission is **"Log in using OAuth 2.0 Access Tokens"**, not "OAuth 2.0 Access Tokens." The FAQ page and Required Features page both call out this common confusion.
5. **Sandbox OAuth 2.0 re-authorization** — Fully confirmed for both Authorization Code flow AND Client Credentials flow. Both require explicit setup in sandbox separately and reset on sandbox refresh.

---

## Source Index (Official Oracle/NetSuite Domains Only)

| URL | Domain | Topic |
|-----|--------|-------|
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_2104046421.html | docs.oracle.com | SOAP Removal Plans FAQ |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N3445710.html | docs.oracle.com | Authentication for SOAP Web Services |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N2971402.html | docs.oracle.com | Authentication for RESTlets |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/chapter_157769826287.html | docs.oracle.com | OAuth 2.0 Overview |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157780312610.html | docs.oracle.com | OAuth 2.0 for REST Web Services |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_158263562006.html | docs.oracle.com | Setting up OAuth 2.0 for a RESTlet Integration |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_1011040638.html | docs.oracle.com | OAuth 2.0 for SuiteAnalytics Connect |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_4381113277.html | docs.oracle.com | Token-based Authentication and Web Services |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_4254801119.html | docs.oracle.com | Token-based Authentication and RESTlets |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/chapter_4247329078.html | docs.oracle.com | Token-based Authentication (TBA) Overview |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_1532968056.html | docs.oracle.com | Mandatory 2FA for NetSuite Access |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N328126.html | docs.oracle.com | Designate Two-Factor Authentication Roles |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_1515446005.html | docs.oracle.com | Permissions Requiring 2FA |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_0714080625.html | docs.oracle.com | AI Connector Required Features and Permissions |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_4160616848.html | docs.oracle.com | NetSuite AI Connector Service FAQ |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N285436.html | docs.oracle.com | NetSuite Roles Overview |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_N295396.html | docs.oracle.com | Standard Roles Permissions Table |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_157771979135.html | docs.oracle.com | Managing OAuth 2.0 Authorized Applications |
| https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/section_162686838198.html | docs.oracle.com | OAuth 2.0 Client Credentials Setup |
| https://www.netsuite.com/portal/services/training/suite-training/netsuite-certification.shtml | netsuite.com | NetSuite Certification Main Page |
| https://education.oracle.com/oracle-netsuite-ai-foundations-associate/pexam_N16765GC10 | education.oracle.com | AI Foundations Associate Exam |
| https://education.oracle.com/oracle-netsuite-accounting-professional/pexam_N16301GC10 | education.oracle.com | Accounting Professional Exam |
| https://education.oracle.com/oracle-netsuite-erp-consultant-professional/pexam_N16302GC10 | education.oracle.com | ERP Consultant Professional Exam |
| https://education.oracle.com/oracle-netsuite-suitefoundation-specialist/pexam_N16300GC10 | education.oracle.com | SuiteFoundation Specialist Exam |
| https://education.oracle.com/oracle-netsuite-bi-and-reporting-associate/pexam_N16724GC10 | education.oracle.com | BI and Reporting Associate Exam |
| https://education.oracle.com/oracle-netsuite-bi-and-reporting-specialist/pexam_N16740GC10 | education.oracle.com | BI and Reporting Specialist Exam |
| https://education.oracle.com/oracle-netsuite-application-developer-professional/pexam_N16304GC10 | education.oracle.com | Application Developer Professional Exam |
| https://education.oracle.com/oracle-netsuite-administrator-professional/pexam_N16291GC10 | education.oracle.com | Administrator Professional Exam |
| https://education.oracle.com/oracle-netsuite-financial-user/pexam_N16599GC10 | education.oracle.com | Financial User Exam |
