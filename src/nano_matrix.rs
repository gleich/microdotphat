use crate::Error;
use embedded_hal::i2c::I2c;

pub enum Matrix {
    One,
    Two,
}

pub struct NanoMatrix {
    address: u8,
    brightness: u8,
    matrix_1: [u8; 8],
    matrix_2: [u8; 8],
}

impl NanoMatrix {
    pub fn new(address: u8) -> Self {
        NanoMatrix {
            address,
            brightness: 127,
            matrix_1: [0; 8],
            matrix_2: [0; 8],
        }
    }

    pub fn setup<I2C, E>(&self, i2c: &mut I2C) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        i2c.write(self.address, &[commands::CMD_MODE, commands::MODE])?;
        i2c.write(self.address, &[commands::CMD_OPTIONS, commands::OPTS])?;
        i2c.write(self.address, &[commands::CMD_BRIGHTNESS, self.brightness])?;
        Ok(())
    }

    pub fn set_pixel(&mut self, matrix: &Matrix, x: usize, y: usize, on: bool) {
        match matrix {
            Matrix::One => {
                if on {
                    self.matrix_1[y] |= 0b1 << x;
                } else {
                    self.matrix_1[y] &= !(0b1 << x);
                }
            }
            Matrix::Two => {
                if on {
                    self.matrix_2[x] |= 0b1 << y;
                } else {
                    self.matrix_2[x] &= !(0b1 << y);
                }
            }
        }
    }

    pub fn clear<I2C, E>(&mut self, i2c: &mut I2C) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        self.matrix_1 = [0; 8];
        self.matrix_2 = [0; 8];
        self.update(i2c)?;
        Ok(())
    }

    pub fn set_brightness<I2C, E>(&mut self, i2c: &mut I2C, brightness: f32) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        self.brightness = (brightness * 127.0).clamp(0.0, 127.0) as u8;
        i2c.write(self.address, &[commands::CMD_BRIGHTNESS, self.brightness])?;
        Ok(())
    }

    pub fn set_decimal(&mut self, matrix: Matrix, value: u8) {
        match matrix {
            Matrix::One => {
                if value == 1 {
                    self.matrix_1[6] |= 0b10000000;
                } else {
                    self.matrix_1[6] |= 0b01111111;
                }
            }
            Matrix::Two => {
                if value == 1 {
                    self.matrix_2[6] |= 0b10000000;
                } else {
                    self.matrix_2[6] |= 0b01111111;
                }
            }
        }
    }

    pub fn update<I2C, E>(&mut self, i2c: &mut I2C) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut frame1 = [0u8; 9];
        frame1[0] = commands::CMD_MATRIX_1;
        frame1[1..].copy_from_slice(&self.matrix_1);
        i2c.write(self.address, &frame1)?;

        let mut frame2 = [0u8; 9];
        frame2[0] = commands::CMD_MATRIX_2;
        frame2[1..].copy_from_slice(&self.matrix_2);
        i2c.write(self.address, &frame2)?;

        i2c.write(self.address, &[commands::CMD_UPDATE, 0x01])?;
        Ok(())
    }
}

mod commands {
    pub const CMD_MODE: u8 = 0x00;
    pub const CMD_BRIGHTNESS: u8 = 0x19;
    pub const CMD_UPDATE: u8 = 0x0C;
    pub const CMD_OPTIONS: u8 = 0x0D;

    pub const CMD_MATRIX_1: u8 = 0x01;
    pub const CMD_MATRIX_2: u8 = 0x0E;

    pub const MODE: u8 = 0b00011000;
    pub const OPTS: u8 = 0b00001110;
}
