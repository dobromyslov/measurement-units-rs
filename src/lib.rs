pub mod units;

pub trait Convertable {
    fn conversion_coefficient(&self) -> f64;
}

pub fn convert<T>(value: &f64, from: &T, to: &T, precision: &Option<u32>) -> f64
where T: Convertable + PartialEq
{
    let mut result = *value;
    if from != to {
        result *= from.conversion_coefficient() / to.conversion_coefficient();
    }

    match precision {
        None => result,
        Some(precision) => round_to_precision(&result, precision)
    }
}

fn round_to_precision(value: &f64, precision: &u32) -> f64 {
    let multiplier= 10u32.pow(*precision) as f64;
    return (value * multiplier).round() / multiplier;
}

#[cfg(test)]
mod tests;
