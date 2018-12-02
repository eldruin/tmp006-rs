extern crate tmp006;
use tmp006::{Error, SlaveAddr, Tmp006};
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
    ($name:ident, $method:ident, $reg:ident, $value_msb:expr, $value_lsb:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$value_msb, $value_lsb])];
            let mut tmp = new(&trans);
            let current = tmp.$method().unwrap();
            assert_eq!($expected, current);
            destroy(tmp);
        }
    };
}

write_read_test!(can_read_voltage_max, read_object_voltage, V_OBJECT, 0x7F, 0xFF,  32767);
write_read_test!(can_read_voltage_0,   read_object_voltage, V_OBJECT,    0,    0,      0);
write_read_test!(can_read_voltage_min, read_object_voltage, V_OBJECT, 0x80, 0x00, -32768);

write_read_test!(can_read_ambient_t_max, read_ambient_temperature, TEMP_AMBIENT, 0x7F, 0xFC,  8191);
write_read_test!(can_read_ambient_t_0,   read_ambient_temperature, TEMP_AMBIENT,    0,    0,     0);
write_read_test!(can_read_ambient_t_min, read_ambient_temperature, TEMP_AMBIENT, 0x80, 0x00, -8192);

write_read_test!(can_read_manuf, read_manufacturer_id, MANUFAC_ID, 0x54, 0x49, 0x5449);

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
