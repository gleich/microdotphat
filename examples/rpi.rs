use std::{thread::sleep, time::Duration};

use microdotphat::nano_matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let mut microdot = NanoMatrix::new(0x61, i2c);
    microdot.set_pixel(Matrix::One, 0, 0, true);
    microdot.update().expect("failed to update");
    sleep(Duration::from_secs(10));
    microdot.clear(Matrix::One).expect("failed to clear");
}
