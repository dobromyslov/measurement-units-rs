use crate::Convertable;

#[derive(PartialEq)]
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
