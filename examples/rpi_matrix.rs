use std::{thread::sleep, time::Duration};

use microdotphat::nano_matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let mut matrix = NanoMatrix::new(0x61);
    matrix.set_decimal(Matrix::One, true);
    matrix.update(&mut i2c).expect("failed to update");
    sleep(Duration::from_secs(5));
    println!("Set brightness to 0.1");
    matrix
        .set_brightness(&mut i2c, 0.1)
        .expect("failed to set brightness");
    sleep(Duration::from_secs(5));
    matrix.clear(&mut i2c).expect("failed to clear");
}
