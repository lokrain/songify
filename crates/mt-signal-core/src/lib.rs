//! mt-signal-core
//!
//! Low-level, deterministic signal primitives used by mt-analysis and mt-engine.
//!
//! Contains:
//! - Sample traits and conversions
//! - Fixed-size frames for mono/stereo/N-channel audio
//! - Window functions (Hann, Hamming, Blackman, Rectangular)
//! - Minimal complex and spectrum views
//! - Traits describing STFT/spectrum contracts (no implementations)
//!
//! Constraints:
//! - `no_std` capable.
//! - No heap allocations.
//! - No I/O, logging, or randomness.
//! - No FFT implementation: higher layers or external crates plug into traits.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

#[cfg(feature = "std")]
extern crate std;

pub mod traits;
pub mod sample;
pub mod frame;
pub mod window;
pub mod spectrum;

pub use crate::{
    frame::{Frame, MonoFrame, StereoFrame},
    sample::{as_f32_slice, Sample},
    spectrum::{Complex32, SpectrumView},
    window::{fill_window, window_value, WindowKind},
};
