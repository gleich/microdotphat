pub mod nano_matrix;

// #[derive(Debug)]
// pub struct MicroDotPHAT<I2C> {
//     pub i2c: I2C,
//     pub display_addresses: [u8; 3],
//     pub buff: [u8; 6],
// }

// impl<I2C, I2cError> MicroDotPHAT<I2C>
// where
//     I2C: Write<Error = I2cError>,
// {
//     /// Make a new microdot instance
//     pub fn new(address: u8, i2c: I2C) -> Self {
//         MicroDotPHAT {
//             i2c,
//             display_addresses: [address, address + 1, address + 2],
//             buff: [32, 32, 32, 32, 32, 32],
//         }
//     }

//     pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
//         delay.delay_ms(15);
//         for display in self.display_addresses {
//             self.i2c.write(display, &[0xFF, 0x00])?; // reset display
//             self.i2c.write(display, &[0x00, 0x18])?; // write to config register
//         }
//         delay.delay_ms(15);
//         Ok(())
//     }

//     pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<I2cError>> {
//         for display in self.display_addresses {
//             self.i2c.write(display, &[0x0D, DISPLAY_CURRENT])?; // write to lighting effect register
//             self.i2c.write(display, &[0x19, brightness])?; // write to lighting effect register
//         }
//         Ok(())
//     }

//     pub fn update(&mut self, address: u8) -> Result<(), Error<I2cError>> {
//         self.i2c.write(address, &[0x0c, 0xff])?;
//         Ok(())
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
