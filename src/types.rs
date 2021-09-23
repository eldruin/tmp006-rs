//! Type definition

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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a2, a1, a0) => {
                default | ((a2 as u8) << 2) | ((a1 as u8) << 1) | a0 as u8
            }
        }
    }
}

pub const DEVICE_BASE_ADDRESS: u8 = 0b100_0000;

pub struct Register;
impl Register {
    pub const V_OBJECT: u8 = 0x00;
    pub const TEMP_AMBIENT: u8 = 0x01;
    pub const CONFIG: u8 = 0x02;
    pub const MANUFAC_ID: u8 = 0xFE;
    pub const DEVICE_ID: u8 = 0xFE;
}

pub struct BitFlagsHigh;
impl BitFlagsHigh {
    pub const SW_RESET: u8 = 0b1000_0000;
    pub const MOD: u8 = 0b0111_0000;
    pub const CR2: u8 = 0b0000_1000;
    pub const CR1: u8 = 0b0000_0100;
    pub const CR0: u8 = 0b0000_0010;
    pub const DRDY_EN: u8 = 0b0000_0001;
}

pub struct BitFlagsLow;
impl BitFlagsLow {
    pub const DRDY: u8 = 0b1000_0000;
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigHigh {
    pub bits: u8,
}

/// TMP006 device driver.
#[derive(Debug)]
pub struct Tmp006<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    /// Configuration register status.
    pub(crate) config: ConfigHigh,
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
        assert_eq!(
            0b100_0000,
            SlaveAddr::Alternative(false, false, false).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0b100_0001,
            SlaveAddr::Alternative(false, false, true).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0b100_0010,
            SlaveAddr::Alternative(false, true, false).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0b100_0100,
            SlaveAddr::Alternative(true, false, false).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0b100_0111,
            SlaveAddr::Alternative(true, true, true).addr(DEVICE_BASE_ADDRESS)
        );
    }
}
