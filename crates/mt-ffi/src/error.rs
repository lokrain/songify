//! FFI status codes.
//!
//! These codes are stable C ABI and must not be renumbered.

/// Status codes returned by all mt-ffi functions.
///
/// 0 = success, non-zero = error.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MtFfiStatus {
    /// Operation successful.
    MtFfiOk = 0,
    /// One or more null pointers where non-null was required.
    MtFfiErrorNull = 1,
    /// Invalid arguments (sizes, ranges, etc.).
    MtFfiErrorInvalidArg = 2,
    /// Underlying engine error (configuration, runtime, etc.).
    MtFfiErrorEngine = 3,
    /// A Rust panic was caught inside the FFI boundary.
    MtFfiErrorPanic = 4,
}

impl MtFfiStatus {
    #[inline]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::MtFfiOk)
    }
}
