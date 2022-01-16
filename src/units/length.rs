use crate::Convertable;

/// Measurement units for length.
#[derive(PartialEq, Debug)]
pub enum Length {
    MM,
    CM,
    DM,
    M,
}

impl Convertable for Length {
    fn conversion_coefficient(&self) -> f64 {
        use Length::*;
        match self {
            MM => 0.001,
            CM => 0.01,
            DM => 0.1,
            M => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert;
    use super::Length;

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
}
