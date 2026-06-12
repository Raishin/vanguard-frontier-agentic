//! Persistence module — SQLite index manager, schema migrations, single-writer task, and audit log.
//! Module skeleton — implemented in later waves.

pub mod schema;
pub mod index;
pub mod writer;
pub mod audit;
