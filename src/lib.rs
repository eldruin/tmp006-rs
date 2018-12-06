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
//! [`set_conversion_rate()`]: struct.Tmp006.html#method.set_conversion_rate
//! [`enable_drdy_pin()`]: struct.Tmp006.html#method.enable_drdy_pin
//! [`is_data_ready()`]: struct.Tmp006.html#method.is_data_ready
//! [`reset()`]: struct.Tmp006.html#method.reset
//! [`read_manufacturer_id()`]: struct.Tmp006.html#method.read_manufacturer_id
//! [`read_device_id()`]: struct.Tmp006.html#method.read_device_id
//!
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
//! ### Read raw sensor data
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
use hal::blocking::i2c;
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

impl ConfigHigh {
    fn with_high(self, mask: u8) -> Self {
        ConfigHigh {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        ConfigHigh {
            bits: self.bits & !mask,
        }
    }
}

impl Default for ConfigHigh {
    fn default() -> Self {
        ConfigHigh { bits: 0 }
            .with_high(BitFlagsHigh::MOD)
            .with_high(BitFlagsHigh::CR1)
    }
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

impl<I2C, E> Tmp006<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the TMP006 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp006 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: ConfigHigh::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the sensor (default state).
    ///
    /// Sensor and ambient continuous conversion.
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlagsHigh::MOD))
    }

    /// Disable the sensor (power-down).
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_low(BitFlagsHigh::MOD))
    }

    /// Reset the sensor (software reset).
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlagsHigh::SW_RESET))?;
        self.config = ConfigHigh::default();
        Ok(())
    }

    /// Enable DRDY pin.
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn enable_drdy_pin(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlagsHigh::DRDY_EN))
    }

    /// Disable DRDY pin.
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn disable_drdy_pin(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_low(BitFlagsHigh::DRDY_EN))
    }

    /// Set the ADC conversion rate.
    ///
    /// Note: calling this clears the data-ready bit.
    pub fn set_conversion_rate(&mut self, rate: ConversionRate) -> Result<(), Error<E>> {
        use BitFlagsHigh as BF;
        use ConversionRate as CR;
        let config;
        match rate {
            CR::Cps4    => config = self.config.with_low( BF::CR2).with_low( BF::CR1).with_low( BF::CR0),
            CR::Cps2    => config = self.config.with_low( BF::CR2).with_low( BF::CR1).with_high(BF::CR0),
            CR::Cps1    => config = self.config.with_low( BF::CR2).with_high(BF::CR1).with_low( BF::CR0),
            CR::Cps0_5  => config = self.config.with_low( BF::CR2).with_high(BF::CR1).with_high(BF::CR0),
            CR::Cps0_25 => config = self.config.with_high(BF::CR2).with_low( BF::CR1).with_low( BF::CR0),
        }
        self.write_config(config)
    }

    fn write_config(&mut self, config: ConfigHigh) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIG, config.bits, 0])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

impl<I2C, E> Tmp006<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the object temperature in Kelvins.
    ///
    /// This uses the sensor voltage and ambient temperature as well as an
    /// input calibration factor.
    ///
    /// The input calibration factor can be calculated with the formulas
    /// provided in the [TMP006 user guide].
    /// Typical values are between `5*10^-14` and `7*10^-14`
    ///
    /// [TMP006 user guide](https://cdn-shop.adafruit.com/datasheets/tmp006ug.pdf)
    pub fn read_object_temperature(
        &mut self,
        calibration_factor: f64,
    ) -> nb::Result<f64, Error<E>> {
        const A1: f64 = 1.75e-3;
        const A2: f64 = -1.678e-5;
        const B0: f64 = -2.94e-5;
        const B1: f64 = -5.7e-7;
        const B2: f64 = 4.63e-9;
        const C2: f64 = 13.4;
        const T_REF: f64 = 298.15;

        let data = self.read_sensor_data()?;
        let v_obj = data.object_voltage;
        let t_die = data.ambient_temperature;

        let t_diff = f64::from(t_die) - T_REF;
        let t_diff_sq = t_diff * t_diff;
        let vos = B0 + B1 * t_diff + B2 * t_diff_sq;
        let v_diff = f64::from(v_obj) - vos;
        let fv_obj = v_diff + C2 * v_diff * v_diff;
        let s0 = calibration_factor;
        let s = s0 * (1.0 + A1 * t_diff + A2 * t_diff_sq);
        let tobj = (libm::pow(f64::from(t_die), 4.0) + fv_obj / s)
            .sqrt()
            .sqrt();

        Ok(tobj)
    }

    /// Read the data from the sensor.
    ///
    /// These values can be used to calculate the object temperature as done in
    /// [`read_object_temperature()`].
    ///
    /// [`read_object_temperature()`]: struct.Tmp006.html#method.read_object_temperature
    pub fn read_sensor_data(&mut self) -> nb::Result<SensorData, Error<E>> {
        let ready = self.is_data_ready().map_err(nb::Error::Other)?;
        if !ready {
            return Err(nb::Error::WouldBlock);
        }
        let v = self
            .read_register(Register::V_OBJECT)
            .map_err(nb::Error::Other)?;
        let temp = self
            .read_register(Register::TEMP_AMBIENT)
            .map_err(nb::Error::Other)?;
        let data = SensorData {
            object_voltage: v as i16,
            ambient_temperature: temp as i16 / 4,
        };
        Ok(data)
    }

    /// Reads whether there is data ready to be read.
    ///
    /// When this returens `false` it means that a conversion is in progress.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_data_ready(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register(Register::CONFIG)?;
        Ok((config & u16::from(BitFlagsLow::DRDY)) != 0)
    }

    /// Read the manufacturer ID.
    ///
    /// This is per default `0x5449`.
    pub fn read_manufacturer_id(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::MANUFAC_ID)
    }

    /// Read the device ID.
    ///
    /// This is per default `0x0067`.
    pub fn read_device_id(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::DEVICE_ID)
    }

    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok((u16::from(data[0]) << 8) | u16::from(data[1]))
    }
}

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