#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::TheoryError;
use mt_core::pitch::{
    Accidental, Letter, MidiNote, PITCH_CLASS_COUNT, PitchClass, SpelledPitchClass,
};

#[test]
fn pitch_class_bounds() {
    for value in 0..PITCH_CLASS_COUNT {
        assert!(PitchClass::new(value).is_ok());
    }

    assert_eq!(
        PitchClass::new(PITCH_CLASS_COUNT),
        Err(TheoryError::InvalidPitchClass(PITCH_CLASS_COUNT))
    );
}

#[test]
fn pitch_class_transpose_wraps() {
    let c = PitchClass::new(0).unwrap();
    let transposed = c.transpose(14); // 14 semitones == major ninth
    assert_eq!(transposed.as_u8(), 2);
}

#[test]
fn spelled_pitch_class_maps_correctly() {
    let sharp = SpelledPitchClass::new(Letter::F, Accidental::Sharp);
    assert_eq!(sharp.to_pitch_class().as_u8(), 6);

    let double_flat = SpelledPitchClass::new(Letter::A, Accidental::DoubleFlat);
    assert_eq!(double_flat.to_pitch_class().as_u8(), 7);
}

#[test]
fn midi_note_round_trip() {
    let note = MidiNote::new(60).unwrap();
    assert_eq!(note.value(), 60);
    assert_eq!(note.pitch_class().as_u8(), 0);
    assert_eq!(note.octave(), 4);

    assert_eq!(MidiNote::new(128), Err(TheoryError::InvalidMidiNote(128)));
}
