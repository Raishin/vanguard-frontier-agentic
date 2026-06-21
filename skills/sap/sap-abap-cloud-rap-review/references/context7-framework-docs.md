# Context7 framework docs — SAP ABAP Cloud RAP Review

**Role**: supplementary. Official SAP ABAP Cloud and RAP documentation (help.sap.com/docs/abap-cloud) is the primary source for all RAP architecture guidance. Context7-sourced RAP openSAP sample code supplements with implementation-level patterns for authorization checks and ABAP unit testing.

**Library used**: ABAP RESTful Application Programming Model openSAP Samples
Context7 library ID: `/sap-samples/abap-platform-rap-opensap`
Lookup target: RAP behavior definition authorization checks, AUTHORITY-CHECK implementation, ABAP unit test patterns, CDS test environment, ROLLBACK ENTITIES
Skill: `sap-abap-cloud-rap-review`
Classification: supplementary

---

## RAP authorization — AUTHORITY-CHECK for update operations (supplementary)

Source: sap-samples/abap-platform-rap-opensap (Context7 `/sap-samples/abap-platform-rap-opensap`)
Reference: https://github.com/sap-samples/abap-platform-rap-opensap/blob/main/week3/sources/W3U7_CLAS_ZBP_I_RAP_TRAVEL.txt

```abap
METHOD is_update_granted.
    IF has_before_image = abap_true.
      AUTHORITY-CHECK OBJECT 'ZOSTAT####'
        ID 'ZOSTAT####' FIELD travel_status
        ID 'ACTVT' FIELD '02'.
    ELSE.
      AUTHORITY-CHECK OBJECT 'ZOSTAT####'
        ID 'ZOSTAT####' DUMMY
        ID 'ACTVT' FIELD '02'.
    ENDIF.
    update_granted = COND #( WHEN sy-subrc = 0 THEN abap_true ELSE abap_false ).

    " Simulate full access - for testing purposes only! Needs to be removed for a productive implementation.
    update_granted = abap_true.
  ENDMETHOD.
```

**Relevance**: The `update_granted = abap_true` line is a documented development-only simulation. Any production-bound behavior implementation containing this line without removal is a `critical` finding. The correct production pattern is `AUTHORITY-CHECK` with `sy-subrc = 0` evaluation only — no pass-through override.

Activity codes: `'02'` = Update, `'06'` = Delete, `'01'` = Create.

---

## RAP authorization — operation-level result mapping (supplementary)

Source: sap-samples/abap-platform-rap-opensap (Context7 `/sap-samples/abap-platform-rap-opensap`)
Reference: https://github.com/sap-samples/abap-platform-rap-opensap/blob/main/week3/unit6.md

```abap
APPEND VALUE #( %tky = travel-%tky
                %update              = COND #( WHEN update_granted = abap_true THEN if_abap_behv=>auth-allowed ELSE if_abap_behv=>auth-unauthorized )
                %action-acceptTravel = COND #( WHEN update_granted = abap_true THEN if_abap_behv=>auth-allowed ELSE if_abap_behv=>auth-unauthorized )
                %action-rejectTravel = COND #( WHEN update_granted = abap_true THEN if_abap_behv=>auth-allowed ELSE if_abap_behv=>auth-unauthorized )
                %assoc-_Booking      = COND #( WHEN update_granted = abap_true THEN if_abap_behv=>auth-allowed ELSE if_abap_behv=>auth-unauthorized )
                %delete              = COND #( WHEN delete_granted = abap_true THEN if_abap_behv=>auth-allowed ELSE if_abap_behv=>auth-unauthorized )
              )
  TO result.
```

**Relevance**: The `result` table in `CHECK_AUTHORIZATION` must enumerate all operations (`%update`, `%delete`, `%action-*`, `%assoc-*`) and assign `if_abap_behv=>auth-allowed` or `if_abap_behv=>auth-unauthorized` for each. A missing operation entry results in the RAP framework defaulting to `auth-allowed` for that operation — a silent authorization bypass.

---

## RAP unit testing — integration test class structure (supplementary)

Source: sap-samples/abap-platform-rap-opensap (Context7 `/sap-samples/abap-platform-rap-opensap`)
Reference: https://github.com/sap-samples/abap-platform-rap-opensap/blob/main/week4/unit3.md

```abap
CLASS ltcl_integration_test DEFINITION FINAL FOR TESTING
     DURATION SHORT
     RISK LEVEL HARMLESS.

     PRIVATE SECTION.
        CLASS-DATA:
         cds_test_environment TYPE REF TO if_cds_test_environment.

        CLASS-METHODS:
         class_setup,
         class_teardown.
        METHODS:
         setup,
         teardown.
        METHODS:
         create_travel FOR TESTING RAISING cx_static_check.
    ENDCLASS.

    CLASS ltcl_integration_test IMPLEMENTATION.
      METHOD class_setup.
        cds_test_environment = cl_cds_test_environment=>create_for_multiple_cds(
            i_for_entities = VALUE #( ( i_for_entity = 'zi_rap_travel_u_####' )
                                    ( i_for_entity = 'zi_rap_booking_u_####' ) )
                                  ).
      ENDMETHOD.

      METHOD class_teardown.
        cds_test_environment->destroy( ).
      ENDMETHOD.

      METHOD teardown.
        ROLLBACK ENTITIES.
        cds_test_environment->clear_doubles( ).
      ENDMETHOD.
    ENDCLASS.
```

**Relevance**: Required structural pattern for RAP integration tests:
- `cl_cds_test_environment=>create_for_multiple_cds()` creates CDS test doubles to avoid live data access.
- `cds_test_environment->destroy()` in `class_teardown` is mandatory to clean up test doubles.
- `ROLLBACK ENTITIES` in `teardown` is mandatory to roll back any entity modifications between test methods.
- Missing any of these constitutes a test isolation finding.

---

## Scope boundaries for Context7 usage

Context7 RAP openSAP sample documentation applies to:

- **Authorization implementation**: `AUTHORITY-CHECK` patterns, operation-level result mapping in `CHECK_AUTHORIZATION`
- **ABAP unit tests**: CDS test environment setup, test class structure, `ROLLBACK ENTITIES` teardown

It does not replace official SAP Help Portal guidance for:
- BDEF syntax and keyword reference (use help.sap.com/docs/abap-cloud/abap-rap/behavior-definition)
- CDS view type definitions and access control (use help.sap.com/docs/abap-cloud/abap-rap/cds-data-model)
- Released API release contract verification (use api.sap.com)
- ABAP Cloud tier-2 forbidden construct list (use help.sap.com/docs/abap-cloud/abap-cloud/what-is-abap-cloud)

Always label Context7-sourced guidance as `context7-supplementary` in responses.
