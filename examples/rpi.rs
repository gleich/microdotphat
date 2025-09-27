use std::{thread::sleep, time::Duration};

use microdotphat::{
    nano_matrix::{Matrix, NanoMatrix},
    MicrodotPHAT, HEIGHT, WIDTH,
};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("Failed to load i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create new microdotphat");
    display.clear(&mut i2c).expect("failed to clear display");

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            display.set_pixel(x, y, true);
            display.show(&mut i2c).expect("failed to write to display");
            sleep(Duration::from_millis(50));
        }
    }
}
