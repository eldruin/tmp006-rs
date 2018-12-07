//! This is a platform agnostic Rust driver for the TMP006/TMP006B non-contact
//! infrared (IR) thermopile temperature sensor, based on the
//! [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See: [`enable()`].
//! - Read the object temperature. See: [`read_object_temperature()`].
//! - Read the object voltage and ambient temperature raw data. See: [`read_sensor_data()`].
//! - Calculate the object temperature from the sensor raw data. See: [`calculate_object_temperature()`].
//! - Set the ADC conversion rate. See: [`set_conversion_rate()`].
//! - Enable/disable the DRDY pin. See: [`enable_drdy_pin()`].
//! - Read whether data is ready to be read. See: [`is_data_ready()`].
//! - Perform a software reset. See: [`reset()`].
//! - Read the manufacturer ID. See: [`read_manufacturer_id()`].
//! - Read the device ID. See: [`read_device_id()`].
//!
//! [`enable()`]: struct.Tmp006.html#method.enable
//! [`read_object_temperature()`]: struct.Tmp006.html#method.read_object_temperature
//! [`read_sensor_data()`]: struct.Tmp006.html#method.read_sensor_data
//! [`calculate_object_temperature()`]: struct.Tmp006.html#method.calculate_object_temperature
//! [`set_conversion_rate()`]: struct.Tmp006.html#method.set_conversion_rate
//! [`enable_drdy_pin()`]: struct.Tmp006.html#method.enable_drdy_pin
//! [`is_data_ready()`]: struct.Tmp006.html#method.is_data_ready
//! [`reset()`]: struct.Tmp006.html#method.reset
//! [`read_manufacturer_id()`]: struct.Tmp006.html#method.read_manufacturer_id
//! [`read_device_id()`]: struct.Tmp006.html#method.read_device_id
//!
//! [Introductory blog post](https://blog.eldruin.com/tmp006-contact-less-infrared-ir-thermopile-driver-in-rust/)
//!
//! ## The device
//!
//! The TMP006 and TMP006B are the first in a series of temperature sensors
//! that measure the temperature of an object without the need to make contact
//! with the object. This sensor uses a thermopile to absorb the infrared
//! energy emitted from the object being measured and uses the corresponding
//! change in thermopile voltage to determine the object temperature.
//!
//! Infrared sensor voltage range is specified from -40°C to +125°C to enable
//! use in a wide range of applications. Low power consumption along with low
//! operating voltage makes the device suitable for battery-powered
//! applications. The low package height of the chip-scale format enables
//! standard high- volume assembly methods, and can be useful where limited
//! spacing to the object being measured is available.
//!
//! Datasheet:
//! - [TMP006/B](http://www.ti.com/ww/eu/sensampbook/tmp006.pdf)
//! User guide:
//! - [TMP006 user guide](https://cdn-shop.adafruit.com/datasheets/tmp006ug.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples in this repository: [tmp006-examples]
//!
//! [tmp006-examples]: https://github.com/eldruin/tmp006-examples
//!
//! ### Read object temperature
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! #[macro_use]
//! extern crate nb;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Tmp006::new(dev, address);
//! let calibration_factor = 6e-14;
//! let temperature = block!(sensor
//!     .read_object_temperature(calibration_factor))
//!     .unwrap();
//! println!("Temperature: {}K", temperature);
//! # }
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut sensor = Tmp006::new(dev, address);
//! # }
//! ```
//!
//! ### Read raw sensor data and calculate the object temperature manually
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! #[macro_use]
//! extern crate nb;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! let data = block!(sensor.read_sensor_data()).unwrap();
//! println!(
//!     "Object voltage: {}\nAmbient temperature: {}",
//!     data.object_voltage, data.ambient_temperature);
//! let calibration_factor = 6e-14;
//! let temp = sensor.calculate_object_temperature(data, calibration_factor);
//! println!("Temperature: {}K", temp);
//! # }
//! ```
//!
//! ### Set the conversion rate to 2 per second
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! sensor.set_conversion_rate(ConversionRate::Cps2).unwrap();
//! # }
//! ```
//!
//! ### Enable the DRDY (data ready) pin
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! sensor.enable_drdy_pin().unwrap();
//! # }
//! ```
//!
//! ### Read whether the data is ready to be read
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp006;
//!
//! use hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! loop {
//!     let ready = sensor.is_data_ready().unwrap();
//!     println!("Data ready?: {}", ready);
//!     // add some delay here...
//! }
//! # }
//! ```
#![deny(missing_docs, unsafe_code, warnings)]
#![no_std]

extern crate embedded_hal as hal;
extern crate libm;
extern crate nb;
// necessary only for targets without math function implementation
#[allow(unused_imports)]
use libm::F64Ext;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// ADC conversion rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConversionRate {
    /// 4 conversions per second
    Cps4,
    /// 2 conversions per second
    Cps2,
    /// 1 conversion per second (default)
    Cps1,
    /// 0.5 conversions per second
    Cps0_5,
    /// 0.25 conversions per second
    Cps0_25,
}

/// Data as read from the sensor.
///
/// These values can be used to calculate the object temperature as done in
/// [`read_object_temperature()`].
///
/// [`read_object_temperature()`]: struct.Tmp006.html#method.read_object_temperature
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SensorData {
    /// Object voltage: `[-32768..32767]`
    pub object_voltage: i16,
    /// Ambient temperature: `[-8192..8191]`
    pub ambient_temperature: i16,
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    ///
    /// Some of these combinations require connecting the ADDR0 pin to
    /// SCL or SDA. Check table 1 on page 7 of the datasheet: [TMP006/B].
    Alternative(bool, bool, bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a2, a1, a0) => {
                default
                    | ((a2 as u8) << 2)
                    | ((a1 as u8) << 1)
                    | a0 as u8
        }
    }
}
}

const DEVICE_BASE_ADDRESS: u8 = 0b100_0000;

struct Register;
impl Register {
    const V_OBJECT     : u8 = 0x00;
    const TEMP_AMBIENT : u8 = 0x01;
    const CONFIG       : u8 = 0x02;
    const MANUFAC_ID   : u8 = 0xFE;
    const DEVICE_ID    : u8 = 0xFE;
}


struct BitFlagsHigh;
impl BitFlagsHigh {
    const SW_RESET : u8 = 0b1000_0000;
    const MOD      : u8 = 0b0111_0000;
    const CR2      : u8 = 0b0000_1000;
    const CR1      : u8 = 0b0000_0100;
    const CR0      : u8 = 0b0000_0010;
    const DRDY_EN  : u8 = 0b0000_0001;
}
struct BitFlagsLow;
impl BitFlagsLow {
    const DRDY     : u8 = 0b1000_0000;
}

#[derive(Debug, Clone, Copy)]
struct ConfigHigh {
    bits: u8,
}

/// TMP006 device driver.
#[derive(Debug, Default)]
pub struct Tmp006<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: ConfigHigh,
}

mod config;
mod reading;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_0000, SlaveAddr::Alternative(false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0001, SlaveAddr::Alternative(false, false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0010, SlaveAddr::Alternative(false,  true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0100, SlaveAddr::Alternative( true, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0111, SlaveAddr::Alternative( true,  true,  true).addr(DEVICE_BASE_ADDRESS));
    }
}
