use std::{thread::sleep, time::Duration};

use microdotphat::MicrodotPHAT;
use rppal::i2c::I2c;

fn main() {
    // Open I²C and init display
    let mut i2c = I2c::new().expect("failed to open i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create microdotphat");

    // Clear any previous contents
    display.clear(&mut i2c).expect("failed to clear display");

    // Write a test string at (x=0, y=0). Set kerning=true for tight spacing.
    let _len = display.write_string("60.0db", 0, 0);

    // Send buffer to hardware
    display
        .show(&mut i2c, true)
        .expect("failed to write to display");

    // Keep it on for a moment so you can see it
    sleep(Duration::from_secs(3));
}
