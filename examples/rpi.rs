use std::{thread::sleep, time::Duration};

use microdotphat::nano_matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let mut microdot = NanoMatrix::new(0x61, i2c);
    microdot.set_pixel(Matrix::One, 0, 0, true);
    microdot.update().expect("failed to update");
    sleep(Duration::from_secs(5));
    println!("Set brightness to 0.1");
    microdot
        .set_brightness(0.1)
        .expect("failed to set brightness");
    sleep(Duration::from_secs(5));
    microdot.clear(Matrix::One).expect("failed to clear");
}
