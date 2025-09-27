use crate::Error;
use embedded_hal::i2c::I2c;

pub enum Matrix {
    MatrixOne,
    MatrixTwo,
}

pub struct NanoMatrix<I2C> {
    i2c: I2C,
    address: u8,
    brightness: u8,
    matrix_1: [u8; 8],
    matrix_2: [u8; 8],
}

impl<I2C, I2cError> NanoMatrix<I2C>
where
    I2C: I2c<Error = I2cError>,
{
    pub fn new(address: u8, i2c: I2C) -> Self {
        NanoMatrix {
            i2c,
            address,
            brightness: 127,
            matrix_1: [0; 8],
            matrix_2: [0; 8],
        }
    }

    pub fn setup(&mut self) -> Result<(), Error<I2cError>> {
        self.i2c
            .write(self.address, &[commands::CMD_MODE, commands::MODE])?;
        self.i2c
            .write(self.address, &[commands::CMD_OPTIONS, commands::OPTS])?;
        self.i2c
            .write(self.address, &[commands::CMD_BRIGHTNESS, self.brightness])?;
        Ok(())
    }

    pub fn set_pixel(&mut self, matrix: Matrix, x: usize, y: usize, on: bool) {
        match matrix {
            Matrix::MatrixOne => {
                if on {
                    self.matrix_1[y] |= 0b1 << x;
                } else {
                    self.matrix_1[y] &= !(0b1 << x);
                }
            }
            Matrix::MatrixTwo => {
                if on {
                    self.matrix_2[y] |= 0b1 << x;
                } else {
                    self.matrix_2[y] &= !(0b1 << x);
                }
            }
        }
    }

    pub fn update(&mut self) -> Result<(), Error<I2cError>> {
        let mut frame1 = [0u8; 9];
        frame1[0] = commands::CMD_MATRIX_1;
        frame1[1..].copy_from_slice(&self.matrix_1);
        self.i2c.write(self.address, &frame1)?;

        let mut frame2 = [0u8; 9];
        frame2[0] = commands::CMD_MATRIX_2;
        frame2[1..].copy_from_slice(&self.matrix_2);
        self.i2c.write(self.address, &frame2)?;

        self.i2c
            .write(self.address, &[commands::CMD_UPDATE, 0x01])?;
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
