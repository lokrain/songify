#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::TheoryError;
use mt_core::key::{Key, KeyMode};
use mt_core::pitch::PitchClass;

#[test]
fn key_from_semitone_respects_mode() {
    let major = Key::from_semitone(2, false).unwrap();
    assert_eq!(major.tonic().as_u8(), 2);
    assert_eq!(major.mode(), KeyMode::Major);

    let minor = Key::from_semitone(9, true).unwrap();
    assert_eq!(minor.tonic().as_u8(), 9);
    assert_eq!(minor.mode(), KeyMode::Minor);
}

#[test]
fn key_from_semitone_rejects_invalid_pitch_class() {
    assert_eq!(
        Key::from_semitone(12, false),
        Err(TheoryError::InvalidPitchClass(12))
    );
}

#[test]
fn key_display_is_readable() {
    let key = Key::new(PitchClass::new(0).unwrap(), KeyMode::Minor);
    assert_eq!(key.to_string(), "C min");
}
