use std::time::Instant;

/// Identifies which output stream a line came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputStream {
    Stdout,
    Stderr,
}

/// A single line of output from a subprocess.
#[derive(Debug, Clone)]
pub struct OutputLine {
    pub content: String,
    pub timestamp: Instant,
    pub stream: OutputStream,
}
