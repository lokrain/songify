//! mt_alloc
//!
//! Heap-based primitives built on `mt_core`:
//! - Tempo map over `TempoEvent` / `MeterEvent` / `SwingEvent`.
//! - MIDI normalizer → `NoteEvent`s.
//! - NoteStore for efficient queries.
//! - EventRing: generic ring buffer for batch-style events.
//! - SpscQueue: single-producer, single-consumer queue for RT↔non-RT (opt-in).
//! - FeatureBuffer: buffers for numeric features.
//!
//! Design:
//! - Deterministic.
//! - No I/O, no randomness.
//! - All concurrency semantics explicit, no hidden threads.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

extern crate alloc;

pub mod alloc_utils;
pub mod event_ring;
pub mod feature_buffer;
pub mod midi_normalizer;
pub mod note_store;
pub mod spsc_queue;
pub mod tempo_map;

pub use event_ring::EventRing;
pub use feature_buffer::FeatureBuffer;
pub use midi_normalizer::MidiNormalizer;
pub use note_store::NoteStore;
pub use spsc_queue::SpscQueue;
pub use tempo_map::TempoMap;
