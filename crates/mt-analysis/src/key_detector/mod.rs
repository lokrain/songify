//! Key detection façade.

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::vec::Vec;

use crate::config::KeyConfig;
use crate::traits::KeyAnalyzer;
use mtcore::events::{KeyEvent, NoteEvent};
use mt_core::key::{Key, KeyMode};
use mt_core::pitch::PitchClass;
use mt_core::time::SampleTime;

mod histogram;

pub use histogram::HistogramKeyAnalyzer;

/// A simple histogram-based implementation is provided by `HistogramKeyAnalyzer`.
