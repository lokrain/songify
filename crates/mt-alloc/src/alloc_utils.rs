//! Small helpers over `alloc` types.
//!
//! No policy, just utilities reused internally.

use alloc::vec::Vec;

/// Create a zeroed Vec<T> of given length.
/// Only for `Copy + Default` types; deterministic initialization.
pub fn zeroed_vec<T>(len: usize) -> Vec<T>
where
    T: Copy + Default,
{
    vec![T::default(); len]
}
