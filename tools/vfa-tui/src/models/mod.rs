pub mod agent;
pub mod export;
pub mod gate;
pub mod harness;
pub mod integrity;
pub mod mcp_ref;
pub mod provider;
pub mod role;
pub mod rule;
pub mod skill;

// Re-export all model types for convenient access.
// Some may appear unused in the binary but are used by the lib crate consumers.
#[allow(unused_imports)]
pub use agent::{Agent, AgentType, ExecutionTier, Lifecycle};
#[allow(unused_imports)]
pub use export::{ExportCommand, ExportSelection};
#[allow(unused_imports)]
pub use gate::{extract_validation_gates, GateStatus, ValidationGate};
#[allow(unused_imports)]
pub use harness::{Harness, SourceType};
#[allow(unused_imports)]
pub use integrity::{AssetIntegrity, IntegrityFile, IntegrityScope, IntegrityTree};
#[allow(unused_imports)]
pub use mcp_ref::{McpReference, McpType, PinStrategy, SignedRelease, TrustMatrix};
#[allow(unused_imports)]
pub use provider::Provider;
#[allow(unused_imports)]
pub use role::{Role, RoleCatalog};
#[allow(unused_imports)]
pub use rule::{Rule, RuleType};
#[allow(unused_imports)]
pub use skill::{Skill, SkillType};
