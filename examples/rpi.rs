use std::{thread::sleep, time::Duration};

use microdotphat::{
    matrix::{Matrix, NanoMatrix},
    MicroDotPHAT,
};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let microdot = MicroDotPHAT::new(microdotphat::DEFAULT_ADDRESS, i2c);
}
