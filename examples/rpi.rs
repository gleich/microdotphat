use std::{thread::sleep, time::Duration};

use microdotphat::matrix::{Matrix, NanoMatrix};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    println!("Loaded I2C bus");
    let mut matrix = NanoMatrix {
        address: 0x63,
        brightness: 127,
        matrix_1: [0; 8],
        matrix_2: [0; 8],
        i2c,
    };
    matrix.setup().expect("Failed to setup matrix");
    loop {
        matrix.update().expect("Failed to update display");
        matrix.set_pixel(&Matrix::One, 0, 0, true);
        sleep(Duration::from_secs(1));
        matrix.update().expect("Failed to update display");
        sleep(Duration::from_secs(1));
        matrix.set_pixel(&Matrix::One, 0, 0, false);
    }
    println!("Done");
}
