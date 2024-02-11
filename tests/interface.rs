use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use tmp006::{ConversionRate, SensorData, SlaveAddr, Tmp006};

const DEV_ADDR: u8 = 0b100_0000;

struct Register;
impl Register {
    const V_OBJECT: u8 = 0x00;
    const TEMP_AMBIENT: u8 = 0x01;
    const CONFIG: u8 = 0x02;
    const MANUFAC_ID: u8 = 0xFE;
    const DEVICE_ID: u8 = 0xFE;
}

struct BitFlagsHigh;
impl BitFlagsHigh {
    const SW_RESET: u8 = 0b1000_0000;
    const MOD: u8 = 0b0111_0000;
    const CR2: u8 = 0b0000_1000;
    const CR1: u8 = 0b0000_0100;
    const CR0: u8 = 0b0000_0010;
    const DRDY_EN: u8 = 0b0000_0001;
}
struct BitFlagsLow;
impl BitFlagsLow {
    const DRDY: u8 = 0b1000_0000;
}

const CONFIG_DEFAULT: u8 = BitFlagsHigh::MOD | BitFlagsHigh::CR1;
const CONFIG_RDY_LOW: u8 = BitFlagsLow::DRDY;

fn new(transactions: &[I2cTrans]) -> Tmp006<I2cMock> {
    Tmp006::new(I2cMock::new(transactions), SlaveAddr::default())
}

fn destroy(tmp: Tmp006<I2cMock>) {
    tmp.destroy().done();
}

#[test]
fn can_create() {
    let tmp = new(&[]);
    destroy(tmp);
}

macro_rules! write_test {
    ($name:ident, $method:ident, $reg:ident, $value_msb:expr, $value_lsb:expr $( ,$arg:expr )*) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value_msb, $value_lsb])];
            let mut tmp = new(&trans);
            tmp.$method($( $arg, )*).unwrap();
            destroy(tmp);
        }
    };
}

write_test!(can_enable, enable, CONFIG, CONFIG_DEFAULT, 0);
write_test!(
    can_disable,
    disable,
    CONFIG,
    CONFIG_DEFAULT & !BitFlagsHigh::MOD,
    0
);
write_test!(
    can_reset,
    reset,
    CONFIG,
    CONFIG_DEFAULT | BitFlagsHigh::SW_RESET,
    0
);
write_test!(
    can_enable_drdy,
    enable_drdy_pin,
    CONFIG,
    CONFIG_DEFAULT | BitFlagsHigh::DRDY_EN,
    0
);
write_test!(
    can_disable_drdy,
    disable_drdy_pin,
    CONFIG,
    CONFIG_DEFAULT,
    0
);

fn get_config_high(cr2: bool, cr1: bool, cr0: bool) -> u8 {
    let mut config = BitFlagsHigh::MOD;
    if cr2 {
        config |= BitFlagsHigh::CR2;
    }
    if cr1 {
        config |= BitFlagsHigh::CR1;
    }
    if cr0 {
        config |= BitFlagsHigh::CR0;
    }
    config
}

macro_rules! conversion_rate_test {
    ($name:ident, $variant:ident, $cr2:expr, $cr1:expr, $cr0:expr) => {
        write_test!(
            $name,
            set_conversion_rate,
            CONFIG,
            get_config_high($cr2, $cr1, $cr0),
            0,
            ConversionRate::$variant
        );
    };
}

conversion_rate_test!(can_set_cr4, Cps4, false, false, false);
conversion_rate_test!(can_set_cr2, Cps2, false, false, true);
conversion_rate_test!(can_set_cr1, Cps1, false, true, false);
conversion_rate_test!(can_set_cr0_5, Cps0_5, false, true, true);
conversion_rate_test!(can_set_cr0_25, Cps0_25, true, false, false);

macro_rules! write_read_test {
    ($name:ident, $method:ident, $expected:expr, $( [ $reg:ident, $value_msb:expr, $value_lsb:expr ] ),*) => {
        #[test]
        fn $name() {
            let trans = [
                $( I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$value_msb, $value_lsb]) ),*
            ];
            let mut tmp = new(&trans);
            let current = tmp.$method().unwrap();
            assert_eq!($expected, current);
            destroy(tmp);
        }
    };
}

macro_rules! sensor_data_test {
    ($name:ident, $object_volt:expr, $ambient_temp:expr, $msb_v:expr, $lsb_v:expr, $msb_t:expr, $lsb_t:expr) => {
        write_read_test!(
            $name,
            read_sensor_data,
            SensorData {
                object_voltage: $object_volt,
                ambient_temperature: $ambient_temp
            },
            [CONFIG, CONFIG_DEFAULT, CONFIG_RDY_LOW],
            [V_OBJECT, $msb_v, $lsb_v],
            [TEMP_AMBIENT, $msb_t, $lsb_t]
        );
    };
}

sensor_data_test!(can_read_voltage_max, 32767, 0, 0x7F, 0xFF, 0, 0);
sensor_data_test!(can_read_voltage_0, 0, 0, 0, 0, 0, 0);
sensor_data_test!(can_read_voltage_min, -32768, 0, 0x80, 0x00, 0, 0);

sensor_data_test!(can_read_ambient_t_max, 0, 8191, 0, 0, 0x7F, 0xFC);
sensor_data_test!(can_read_ambient_t_0, 0, 0, 0, 0, 0, 0);
sensor_data_test!(can_read_ambient_t_min, 0, -8192, 0, 0, 0x80, 0x00);

write_read_test!(
    can_read_data_ready,
    is_data_ready,
    true,
    [CONFIG, 0, CONFIG_RDY_LOW]
);
write_read_test!(
    can_read_data_not_ready,
    is_data_ready,
    false,
    [CONFIG, 0, 0]
);

write_read_test!(
    can_read_manuf,
    read_manufacturer_id,
    0x5449,
    [MANUFAC_ID, 0x54, 0x49]
);
write_read_test!(
    can_read_dev_id,
    read_device_id,
    0x0067,
    [DEVICE_ID, 0x00, 0x67]
);

macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block."),
        }
    };
}

#[test]
fn cannot_read_object_temperature_if_not_ready() {
    let trans = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![0, 0],
    )];
    let mut tmp = new(&trans);
    let result = tmp.read_object_temperature(6e-14);
    assert_would_block!(result);
    destroy(tmp);
}

#[test]
fn cannot_read_data_if_not_ready() {
    let trans = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![0, 0],
    )];
    let mut tmp = new(&trans);
    let result = tmp.read_sensor_data();
    assert_would_block!(result);
    destroy(tmp);
}

#[test]
fn can_read_object_temperature_real_data() {
    /* For some example values of V_obj=-100 and T_ambient=675.
        If you put this into maxima (the program) (or mathematica) you should
        be able to get the same result: 278.5701125352883.
        sqrt(sqrt(
            (675/128 + 273.15)^4+(
                ((-100*156.25*10^-9)
                    - (-2.94e-5 -5.7e-7*((675/128 + 273.15)-298.15)
                    + 4.63e-9*((675/128 + 273.15)-298.15)²))
                + 13.4 * ((-100*156.25*10^-9)
                - (-2.94e-5 -5.7e-7*((675/128 + 273.15)-298.15)
                    + 4.63e-9*((675/128 + 273.15)-298.15)²))²)
                 /
                ( 6e-14
                    * (1 + 1.75e-3*((675/128 + 273.15)-298.15)
                        -1.678e-5*((675/128 + 273.15)-298.15)²)
                )
        ))
    */

    let trans = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![0, CONFIG_RDY_LOW]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::V_OBJECT], vec![0xFF, 0b1001_1011]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::TEMP_AMBIENT], vec![0xA, 0x8C]),
    ];
    let mut tmp = new(&trans);
    let current = tmp.read_object_temperature(6e-14).unwrap();
    assert!((current - 278.57).abs() < 0.1);
    destroy(tmp);
}
