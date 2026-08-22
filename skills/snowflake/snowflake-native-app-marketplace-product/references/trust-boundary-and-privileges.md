# Trust Boundary and Privileges

What a consumer is actually being asked to trust, and how to design an application that a governed enterprise can install. Load when reviewing a manifest or a setup script.

## The request list is a sales document

- An application requests account-level privileges and references from the consumer. That list is read by a security team that does not know the provider and has no reason to extend goodwill.
- Justify each request against a named, consumer-visible capability. A privilege that exists to simplify installation is the one that stalls the deal — and enterprise deals stall silently rather than being rejected, so the provider often never learns why.
- Prefer references — a consumer-granted binding to a specific object — over broad account privileges wherever the framework supports it. A reference is a scoped, revocable, auditable grant; an account privilege is not.
- State what a compromised or malicious version of the application could do with each requested privilege. That is the analysis the consumer's security team performs, and doing it first is cheaper than being surprised by it.
- Design so an installer who is not an account administrator can complete installation wherever possible. Requiring the most privileged human in the consumer organization for routine installation is an adoption tax.

## Both directions of the boundary

- **Provider to consumer:** what the application can read in the consumer account, what it can write, and what it can send outward through any external access it requests. This is the security questionnaire's first section.
- **Consumer to provider:** what telemetry, usage data, or shared-back data the provider receives, under what consent, and whether it can include consumer business data. This is the section that involves the consumer's legal team.
- Both answers should be written by the provider before a consumer asks. An application whose provider cannot state them precisely reads as one that has not considered them.
- Snowflake requires applications published externally to pass its automated security review. That establishes a floor for known unsafe patterns; it does not establish least privilege, a defensible trust boundary, or safe application behaviour, all of which remain the provider's responsibility.

## Application roles

- Application roles define what a consumer's users get inside the installed application. Design them as a real permission model, not as one role that grants everything the app contains.
- Separate the administrative role from the consuming role at minimum, so a consumer can grant use without granting configuration.
- State what each application role can reach, including any consumer objects bound through references. A consumer administrator will want to grant the narrowest role and will ask what it can see.
- Setup scripts execute in the consumer account. Review what the script creates, what it grants, and what it leaves behind if installation fails partway.

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/developer-guide/native-apps/requesting-about-privileges — How an application requests account-level privileges and references from a consumer, and the consumer-side grant model
- https://docs.snowflake.com/en/developer-guide/native-apps/security-app-security — The Native App security model and the automated security review requirement for externally published applications
