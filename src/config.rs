use hal::blocking::i2c;
use {
    BitFlagsHigh, ConfigHigh, ConversionRate, DEVICE_BASE_ADDRESS,
    Error, Register, SlaveAddr, Tmp006
};

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
