use crate::Convertable;

#[derive(PartialEq)]
pub enum Volume {
    L,
    M3,
}

impl Convertable for Volume {
    fn conversion_coefficient(&self) -> f64 {
        use Volume::*;
        match self {
            L => 0.001,
            M3 => 1.0,
        }
    }
}
