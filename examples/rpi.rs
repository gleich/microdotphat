use std::{thread::sleep, time::Duration};

use microdotphat::nano_matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let microdot = NanoMatrix::new(0x61, i2c);
}
