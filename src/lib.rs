pub mod matrix;

const WIDTH: usize = 45;
const HEIGHT: usize = 7;

pub struct MicrodotPhat {
    pub buffer: [[u8; WIDTH]; HEIGHT],
    pub decimal_buffer: [u8; 6],
}

impl MicrodotPhat {
    pub fn clear(&mut self) {
        self.buffer = [[0; WIDTH]; HEIGHT];
        self.decimal_buffer = [0; 6];
    }
}
