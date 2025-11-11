//! Sample implementations and helpers.
//!
//! Only primitive scalar types are supported as `Sample`.
//! All conversions are deterministic and clamped.
use crate::traits::Sample;

/// Implementation for f32 in [-1.0, 1.0] (not enforced at type level).
impl Sample for f32 {
    #[inline]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        x
    }

    #[inline]
    fn zero() -> Self {
        0.0
    }
}

/// Implementation for f64. Mapped to/from f32 deterministically.
impl Sample for f64 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        x as f64
    }

    #[inline]
    fn zero() -> Self {
        0.0
    }
}

/// Implementation for 16-bit PCM: i16.
///
/// Range mapping:
/// -128..=127 inclusive mapped symmetrically via i16::MAX.
impl Sample for i16 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32 / i16::MAX as f32
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        let scaled = x * i16::MAX as f32;
        clamp_i32(scaled as i32, i16::MIN as i32, i16::MAX as i32) as i16
    }

    #[inline]
    fn zero() -> Self {
        0
    }
}

/// Implementation for 24-bit-style stored in i32 (caller defines packing).
impl Sample for i32 {
    #[inline]
    fn to_f32(self) -> f32 {
        // Treat as 24-bit PCM in 32-bit container: use 23rd bit as max magnitude.
        let max = (1_i32 << 23) - 1;
        self as f32 / max as f32
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        let max = (1_i32 << 23) - 1;
        let scaled = x * max as f32;
        clamp_i32(scaled as i32, -max, max)
    }

    #[inline]
    fn zero() -> Self {
        0
    }
}

#[inline]
const fn clamp_i32(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

/// Utility: view any `Sample` slice as f32 slice without allocation.
///
/// This is lossy for integer types but deterministic.
#[must_use]
pub fn as_f32_slice<S: Sample>(input: &[S], out: &mut [f32]) -> usize {
    let n = core::cmp::min(input.len(), out.len());
    let mut i = 0;
    while i < n {
        out[i] = input[i].to_f32();
        i += 1;
    }
    n
}

/// Read-only view over a complex spectrum (e.g. result of an FFT).
///
/// Implementations are provided in higher crates; this crate only defines the contract.
pub trait SpectrumView {
    /// Complex bins (DC..Nyquist or as defined by implementation).
    fn bins(&self) -> &[crate::spectrum::Complex32];

    /// Sample rate associated with this spectrum.
    fn sample_rate_hz(&self) -> u32;

    /// Bin index → frequency in Hz using standard mapping.
    fn bin_freq_hz(&self, bin: usize) -> f32 {
        let sr = self.sample_rate_hz() as f32;
        let n = self.bins().len() as f32;
        if n <= 0.0 {
            return 0.0;
        }
        (bin as f32 * sr) / n
    }
}
