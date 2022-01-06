use crate::Convertable;

#[derive(PartialEq)]
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
