//! Timeline events used across analysis and engine layers.
//!
//! These types are POD-like, deterministic, and reference mt-core primitives.

pub mod chord_event;
pub mod key_event;
pub mod meter;
pub mod note;
pub mod segment;
pub mod swing;
pub mod tempo;

pub use chord_event::ChordEvent;
pub use key_event::KeyEvent;
pub use meter::MeterEvent;
pub use note::{NoteEvent, NoteId, TrackId};
pub use segment::{SegmentEvent, SegmentKind};
pub use swing::SwingEvent;
pub use tempo::TempoEvent;
