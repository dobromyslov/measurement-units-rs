use crate::Convertable;

#[derive(PartialEq)]
pub enum Mass {
    G,
    KG,
}

impl Convertable for Mass {
    fn conversion_coefficient(&self) -> f64 {
        use Mass::*;
        match self {
            G => 0.001,
            KG => 1.0,
        }
    }
}
