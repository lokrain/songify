use crate::sample::Sample;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame<S: Sample, const N: usize> {
    pub channels: [S; N],
}

impl<S: Sample, const N: usize> Frame<S, N> {
    pub const fn new(channels: [S; N]) -> Self {
        Self { channels }
    }

    pub fn zero() -> Self {
        Self {
            channels: [S::zero(); N],
        }
    }

    pub const fn channels(&self) -> usize {
        N
    }

    pub fn get(&self, idx: usize) -> Option<&S> {
        self.channels.get(idx)
    }

    pub fn scaled(self, gain: f32) -> Self {
        let mut out = self.channels;
        let mut i = 0;
        while i < N {
            out[i] = out[i].scale(gain);
            i += 1;
        }
        Self { channels: out }
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(S) -> S,
    {
        let mut out = self.channels;
        let mut i = 0;
        while i < N {
            out[i] = f(out[i]);
            i += 1;
        }
        Self { channels: out }
    }
}

pub type MonoFrame<S> = Frame<S, 1>;
pub type StereoFrame<S> = Frame<S, 2>;
