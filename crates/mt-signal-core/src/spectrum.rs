#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

impl Complex32 {
    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn mag_sq(self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    pub fn mag(self) -> f32 {
        self.mag_sq().sqrt()
    }
}

pub trait SpectrumView {
    fn bins(&self) -> &[Complex32];
    fn sample_rate_hz(&self) -> u32;

    fn bin_freq_hz(&self, bin: usize) -> f32 {
        let sr = self.sample_rate_hz() as f32;
        let n = self.bins().len() as f32;
        if n <= 0.0 {
            0.0
        } else {
            (bin as f32 * sr) / n
        }
    }
}
