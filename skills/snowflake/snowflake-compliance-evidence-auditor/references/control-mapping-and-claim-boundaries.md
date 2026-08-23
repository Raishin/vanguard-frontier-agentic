# Control Mapping and Claim Boundaries

How to map an organization's control statements to Snowflake evidence without manufacturing assurance. Load when a control map or a compliance assertion is being produced.

## Mapping rules

- Map a control to evidence, not to a feature. 'We use masking policies' is a feature statement; 'column X carried policy Y throughout the period, and access to it is recorded in ACCESS_HISTORY' is evidence.
- For every mapping, write the negative: what this evidence does not establish. A mapping without its negative overstates assurance by omission, which is the failure mode auditors are trained to find.
- Where no queryable evidence exists for a control, say so. An unmapped control is a finding; a control mapped to weak evidence and presented as satisfied is a misstatement.
- Use primary standard sources for the control statement itself and label them `STANDARD-BASED`. Do not let a vendor page define what a regulation requires.
- Never assert `feature enabled ⇒ regulatory compliance`. The gap between them is where audit findings live, and closing it rhetorically is the failure this agent exists to prevent.

## Segregation of duties from the grant graph

- The question is not 'is there an approval step' but 'could one principal have made and approved the change'. Answer it from what roles a principal held during the period and what they actually did.
- Include the roles held transitively. A principal who could assume a role that could approve is not segregated from approval.
- Include automation identities. A pipeline identity that can both deploy and grant is a segregation-of-duties conflict with a machine on one side, and it is usually the one nobody checks.
- Include privileges held briefly. A grant created and removed inside the period counts; a snapshot query will not show it.

## Claim boundaries this agent enforces

- This agent reports evidence coverage. It does not certify, attest, opine on, or endorse compliance with any framework — that conclusion belongs to the organization's compliance function and its auditors, and the report names them.
- Where an unsupported external assertion has already been made, that is escalated ahead of every other finding, because the exposure is already live.
- A request to 'just say we meet the requirement' is refused, and the refusal is recorded in the report along with what the evidence does support.
