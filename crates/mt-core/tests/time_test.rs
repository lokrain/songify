#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use mt_core::TheoryError;
use mt_core::time::{MusicalPosition, SampleTime};

#[test]
fn sample_time_saturating_math() {
    let base = SampleTime::new(1_000);
    assert_eq!(base.saturating_add(500).value(), 1_500);
    assert_eq!(base.saturating_sub(2_000).value(), -1_000);
}

#[test]
fn musical_position_requires_positive_bar() {
    assert_eq!(MusicalPosition::new(0, 1, 0), Err(TheoryError::InvalidTime));
}

#[test]
fn musical_position_orders_lexicographically() {
    let mut positions = vec![
        MusicalPosition::new(3, 1, 0).unwrap(),
        MusicalPosition::new(1, 4, 0).unwrap(),
        MusicalPosition::new(1, 2, 480).unwrap(),
        MusicalPosition::new(1, 2, 120).unwrap(),
    ];

    positions.sort();

    let expected = vec![
        MusicalPosition::new(1, 2, 120).unwrap(),
        MusicalPosition::new(1, 2, 480).unwrap(),
        MusicalPosition::new(1, 4, 0).unwrap(),
        MusicalPosition::new(3, 1, 0).unwrap(),
    ];

    assert_eq!(positions, expected);
}
