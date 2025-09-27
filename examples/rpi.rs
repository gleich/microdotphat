use std::{thread::sleep, time::Duration};

use microdotphat::{
    nano_matrix::{Matrix, NanoMatrix},
    MicrodotPHAT,
};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("Failed to load i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create new microdotphat");
}
