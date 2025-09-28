use std::{thread::sleep, time::Duration};

use microdotphat::nano_matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let mut matrix = NanoMatrix::new(0x61);
    matrix.set_decimal(Matrix::One, true);
    matrix.set_decimal(Matrix::Two, true);
    matrix.update(&mut i2c);
    sleep(Duration::from_secs(5));
}
