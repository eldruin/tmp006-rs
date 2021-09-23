use linux_embedded_hal::I2cdev;
use nb::block;
use tmp006::{SlaveAddr, Tmp006};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Tmp006::new(dev, address);
    let calibration_factor = 6e-14;
    let temperature = block!(sensor.read_object_temperature(calibration_factor)).unwrap();
    println!("Temperature: {}K", temperature);
}
