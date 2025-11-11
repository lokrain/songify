#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::TheoryError;
use mt_core::chord::Chord;
use mt_core::chord_kind::ChordKindId;
use mt_core::pitch::PitchClass;

#[test]
fn chord_allows_inversion_with_member_bass() {
    let root = PitchClass::new(0).unwrap(); // C
    let bass = PitchClass::new(4).unwrap(); // E
    let chord = Chord::new(root, ChordKindId::Maj, Some(bass));
    assert!(chord.is_ok());
}

#[test]
fn chord_rejects_non_member_bass() {
    let root = PitchClass::new(0).unwrap();
    let bass = PitchClass::new(1).unwrap();
    let err = Chord::new(root, ChordKindId::Maj, Some(bass));
    assert_eq!(err, Err(TheoryError::InvalidChord));
}
