//! ABI versioning.
//!
//! Bump `mt-ABI_VERSION` when the C ABI changes in any incompatible way.
//! Keep this independent from crate/package semver.

/// Current ABI version for mt-ffi.
///
/// This is a monotonic integer. Any breaking change to function
/// signatures or `#[repr(C)]` types must bump this.
pub const mt-ABI_VERSION: u32 = 1;

/// Returns the ABI version at runtime for C callers.
#[no_mangle]
pub extern "C" fn mt-ffi_get_abi_version() -> u32 {
    mt-ABI_VERSION
}
