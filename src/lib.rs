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
//! Please find additional examples in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read object temperature
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Tmp006::new(dev, address);
//! let calibration_factor = 6e-14;
//! let temperature = block!(sensor
//!     .read_object_temperature(calibration_factor))
//!     .unwrap();
//! println!("Temperature: {}K", temperature);
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut sensor = Tmp006::new(dev, address);
//! ```
//!
//! ### Read raw sensor data and calculate the object temperature manually
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//! use tmp006::{Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! let data = block!(sensor.read_sensor_data()).unwrap();
//! println!(
//!     "Object voltage: {}\nAmbient temperature: {}",
//!     data.object_voltage, data.ambient_temperature);
//! let calibration_factor = 6e-14;
//! let temp = sensor.calculate_object_temperature(data, calibration_factor);
//! println!("Temperature: {}K", temp);
//! ```
//!
//! ### Set the conversion rate to 2 per second
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! sensor.set_conversion_rate(ConversionRate::Cps2).unwrap();
//! ```
//!
//! ### Enable the DRDY (data ready) pin
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! sensor.enable_drdy_pin().unwrap();
//! ```
//!
//! ### Read whether the data is ready to be read
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp006::{ConversionRate, Tmp006, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp006::new(dev, SlaveAddr::default());
//! loop {
//!     let ready = sensor.is_data_ready().unwrap();
//!     println!("Data ready?: {}", ready);
//!     // add some delay here...
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/tmp006/0.2.0")]
#![deny(missing_docs, unsafe_code)]
#![no_std]

mod types;
use crate::types::{BitFlagsHigh, BitFlagsLow, ConfigHigh, Register, DEVICE_BASE_ADDRESS};
pub use crate::types::{ConversionRate, Error, SensorData, SlaveAddr, Tmp006};

mod config;
mod reading;
