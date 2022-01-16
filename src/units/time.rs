use crate::Convertable;

/// Measurement units for time.
#[derive(PartialEq, Debug)]
pub enum Time {
    S,
    M,
    H,
}

impl Convertable for Time {
    fn conversion_coefficient(&self) -> f64 {
        use Time::*;
        match self {
            S => 1.0,
            M => 60.0,
            H => 360.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert;
    use super::Time;

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
}
