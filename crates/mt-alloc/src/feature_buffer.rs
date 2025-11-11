//! FeatureBuffer:
//! - Simple container for time-indexed feature vectors.
//! - Used by analysis nodes to accumulate and reuse buffers.

use alloc::vec::Vec;

/// A dense [frames x dim] feature matrix stored row-major.
#[derive(Debug, Clone)]
pub struct FeatureBuffer {
    dim: usize,
    data: Vec<f32>,
}

impl FeatureBuffer {
    /// Create buffer with given dim and capacity in frames.
    pub fn with_capacity(dim: usize, frames_capacity: usize) -> Self {
        assert!(dim > 0, "dim must be > 0");
        Self {
            dim,
            data: Vec::with_capacity(dim * frames_capacity),
        }
    }

    #[must_use]
    pub fn dim(&self) -> usize {
        self.dim
    }

    #[must_use]
    pub fn frames(&self) -> usize {
        if self.dim == 0 {
            0
        } else {
            self.data.len() / self.dim
        }
    }

    /// Push a single feature vector; len must equal dim.
    pub fn push_frame(&mut self, frame: &[f32]) {
        assert_eq!(frame.len(), self.dim, "feature dim mismatch");
        self.data.extend_from_slice(frame);
    }

    /// Get frame i as a slice.
    pub fn frame(&self, index: usize) -> Option<&[f32]> {
        let start = index.checked_mul(self.dim)?;
        let end = start + self.dim;
        if end <= self.data.len() {
            Some(&self.data[start..end])
        } else {
            None
        }
    }

    /// Clear all frames, keep capacity.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
