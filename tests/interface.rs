extern crate tmp006;
use tmp006::{SensorData, SlaveAddr, Tmp006};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0b100_0000;

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

const CONFIG_DEFAULT: u8 = BitFlags::MOD | BitFlags::CR1;

fn new(transactions: &[I2cTrans]) -> Tmp006<I2cMock> {
    Tmp006::new(I2cMock::new(&transactions), SlaveAddr::default())
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
    ($name:ident, $method:ident, $reg:ident, $value_msb:expr, $value_lsb:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value_msb, $value_lsb])];
            let mut tmp = new(&trans);
            tmp.$method().unwrap();
            destroy(tmp);
        }
    };
}

write_test!(can_enable, enable, CONFIG, CONFIG_DEFAULT, 0);
write_test!(can_disable, disable, CONFIG, CONFIG_DEFAULT & !BitFlags::MOD, 0);
write_test!(can_reset, reset, CONFIG, CONFIG_DEFAULT | BitFlags::SW_RESET, 0);

macro_rules! write_read_test {
    ($name:ident, $method:ident, $expected:expr, $( [ $reg:ident, $value_msb:expr, $value_lsb:expr ] ),*) => {
        #[test]
        fn $name() {
            let trans = [ $( I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$value_msb, $value_lsb]) ),* ];
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
            $name, read_sensor_data,
            SensorData { object_voltage: $object_volt, ambient_temperature: $ambient_temp },
            [ V_OBJECT, $msb_v, $lsb_v ],
            [ TEMP_AMBIENT, $msb_t, $lsb_t ]
        );
    };
}

sensor_data_test!(can_read_voltage_max,  32767, 0, 0x7F, 0xFF, 0, 0);
sensor_data_test!(can_read_voltage_0,        0, 0,    0,    0, 0, 0);
sensor_data_test!(can_read_voltage_min, -32768, 0, 0x80, 0x00, 0, 0);

sensor_data_test!(can_read_ambient_t_max, 0,  8191, 0, 0, 0x7F, 0xFC);
sensor_data_test!(can_read_ambient_t_0,   0,     0, 0, 0,    0,    0);
sensor_data_test!(can_read_ambient_t_min, 0, -8192, 0, 0, 0x80, 0x00);

write_read_test!(can_read_data_ready, is_data_ready, true, [ CONFIG, 0, 0b1000_0000 ]);
write_read_test!(can_read_data_not_ready, is_data_ready, false, [ CONFIG, 0, 0 ]);

write_read_test!(can_read_manuf, read_manufacturer_id, 0x5449, [ MANUFAC_ID, 0x54, 0x49 ]);
write_read_test!(can_read_dev_id, read_device_id, 0x0067, [ DEVICE_ID, 0x00, 0x67 ]);

#[test]
fn can_read_object_temperature() {
    /* For some example values of V_obj=514 and T_ambient=257.
        If you put this into maxima (the program) (or mathematica) you should
        be able to get the same result: 89996.69046659373.
        sqrt(sqrt(
            257^4+(
                (514 - (-2.94e-5 -5.7e-7*(257-298.15) + 4.63e-9*(257-298.15)²))
                + 13.4 * (514 - (-2.94e-5 -5.7e-7*(257-298.15) + 4.63e-9*(257-298.15)²))²)
                 /
                ( 6e-14*(1+ 1.75e-3*(257-298.15)-1.678e-5*(257-298.15)²) )
        ))
    */

    let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::V_OBJECT], vec![2, 2]),
                 I2cTrans::write_read(DEV_ADDR, vec![Register::TEMP_AMBIENT], vec![4, 4])];
    let mut tmp = new(&trans);
    let current = tmp.read_object_temperature(6e-14).unwrap();
    assert!((current-89996.69).abs() < 0.1);
    destroy(tmp);
}
