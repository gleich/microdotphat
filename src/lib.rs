use std::marker::PhantomData;

use embedded_hal::blocking::i2c::Write;
use matrix::NanoMatrix;

pub mod matrix;

const WIDTH: usize = 45;
const HEIGHT: usize = 7;

pub struct MicrodotPhat<I2C> {
    pub matrix_one: NanoMatrix<I2C>,
    pub matrix_two: NanoMatrix<I2C>,
    pub matrix_three: NanoMatrix<I2C>,
    pub buffer: [[u8; WIDTH]; HEIGHT],
    pub decimal_buffer: [u8; 6],
    __phantom: PhantomData<I2C>,
}

impl<'a, I2C, I2cError> MicrodotPhat<I2C>
where
    I2C: Write<Error = I2cError>,
{
    pub fn new() -> Result<(), Error<I2cError>> {
        Self {
            matrix_one: NanoMatrix::new(0x63),
            matrix_two: NanoMatrix::new(0x64),
            matrix_three: NanoMatrix::new(0x65),
            buffer: [[0; WIDTH]; HEIGHT],
            decimal_buffer: [0; 6],
            __phantom: PhantomData,
        };
        Ok(())
    }

    pub fn clear(&mut self) {
        self.fill(false);
        self.decimal_buffer = [0; 6];
    }

    pub fn fill(&mut self, on: bool) {
        self.buffer = [[if on { 1 } else { 0 }; WIDTH]; HEIGHT];
    }
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
