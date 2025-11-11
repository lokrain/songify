//! Template-based chord detector.
//!
//! For v1 this delegates to `RuleBasedChordAnalyzer`.
//! Kept separate so future implementations can diverge without API changes.

use std::vec::Vec;

use crate::config::ChordConfig;
use crate::chord_detector::rule_based::RuleBasedChordAnalyzer;
use crate::traits::ChordAnalyzer;
use mt_core::events::{ChordEvent, NoteEvent};

pub struct TemplateMatchingChordAnalyzer;

impl ChordAnalyzer for TemplateMatchingChordAnalyzer {
    fn detect_chords(&self, notes: &[NoteEvent], cfg: &ChordConfig) -> Vec<ChordEvent> {
        RuleBasedChordAnalyzer.detect_chords(notes, cfg)
    }
}
