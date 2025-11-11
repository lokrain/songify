//! Shared signal-level traits.
//!
//! These traits define how other crates (mt-analysis, mt-engine) talk to
//! generic samples, frames, and spectra without depending on concrete backends.

/// Scalar sample type.
///
/// Implemented for primitive PCM-like types only. No heap, no dynamic dispatch.
pub trait Sample:
    Copy + Clone + PartialEq + Send + Sync + 'static
{
    /// Represent this sample as f32 in [-1.0, 1.0] or a documented range.
    fn to_f32(self) -> f32;

    /// Construct from f32, with deterministic clamping/rounding.
    fn from_f32(x: f32) -> Self;

    /// Additive identity.
    fn zero() -> Self;

    /// Multiplicative scaling.
    fn scale(self, gain: f32) -> Self {
        Self::from_f32(self.to_f32() * gain)
    }
}
