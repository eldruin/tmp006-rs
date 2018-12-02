extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate tmp006;

use linux_embedded_hal::I2cdev;
use tmp006::{Tmp006, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tmp006::new(dev, SlaveAddr::default());
    let temperature = sensor.read_object_temperature(6e-14).unwrap();
    println!("Temperature: {}K", temperature);
}
