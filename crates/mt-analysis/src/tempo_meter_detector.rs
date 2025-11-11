//! Deterministic tempo estimator using simple energy autocorrelation.
//!
//! This is offline, mono-signal oriented, and intentionally simple.
//! Good enough as a baseline; can be swapped with more advanced logic
//! without changing the trait.
extern crate alloc;

use crate::config::TempoConfig;
use crate::traits::TempoMeterAnalyzer;
use mt_core::events::TempoEvent;
use mt_core::time::SampleTime;

pub struct SimpleTempoMeterAnalyzer;

impl TempoMeterAnalyzer for SimpleTempoMeterAnalyzer {
    fn detect_tempo(
        &self,
        samples: &[f32],
        sample_rate: u32,
        cfg: &TempoConfig,
    ) -> Vec<TempoEvent> {
        let envelope = compute_envelope(samples, cfg.frame_size, cfg.hop_size);
        if envelope.is_empty() {
            return Vec::new();
        }

        let bpm = estimate_bpm_from_envelope(
            &envelope,
            sample_rate,
            cfg.tempo_range.min_bpm,
            cfg.tempo_range.max_bpm,
        );

        let bpm_x1000 = (bpm * 1000.0 + 0.5) as u32;

        vec![TempoEvent { position: SampleTime::ZERO, bpm_x1000 }]
    }
}

/// Root-mean-square envelope.
fn compute_envelope(samples: &[f32], frame: usize, hop: usize) -> Vec<f32> {
    let mut out = Vec::new();
    if frame == 0 || hop == 0 || samples.len() < frame {
        return out;
    }
    let mut i = 0;
    while i + frame <= samples.len() {
        let mut sum = 0.0;
        for &s in &samples[i..i + frame] {
            sum += s * s;
        }
        let rms = (sum / frame as f32).sqrt();
        out.push(rms);
        i += hop;
    }
    out
}

/// Naive autocorrelation-based tempo estimate in [min_bpm, max_bpm].
fn estimate_bpm_from_envelope(env: &[f32], sample_rate: u32, min_bpm: f32, max_bpm: f32) -> f32 {
    // Envelope hop rate:
    let hop_rate = sample_rate as f32;
    let min_period = (60.0 / max_bpm) * hop_rate;
    let max_period = (60.0 / min_bpm) * hop_rate;

    let min_lag = min_period.max(1.0) as usize;
    let max_lag = max_period.min((env.len() as f32 - 1.0).max(1.0)) as usize;

    let mut best_lag = min_lag;
    let mut best_score = 0.0;

    let n = env.len();
    let mut lag = min_lag;
    while lag <= max_lag {
        let mut sum = 0.0;
        let mut i = 0;
        while i + lag < n {
            sum += env[i] * env[i + lag];
            i += 1;
        }
        if sum > best_score {
            best_score = sum;
            best_lag = lag;
        }
        lag += 1;
    }

    if best_lag == 0 { 120.0 } else { 60.0 * hop_rate / best_lag as f32 }
}
