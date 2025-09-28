use std::{thread::sleep, time::Duration};

use microdotphat::MicrodotPHAT;
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("failed to open i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create microdotphat");

    display.clear(&mut i2c).expect("failed to clear display");

    display.write_string("60.0db");
    display.set_decimal(0, true);
    display.set_decimal(1, true);
    display.set_decimal(2, true);
    display.set_decimal(3, true);

    display
        .show(&mut i2c, true)
        .expect("failed to write to display");

    sleep(Duration::from_secs(3));
}
