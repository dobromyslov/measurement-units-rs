use crate::Convertable;

#[derive(PartialEq)]
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
