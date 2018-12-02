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

macro_rules! method_test {
    ($name:ident, $method:ident, $reg:ident, $value_msb:expr, $value_lsb:expr $(,$arg:expr),*) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value_msb, $value_lsb])];
            let mut tmp = new(&trans);
            tmp.$method( $($arg)* ).unwrap();
            destroy(tmp);
        }
    };
}

method_test!(can_enable, enable, CONFIG, CONFIG_DEFAULT, 0);
method_test!(can_disable, disable, CONFIG, CONFIG_DEFAULT & !BitFlags::MOD, 0);
