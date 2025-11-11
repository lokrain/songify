#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::chord::Chord;
use mt_core::chord_kind::ChordKindId;
use mt_core::events::{
    ChordEvent, KeyEvent, MeterEvent, NoteEvent, NoteId, SegmentEvent, SegmentKind, SwingEvent,
    TempoEvent, TrackId,
};
use mt_core::key::{Key, KeyMode};
use mt_core::pitch::{MidiNote, PitchClass};
use mt_core::time::SampleTime;
use mt_core::traits::{HasConfidence, HasPosition};

#[test]
fn tempo_and_meter_events_report_positions() {
    let tempo = TempoEvent {
        position: SampleTime::new(4800),
        bpm_x1000: 120_000,
    };
    let meter = MeterEvent {
        position: SampleTime::new(9600),
        numerator: 3,
        denominator: 4,
    };

    assert_eq!(tempo.position(), SampleTime::new(4800));
    assert_eq!(meter.position(), SampleTime::new(9600));
}

#[test]
fn note_event_uses_onset_for_position() {
    let note = NoteEvent {
        id: NoteId(42),
        track: TrackId(3),
        onset: SampleTime::new(1_920),
        offset: SampleTime::new(2_880),
        note: MidiNote::new(67).unwrap(),
        velocity: 96,
    };

    assert_eq!(note.position(), SampleTime::new(1_920));
}

#[test]
fn chord_event_reports_confidence_and_position() {
    let chord = Chord::new(PitchClass::new(0).unwrap(), ChordKindId::Maj7, None).unwrap();

    let event = ChordEvent {
        chord,
        onset: SampleTime::new(4_800),
        offset: SampleTime::new(9_600),
        confidence_x1000: 875,
    };

    assert_eq!(event.position(), SampleTime::new(4_800));
    assert_eq!(event.confidence_x1000(), 875);
}

#[test]
fn key_and_segment_events_forward_confidence() {
    let key_event = KeyEvent {
        key: Key::new(PitchClass::new(7).unwrap(), KeyMode::Minor),
        position: SampleTime::new(12_000),
        confidence_x1000: 920,
    };

    let segment_event = SegmentEvent {
        kind: SegmentKind::Chorus,
        onset: SampleTime::new(16_000),
        offset: SampleTime::new(24_000),
        confidence_x1000: 640,
    };

    let swing = SwingEvent {
        position: SampleTime::new(18_000),
        ratio_x1000: 666,
    };

    assert_eq!(key_event.confidence_x1000(), 920);
    assert_eq!(key_event.position(), SampleTime::new(12_000));
    assert_eq!(segment_event.confidence_x1000(), 640);
    assert_eq!(segment_event.position(), SampleTime::new(16_000));
    assert_eq!(swing.position(), SampleTime::new(18_000));
}
