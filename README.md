# measurement-units
Rust library for basic measurements units conversion such as length, mass, time, volume, percents.

## Install
```toml
# Cargo.toml
[dependencies]
measurement-units = "0.1.1"
```

## Usage

```rust
use measurement_units::{convert, Length};
let length_in_centimeters = convert(&1.0, &Length::M, &Length::CM, &Option::None);
assert_eq!(100.0, length_in_centimeters);

let length_in_meters = convert(&99.5, &Length::CM, &Length::M, &Some(0_u32));
assert_eq!(1.0, length_in_meters);
```

## License

MIT (c) 2022 Viacheslav Dobromyslov <<viacheslav@dobromyslov.ru>>
