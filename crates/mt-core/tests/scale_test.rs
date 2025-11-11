#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::pitch::PitchClass;
use mt_core::scale::{ScaleId, build_scale};

#[test]
fn major_scale_contains_expected_pitch_classes() {
    let tonic = PitchClass::new(0).unwrap();
    let (len, values) = build_scale(tonic, ScaleId::Major).expect("major scale should exist");
    assert_eq!(len, 7);
    let expected = [0u8, 2, 4, 5, 7, 9, 11];
    for (idx, value) in expected.iter().enumerate() {
        assert_eq!(values[idx].as_u8(), *value);
    }
}

#[test]
fn chromatic_scale_has_all_pitch_classes() {
    let tonic = PitchClass::new(1).unwrap();
    let (len, values) =
        build_scale(tonic, ScaleId::Chromatic).expect("chromatic scale should exist");
    assert_eq!(len, 12);
    let mut seen = [false; 12];
    for idx in 0..len {
        seen[values[idx].as_u8() as usize] = true;
    }
    assert!(seen.iter().all(|v| *v));
}
