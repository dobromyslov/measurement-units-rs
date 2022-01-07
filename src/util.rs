pub fn round_to_precision(value: &f64, precision: &u32) -> f64 {
    let multiplier= 10u32.pow(*precision) as f64;
    return (value * multiplier).round() / multiplier;
}

#[cfg(test)]
mod tests {
    #[test]
    fn round_to_precision() {
        assert_eq!(
            super::round_to_precision(&0.123456789, &2),
            0.12
        );
        assert_eq!(
            super::round_to_precision(&0.123456789, &0),
            0.0
        );
    }
}
