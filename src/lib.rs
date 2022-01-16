//! # measurement_units
//! `measurement_units` is a rust library for basic measurements units conversion such as length,
//! mass, time, volume, percents.
//!
//! # Examples
//! ```
//! use measurement_units::{convert, Length};
//! let length_in_centimeters = convert(&1.0, &Length::M, &Length::CM, &Option::None);
//! assert_eq!(100.0, length_in_centimeters);
//!
//! let length_in_meters = convert(&99.5, &Length::CM, &Length::M, &Some(0_u32));
//! assert_eq!(1.0, length_in_meters);
//! ```

mod util;
mod units;

pub use units::length::Length;
pub use units::mass::Mass;
pub use units::time::Time;
pub use units::volume::Volume;
pub use units::percent::Percent;

/// Main trait implements list of conversion coefficients for measurement units.
pub trait Convertable {
    fn conversion_coefficient(&self) -> f64;
}

/// Converts measurement unit from one to another using coefficients specified by the Convertable
/// trait in the conversion_coefficient method.
/// Precision argument defines number of decimal digits to roundup the result.
///
/// # Examples
/// ```
/// use measurement_units::{convert, Length};
/// let length_in_centimeters = convert(&1.0, &Length::M, &Length::CM, &Option::None);
/// assert_eq!(100.0, length_in_centimeters);
///
/// let length_in_meters = convert(&99.5, &Length::CM, &Length::M, &Some(0_u32));
/// assert_eq!(1.0, length_in_meters);
/// ```
pub fn convert<T>(value: &f64, from: &T, to: &T, precision: &Option<u32>) -> f64
where T: Convertable + PartialEq
{
    let mut result = *value;
    if from != to {
        result *= from.conversion_coefficient() / to.conversion_coefficient();
    }

    match precision {
        None => result,
        Some(precision) => util::round_to_precision(&result, precision)
    }
}
