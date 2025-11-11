#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::TheoryError;
use mt_core::midi::{MidiChannel, MidiEvent, MidiEventKind};
use mt_core::pitch::MidiNote;

#[test]
fn midi_channel_bounds() {
    assert!(MidiChannel::new(0).is_ok());
    assert!(MidiChannel::new(15).is_ok());
    assert_eq!(
        MidiChannel::new(16),
        Err(TheoryError::InvalidMidiChannel(16))
    );
}

#[test]
fn midi_note_on_and_off_builders() {
    let channel = MidiChannel::new(2).unwrap();
    let note = MidiNote::new(64).unwrap();

    let on = MidiEvent::note_on(channel, note, 100);
    assert_eq!(on.channel.value(), 2);
    assert_eq!(on.kind, MidiEventKind::NoteOn);
    assert_eq!(on.data1, 64);
    assert_eq!(on.data2, 100);

    let off = MidiEvent::note_off(channel, note, 64);
    assert_eq!(off.kind, MidiEventKind::NoteOff);
    assert_eq!(off.data2, 64);
}
