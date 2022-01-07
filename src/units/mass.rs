use crate::Convertable;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::convert;
    use super::Mass;

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
}
