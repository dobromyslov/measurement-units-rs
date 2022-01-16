use crate::Convertable;

/// Measurement units for percent to convert it to the decimal value and vice-versa.
#[derive(PartialEq, Debug)]
pub enum Percent {
    PERCENT,
    DECIMAL
}

impl Convertable for Percent {
    fn conversion_coefficient(&self) -> f64 {
        use Percent::*;
        match self {
            PERCENT => 0.01,
            DECIMAL => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert;
    use super::Percent;

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
}
