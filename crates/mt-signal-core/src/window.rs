use core::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowKind {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
}

pub fn window_value(kind: WindowKind, n: usize, len: usize) -> f32 {
    if len == 0 {
        return 0.0;
    }
    if len == 1 {
        return 1.0;
    }

    let x = n as f32;
    let l = (len - 1) as f32;

    match kind {
        WindowKind::Rectangular => 1.0,
        WindowKind::Hann => 0.5 * (1.0 - (2.0 * PI * x / l).cos()),
        WindowKind::Hamming => 0.54 - 0.46 * (2.0 * PI * x / l).cos(),
        WindowKind::Blackman => {
            let a0 = 0.42_f32;
            let a1 = 0.5_f32;
            let a2 = 0.08_f32;
            let arg = 2.0 * PI * x / l;
            a0 - a1 * arg.cos() + a2 * (2.0 * arg).cos()
        }
    }
}

pub fn fill_window(kind: WindowKind, buf: &mut [f32]) {
    let len = buf.len();
    let mut n = 0;
    while n < len {
        buf[n] = window_value(kind, n, len);
        n += 1;
    }
}
