//! This is a platform agnostic Rust driver for the TMP006/TMP006B non-contact
//! infrared (IR) thermopile temperature sensor, based on the
//! [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Perform a software reset.
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
#![deny(missing_docs, unsafe_code)]
//TODO #![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;
extern crate libm;
use libm::F64Ext;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
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
    Alternative(bool, bool, bool)
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
            SlaveAddr::Alternative(a2, a1, a0) => default           |
                                                  ((a2 as u8) << 2) |
                                                  ((a1 as u8) << 1) |
                                                    a0 as u8
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


struct BitFlags;
impl BitFlags {
    const SW_RESET : u8 = 0b1000_0000;
    const MOD      : u8 = 0b0111_0000;
    const CR2      : u8 = 0b0000_1000;
    const CR1      : u8 = 0b0000_0100;
    const CR0      : u8 = 0b0000_0010;
    const DRDY_EN  : u8 = 0b0000_0001;
}

#[derive(Debug, Clone, Copy)]
struct Config {
    bits: u8,
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0 }
            .with_high(BitFlags::MOD)
            .with_high(BitFlags::CR1)
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
    config: Config,
}

impl<I2C, E> Tmp006<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the TMP006 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp006 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default()
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the sensor (default state).
    ///
    /// Sensor and ambient continuous conversion.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlags::MOD))
    }

    /// Disable the sensor (power-down).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_low(BitFlags::MOD))
    }

    /// Reset the sensor (software reset).
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlags::SW_RESET))?;
        self.config = Config::default();
        Ok(())
    }

    fn write_config(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIG, config.bits, 0])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

impl<I2C, E> Tmp006<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Read the sensor object voltage.
    ///
    /// This can be used in conjunction with the ambient temperature to
    /// calculate the object temperature. See [`read_object_temperature`].
    ///
    /// [`read_object_temperature`]: struct.Tmp006.html#method.read_object_temperature
    ///
    /// The result is in the value range `[-32768..32767]`.
    pub fn read_object_voltage(&mut self) -> Result<i16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::V_OBJECT], &mut data)
            .map_err(Error::I2C)?;
        let voltage = ((u16::from(data[0]) << 8) | u16::from(data[1])) as i16;
        Ok(voltage)
    }

    /// Read the ambient temperature.
    ///
    /// This can be used in conjunction with the sensor object voltage to
    /// calculate the object temperature.See [`read_object_temperature`].
    ///
    /// [`read_object_temperature`]: struct.Tmp006.html#method.read_object_temperature
    ///
    /// The result is in the value range `[-8192..8191]`.
    pub fn read_ambient_temperature(&mut self) -> Result<i16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::TEMP_AMBIENT], &mut data)
            .map_err(Error::I2C)?;
        let temp = ((u16::from(data[0]) << 8) | u16::from(data[1])) as i16;
        let temp = temp / 4;
        Ok(temp)
    }


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
        calibration_factor: f64
    ) -> Result<f64, Error<E>> {
        const A1: f64 = 1.75e-3;
        const A2: f64 = -1.678e-5;
        const B0: f64 = -2.94e-5;
        const B1: f64 = -5.7e-7;
        const B2: f64 = 4.63e-9;
        const C2: f64 = 13.4;
        const T_REF: f64 = 298.15;

        let v_obj = self.read_object_voltage()?;
        let t_die = self.read_ambient_temperature()?;

        let t_diff = f64::from(t_die) - T_REF;
        let t_diff_sq = t_diff * t_diff;
        let vos = B0 + B1*t_diff + B2*t_diff_sq;
        let v_diff = f64::from(v_obj) - vos;
        let fv_obj = v_diff + C2*v_diff*v_diff;
        let s0 = calibration_factor;
        let s = s0 * (1.0+A1*t_diff +A2*t_diff_sq);
        let tobj = (libm::pow(f64::from(t_die), 4.0) + fv_obj / s).sqrt().sqrt();

        Ok(tobj)
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