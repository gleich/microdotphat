use embedded_hal::i2c::I2c;

use crate::nano_matrix::NanoMatrix;

pub mod nano_matrix;

pub struct MicrodotPHAT {
    pub matrices: [NanoMatrix; 3],
}

impl MicrodotPHAT {
    pub fn new<I2C, E>(i2c: &mut I2C) -> Result<Self, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let matrices: [NanoMatrix; 3] = [
            NanoMatrix::new(0x61),
            NanoMatrix::new(0x62),
            NanoMatrix::new(0x63),
        ];
        for matrix in &matrices {
            matrix.setup(i2c)?;
        }

        Ok(Self { matrices })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
