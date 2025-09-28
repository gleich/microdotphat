use embedded_hal::i2c::I2c;

use crate::nano_matrix::{Matrix, NanoMatrix};

mod font;
pub mod nano_matrix;

pub const WIDTH: usize = 30;
pub const HEIGHT: usize = 7;

pub struct MicrodotPHAT {
    buffer: [[u8; HEIGHT]; WIDTH],
    decimals: [u8; 6],
    matrices: [NanoMatrix; 3],
}

impl MicrodotPHAT {
    pub fn new<I2C, E>(i2c: &mut I2C) -> Result<Self, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let matrices: [NanoMatrix; 3] = [
            NanoMatrix::new(0x63),
            NanoMatrix::new(0x62),
            NanoMatrix::new(0x61),
        ];
        for matrix in &matrices {
            matrix.setup(i2c)?;
        }

        Ok(Self {
            matrices,
            buffer: [[0; HEIGHT]; WIDTH],
            decimals: [0; 6],
        })
    }

    pub fn clear<I2C, E>(&mut self, i2c: &mut I2C) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        self.buffer = [[0; HEIGHT]; WIDTH];
        for matrix in &mut self.matrices {
            matrix.clear(i2c)?;
        }
        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.buffer[x][y] = if on { 1 } else { 0 }
    }

    pub fn set_decimal(&mut self, idx: usize, on: bool) {
        if idx < self.decimals.len() {
            self.decimals[idx] = if on { 1 } else { 0 };
        }
    }

    pub fn show<I2C, E>(&mut self, i2c: &mut I2C, rotate180: bool) -> Result<(), Error<E>>
    where
        I2C: embedded_hal::i2c::I2c<Error = E>,
    {
        for (matrix_index, m) in self.matrices.iter_mut().enumerate() {
            let base = matrix_index * 10;

            let di = matrix_index * 2;
            if di < self.decimals.len() {
                m.set_decimal(Matrix::Two, self.decimals[di] != 0);
            }
            if di + 1 < self.decimals.len() {
                m.set_decimal(Matrix::One, self.decimals[di + 1] != 0);
            }

            for half in [Matrix::Two, Matrix::One] {
                let x0 = base + if matches!(half, Matrix::One) { 5 } else { 0 };

                let mut col = [0u8; 5];
                for lx in 0..5 {
                    let gx = x0 + lx;
                    if gx >= WIDTH {
                        break;
                    }

                    let mut bits = 0u8;
                    for y in 0..HEIGHT {
                        let (sx, sy) = if rotate180 {
                            (WIDTH - 1 - gx, HEIGHT - 1 - y)
                        } else {
                            (gx, y)
                        };
                        if self.buffer[sx][sy] != 0 {
                            bits |= 1 << y;
                        }
                    }
                    col[lx] = bits;
                }

                for lx in 0..5 {
                    for y in 0..HEIGHT {
                        let on = (col[lx] >> y) & 1 == 1;
                        m.set_pixel(&half, lx, y, on);
                    }
                }
            }

            m.update(i2c)?;
        }
        Ok(())
    }

    #[inline]
    fn put_column(&mut self, x: usize, offset_y: usize, col: u8) {
        if x >= WIDTH {
            return;
        }
        for y in 0..HEIGHT {
            let on = ((col >> y) & 1) == 1;
            let gy = offset_y + y;
            if gy < HEIGHT {
                self.buffer[x][gy] = if on { 1 } else { 0 };
            }
        }
    }

    pub fn write_string(&mut self, s: &str, offset_x: usize, offset_y: usize) -> usize {
        let mut x = offset_x;
        let space_width = 2;

        for ch in s.chars() {
            if ch == ' ' {
                for _ in 0..space_width {
                    self.put_column(x, offset_y, 0x00);
                    x += 1;
                }
                continue;
            }

            if let Some(cols) = font::lookup_glyph(ch) {
                let (start, end) = { (0usize, 4usize) };

                if end != usize::MAX {
                    for i in start..=end {
                        self.put_column(x, offset_y, cols[i]);
                        x += 1;
                    }
                }
            } else {
                for _ in 0..space_width {
                    self.put_column(x, offset_y, 0x00);
                    x += 1;
                }
            }
        }

        x.saturating_sub(offset_x)
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
