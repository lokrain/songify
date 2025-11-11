//! Chord detection façade.

pub mod rule_based;
pub mod template_matching; // can reuse rule-based or provide alt strategy

pub use rule_based::RuleBasedChordAnalyzer;
