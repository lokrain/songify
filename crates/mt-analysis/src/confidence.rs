//! Confidence utilities for analysis outputs.
//!
//! All analyzers should:
//! - work internally in [0.0, 1.0] scores (or unbounded raw metrics),
//! - normalize to [0.0, 1.0],
//! - convert to [0, 1000] using these helpers,
//! - never write arbitrary "magic" confidence values by hand.
//!
//! This keeps confidence semantics stable across nodes and versions.

/// Max confidence on the global scale.
pub const CONFIDENCE_MAX: u16 = 1000;

/// Clamp a [0.0, 1.0] score into [0, 1000].
///
/// Rules:
/// - NaN → 0
/// - < 0 → 0
/// - > 1 → 1000
/// - linear mapping in between, with rounding.
#[must_use]
pub fn clamp01_to_confidence_x1000(score: f32) -> u16 {
    if score.is_nan() {
        return 0;
    }

    let s = if score < 0.0 {
        0.0
    } else if score > 1.0 {
        1.0
    } else {
        score
    };

    (s * CONFIDENCE_MAX as f32 + 0.5) as u16
}

/// Convert something already interpreted as a probability-like value
/// into the standard [0, 1000] scale.
///
/// Alias kept for readability when code is explicitly about probabilities.
#[must_use]
pub fn probability_to_confidence_x1000(p: f32) -> u16 {
    clamp01_to_confidence_x1000(p)
}

/// Combine multiple confidence values conservatively by taking the minimum.
///
/// Use when all conditions must hold (logical AND).
#[must_use]
pub fn confidence_and(conf_a: u16, conf_b: u16) -> u16 {
    conf_a.min(conf_b)
}

/// Combine two independent-ish confidence values via product in [0, 1].
///
/// Use when signals are treated as independent evidence.
/// Both inputs are in [0, 1000].
#[must_use]
pub fn confidence_product(conf_a: u16, conf_b: u16) -> u16 {
    let a = conf_a as f32 / CONFIDENCE_MAX as f32;
    let b = conf_b as f32 / CONFIDENCE_MAX as f32;
    clamp01_to_confidence_x1000(a * b)
}

/// Weighted average of multiple confidence values.
///
/// - `items`: slice of `(confidence_x1000, weight)`
/// - negative or NaN weights are ignored
/// - if all weights invalid → 0
///
/// Use when blending experts or overlapping windows.
#[must_use]
pub fn confidence_weighted_average(items: &[(u16, f32)]) -> u16 {
    let mut num = 0.0_f32;
    let mut den = 0.0_f32;

    for (conf, w) in items.iter().copied() {
        if !w.is_finite() || w <= 0.0 {
            continue;
        }
        let c = conf as f32 / CONFIDENCE_MAX as f32;
        num += c * w;
        den += w;
    }

    if den <= 0.0 {
        0
    } else {
        clamp01_to_confidence_x1000(num / den)
    }
}
