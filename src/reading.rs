use hal::blocking::i2c;
use {BitFlagsLow, Error, Register, SensorData, Tmp006};

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
        let data = self.read_sensor_data()?;
        let temp = self.calculate_object_temperature(data, calibration_factor);
        Ok(temp)
    }

    /// Calculate the object temperature in Kelvins.
    ///
    /// Given the sensor data and a calibration factor.
    ///
    /// The input calibration factor can be calculated with the formulas
    /// provided in the [TMP006 user guide].
    /// Typical values are between `5*10^-14` and `7*10^-14`
    ///
    /// [TMP006 user guide](https://cdn-shop.adafruit.com/datasheets/tmp006ug.pdf)
    pub fn calculate_object_temperature(
        &self,
        data: SensorData,
        calibration_factor: f64,
    ) -> f64 {
        const A1: f64 = 1.75e-3;
        const A2: f64 = -1.678e-5;
        const B0: f64 = -2.94e-5;
        const B1: f64 = -5.7e-7;
        const B2: f64 = 4.63e-9;
        const C2: f64 = 13.4;
        const T_REF: f64 = 298.15;
        const V_LSB_SIZE: f64 = 156.25e-9;

        let v_obj = f64::from(data.object_voltage) * V_LSB_SIZE;
        let t_die_k = f64::from(data.ambient_temperature) / 128.0 + 273.15;

        let t_diff = t_die_k - T_REF;
        let t_diff_sq = t_diff * t_diff;
        let v_os = B0 + B1 * t_diff + B2 * t_diff_sq;
        let v_diff = v_obj - v_os;
        let fv_obj = v_diff + C2 * v_diff * v_diff;
        let s0 = calibration_factor;
        let s = s0 * (1.0 + A1 * t_diff + A2 * t_diff_sq);
        libm::pow(libm::pow(t_die_k, 4.0) + fv_obj / s, 0.25)
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
