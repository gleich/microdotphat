use embedded_hal::blocking::i2c::Write;

pub struct NanoMatrix<I2C> {
    pub address: u8,
    pub brightness: u8,
    pub i2c: I2C,
    pub matrix_1: [u8; 8],
    pub matrix_2: [u8; 8],
}

impl<I2C, I2cError> NanoMatrix<I2C>
where
    I2C: Write<Error = I2cError>,
{
    pub fn setup(&mut self) -> Result<(), Error<I2cError>> {
        self.i2c
            .write(self.address, &[addresses::CMD_MODE, addresses::MODE])?;
        self.i2c
            .write(self.address, &[addresses::CMD_OPTIONS, addresses::OPTS])?;
        self.i2c
            .write(self.address, &[addresses::CMD_BRIGHTNESS, self.brightness])?;
        Ok(())
    }

    pub fn set_brightness(&mut self, mut brightness: u8) -> Result<(), Error<I2cError>> {
        if brightness > 127 {
            brightness = 127;
        }
        self.i2c
            .write(self.address, &[addresses::CMD_BRIGHTNESS, brightness])?;
        Ok(())
    }

    pub fn set_pixel(&mut self, matrix: Matrix, x: u8, y: u8, on: bool) {
        match matrix {
            Matrix::One => {
                if on {
                    self.matrix_1[y as usize] |= 0b1 << x;
                } else {
                    self.matrix_1[y as usize] &= !(0b1 << x);
                }
            }
            Matrix::Two => {
                if on {
                    self.matrix_2[x as usize] |= 0b1 << y;
                } else {
                    self.matrix_2[x as usize] |= !(0b1 << y);
                }
            }
        }
    }

    pub fn update(&mut self) -> Result<(), Error<I2cError>> {
        let mut buffer = [addresses::CMD_MATRIX_1].to_vec();
        buffer.extend_from_slice(&self.matrix_1);
        self.i2c.write(self.address, &buffer)?;
        buffer = [addresses::CMD_MATRIX_2].to_vec();
        buffer.extend_from_slice(&self.matrix_2);
        self.i2c.write(self.address, &buffer)?;
        self.i2c
            .write(self.address, &[addresses::CMD_UPDATE, 0x01])?;
        Ok(())
    }

    pub fn clear(&mut self, matrix: Matrix) {
        match matrix {
            Matrix::One => {
                self.matrix_1 = [0; 8];
            }
            Matrix::Two => {
                self.matrix_2 = [0; 8];
            }
        }
    }

    pub fn set_decimal(&mut self, matrix: Matrix, on: bool) {
        match matrix {
            Matrix::One => {
                if on {
                    self.matrix_1[6] |= 0b10000000;
                } else {
                    self.matrix_1[6] &= 0b01111111;
                }
            }
            Matrix::Two => {
                if on {
                    self.matrix_2[7] |= 0b10000000;
                } else {
                    self.matrix_2[7] &= 0b01111111;
                }
            }
        }
    }
}

pub enum Matrix {
    One,
    Two,
}

pub mod addresses {
    pub const CMD_MODE: u8 = 0x00;
    pub const CMD_BRIGHTNESS: u8 = 0x19;
    pub const CMD_UPDATE: u8 = 0x0C;
    pub const CMD_OPTIONS: u8 = 0x0D;
    pub const CMD_MATRIX_1: u8 = 0x01;
    pub const CMD_MATRIX_2: u8 = 0x0E;

    pub const MODE: u8 = 0b00011000;
    pub const OPTS: u8 = 0b00001110;
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
