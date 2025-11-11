//! Optional logging hooks.
//!
//! No external dependency; callers can wrap these as needed.

use crate::types::EngineError;

/// Log a non-fatal engine warning.
///
/// v1: no-op by default.
#[inline]
pub fn warn(_msg: &str) {
    // Intentionally empty; integrate with `log` crate in outer layers.
}

/// Log an engine error.
///
/// v1: no-op; errors are returned via Result.
#[inline]
pub fn log_error(_err: &EngineError) {
    // Intentionally empty.
}
