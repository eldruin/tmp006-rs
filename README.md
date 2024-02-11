# Rust TMP006/B Non-Contact Infrared (IR) Thermopile Temperature Sensor Driver

[![crates.io](https://img.shields.io/crates/v/tmp006.svg)](https://crates.io/crates/tmp006)
[![Docs](https://docs.rs/tmp006/badge.svg)](https://docs.rs/tmp006)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/tmp006-rs/workflows/Build/badge.svg)](https://github.com/eldruin/tmp006-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/tmp006-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/tmp006-rs?branch=master)

This is a platform agnostic Rust driver for the TMP006/TMP006B non-contact
infrared (IR) thermopile temperature sensor, based on the
[`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Read the object temperature. See: `read_object_temperature()`.
- Read the object voltage and ambient temperature raw data. See: `read_sensor_data()`.
- Calculate the object temperature from the sensor raw data. See: `calculate_object_temperature()`.
- Set the ADC conversion rate. See: `set_conversion_rate()`.
- Enable/disable the DRDY pin. See: `enable_drdy_pin()`.
- Read whether data is ready to be read. See: `is_data_ready()`.
- Perform a software reset. See: `reset()`.
- Read the manufacturer ID. See: `read_manufacturer_id()`.
- Read the device ID. See: `read_device_id()`.

[Introductory blog post](https://blog.eldruin.com/tmp006-contact-less-infrared-ir-thermopile-driver-in-rust/)

## The device

The TMP006 and TMP006B are the first in a series of temperature sensors
that measure the temperature of an object without the need to make contact
with the object. This sensor uses a thermopile to absorb the infrared
energy emitted from the object being measured and uses the corresponding
change in thermopile voltage to determine the object temperature.

Infrared sensor voltage range is specified from -40°C to +125°C to enable
use in a wide range of applications. Low power consumption along with low
operating voltage makes the device suitable for battery-powered
applications. The low package height of the chip-scale format enables
standard high- volume assembly methods, and can be useful where limited
spacing to the object being measured is available.

Datasheet:
- [TMP006/B](https://media.digikey.com/pdf/Data%20Sheets/Texas%20Instruments%20PDFs/TMP006(B).pdf)
User guide:
- [TMP006 user guide](https://cdn-shop.adafruit.com/datasheets/tmp006ug.pdf)

## Usage example

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use nb::block;
use tmp006::{SlaveAddr, Tmp006};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Tmp006::new(dev, address);
    let calibration_factor = 6e-14;
    let temperature = block!(sensor
        .read_object_temperature(calibration_factor))
        .unwrap();
    println!("Temperature: {}K", temperature);
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/tmp006-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
