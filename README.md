# Rust TMP006/B Non-Contact Infrared (IR) Thermopile Temperature Sensor Driver

[![crates.io](https://img.shields.io/crates/v/tmp006.svg)](https://crates.io/crates/tmp006)
[![Docs](https://docs.rs/tmp006/badge.svg)](https://docs.rs/tmp006)
[![Build Status](https://travis-ci.org/eldruin/tmp006-rs.svg?branch=master)](https://travis-ci.org/eldruin/tmp006-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/tmp006-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/tmp006-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the TMP006/TMP006B non-contact
infrared (IR) thermopile temperature sensor, based on the
[`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device.
- Read the object temperature.
- Read the object voltage.
- Read the ambient temperature.
- Perform a software reset.
- Read the manufacturer ID.
- Read the device ID.

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
- [TMP006/B](http://www.ti.com/ww/eu/sensampbook/tmp006.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

