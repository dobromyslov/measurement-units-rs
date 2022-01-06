use crate::convert;
use crate::units::length::Length;
use crate::units::mass::Mass;
use crate::units::percent::Percent;
use crate::units::time::Time;
use crate::units::volume::Volume;

#[test]
fn convert_length() {
    assert_eq!(
      convert(&1.0, &Length::M, &Length::MM, &None),
      1000.0
    );
    assert_eq!(
      convert(&1000.0, &Length::MM, &Length::M, &None),
      1.0
    );
    assert_eq!(
      convert(&1.0, &Length::DM, &Length::M, &None),
      0.1
    );
    assert_eq!(
      convert(&1.0, &Length::CM, &Length::M, &None),
      0.01
    );
}

#[test]
fn convert_mass() {
    assert_eq!(
        convert(&1.0, &Mass::KG, &Mass::G, &None),
        1000.0
    );
    assert_eq!(
        convert(&1.0, &Mass::G, &Mass::KG, &None),
        0.001
    );
}

#[test]
fn convert_time() {
    assert_eq!(
        convert(&1.0, &Time::S, &Time::M, &None),
        1.0 / 60.0
    );
    assert_eq!(
        convert(&1.0, &Time::S, &Time::H, &None),
        1.0 / 360.0
    );
    assert_eq!(
        convert(&1.0, &Time::H, &Time::S, &None),
        360.0
    );
}

#[test]
fn convert_volume() {
    assert_eq!(
        convert(&1.0, &Volume::L, &Volume::M3, &None),
        0.001
    );
    assert_eq!(
        convert(&1.0, &Volume::M3, &Volume::L, &None),
        1000.0
    );
}

#[test]
fn convert_percent() {
    assert_eq!(
        convert(&1.0, &Percent::PERCENT, &Percent::DECIMAL, &None),
        0.01
    );
    assert_eq!(
        convert(&0.5, &Percent::DECIMAL, &Percent::PERCENT, &None),
        50.0
    );
}

#[test]
fn round_to_precision() {
    assert_eq!(
        crate::round_to_precision(&0.123456789, &2),
        0.12
    );
}
