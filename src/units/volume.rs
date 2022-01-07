use crate::Convertable;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::convert;
    use super::Volume;

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
}
