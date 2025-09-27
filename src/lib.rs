use crate::nano_matrix::NanoMatrix;

pub mod nano_matrix;

pub struct MicrodotPHAT<I2C> {
    pub matrices: [NanoMatrix<I2C>; 3],
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
