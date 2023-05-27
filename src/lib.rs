use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Write;

// from https://github.com/mlukasek/microdotphat_library/blob/fd6ae84061514a80ecf26a36ec3649fdfd1e0722/microdot.cpp#L3
const COLS: [u8; 480] = [
    0x00, 0x00, 0x00, 0x00, 0x00, // (space)  00
    0x00, 0x00, 0x5F, 0x00, 0x00, // !        01
    0x00, 0x07, 0x00, 0x07, 0x00, // "        02
    0x14, 0x7F, 0x14, 0x7F, 0x14, // #        03
    0x24, 0x2A, 0x7F, 0x2A, 0x12, // $        04
    0x23, 0x13, 0x08, 0x64, 0x62, // %        05
    0x36, 0x49, 0x55, 0x22, 0x50, // &        06
    0x00, 0x05, 0x03, 0x00, 0x00, // '        07
    0x00, 0x1C, 0x22, 0x41, 0x00, // (        08
    0x00, 0x41, 0x22, 0x1C, 0x00, // )        09
    0x08, 0x2A, 0x1C, 0x2A, 0x08, // *        0a
    0x08, 0x08, 0x3E, 0x08, 0x08, // +        0b
    0x00, 0x50, 0x30, 0x00, 0x00, // ,        0c
    0x08, 0x08, 0x08, 0x08, 0x08, // -        0d
    0x00, 0x60, 0x60, 0x00, 0x00, // .        0e
    0x20, 0x10, 0x08, 0x04, 0x02, // /        0f
    0x3E, 0x51, 0x49, 0x45, 0x3E, // 0        10
    0x00, 0x42, 0x7F, 0x40, 0x00, // 1        11
    0x42, 0x61, 0x51, 0x49, 0x46, // 2        12
    0x21, 0x41, 0x45, 0x4B, 0x31, // 3        13
    0x18, 0x14, 0x12, 0x7F, 0x10, // 4        14
    0x27, 0x45, 0x45, 0x45, 0x39, // 5        15
    0x3C, 0x4A, 0x49, 0x49, 0x30, // 6        16
    0x01, 0x71, 0x09, 0x05, 0x03, // 7        17
    0x36, 0x49, 0x49, 0x49, 0x36, // 8        18
    0x06, 0x49, 0x49, 0x29, 0x1E, // 9        19
    0x00, 0x36, 0x36, 0x00, 0x00, // :        1a
    0x00, 0x56, 0x36, 0x00, 0x00, // ;        1b
    0x00, 0x08, 0x14, 0x22, 0x41, // <        1c
    0x14, 0x14, 0x14, 0x14, 0x14, // =        1d
    0x41, 0x22, 0x14, 0x08, 0x00, // >        1e
    0x02, 0x01, 0x51, 0x09, 0x06, // ?        1f
    0x32, 0x49, 0x79, 0x41, 0x3E, // @        20
    0x7E, 0x11, 0x11, 0x11, 0x7E, // A        21
    0x7F, 0x49, 0x49, 0x49, 0x36, // B        22
    0x3E, 0x41, 0x41, 0x41, 0x22, // C        23
    0x7F, 0x41, 0x41, 0x22, 0x1C, // D        24
    0x7F, 0x49, 0x49, 0x49, 0x41, // E        25
    0x7F, 0x09, 0x09, 0x01, 0x01, // F        26
    0x3E, 0x41, 0x41, 0x51, 0x32, // G        27
    0x7F, 0x08, 0x08, 0x08, 0x7F, // H        28
    0x00, 0x41, 0x7F, 0x41, 0x00, // I        29
    0x20, 0x40, 0x41, 0x3F, 0x01, // J        2a
    0x7F, 0x08, 0x14, 0x22, 0x41, // K        2b
    0x7F, 0x40, 0x40, 0x40, 0x40, // L        2c
    0x7F, 0x02, 0x04, 0x02, 0x7F, // M        2d
    0x7F, 0x04, 0x08, 0x10, 0x7F, // N        2e
    0x3E, 0x41, 0x41, 0x41, 0x3E, // O        2f
    0x7F, 0x09, 0x09, 0x09, 0x06, // P        30
    0x3E, 0x41, 0x51, 0x21, 0x5E, // Q        31
    0x7F, 0x09, 0x19, 0x29, 0x46, // R        32
    0x46, 0x49, 0x49, 0x49, 0x31, // S        33
    0x01, 0x01, 0x7F, 0x01, 0x01, // T        34
    0x3F, 0x40, 0x40, 0x40, 0x3F, // U        35
    0x1F, 0x20, 0x40, 0x20, 0x1F, // V        36
    0x7F, 0x20, 0x18, 0x20, 0x7F, // W        37
    0x63, 0x14, 0x08, 0x14, 0x63, // X        38
    0x03, 0x04, 0x78, 0x04, 0x03, // Y        39
    0x61, 0x51, 0x49, 0x45, 0x43, // Z        3A
    0x00, 0x00, 0x7F, 0x41, 0x41, // [        3B
    0x02, 0x04, 0x08, 0x10, 0x20, // "\"      3C
    0x41, 0x41, 0x7F, 0x00, 0x00, // ]        3D
    0x04, 0x02, 0x01, 0x02, 0x04, // ^        3E
    0x40, 0x40, 0x40, 0x40, 0x40, // _        3F
    0x00, 0x01, 0x02, 0x04, 0x00, // `        40
    0x20, 0x54, 0x54, 0x54, 0x78, // a        41
    0x7F, 0x48, 0x44, 0x44, 0x38, // b        42
    0x38, 0x44, 0x44, 0x44, 0x20, // c        43
    0x38, 0x44, 0x44, 0x48, 0x7F, // d        44
    0x38, 0x54, 0x54, 0x54, 0x18, // e        45
    0x08, 0x7E, 0x09, 0x01, 0x02, // f        46
    0x08, 0x14, 0x54, 0x54, 0x3C, // g        47
    0x7F, 0x08, 0x04, 0x04, 0x78, // h        48
    0x00, 0x44, 0x7D, 0x40, 0x00, // i        49
    0x20, 0x40, 0x44, 0x3D, 0x00, // j        4A
    0x00, 0x7F, 0x10, 0x28, 0x44, // k        4B
    0x00, 0x41, 0x7F, 0x40, 0x00, // l        4C
    0x7C, 0x04, 0x18, 0x04, 0x78, // m        4D
    0x7C, 0x08, 0x04, 0x04, 0x78, // n        4E
    0x38, 0x44, 0x44, 0x44, 0x38, // o        4F
    0x7C, 0x14, 0x14, 0x14, 0x08, // p        50
    0x08, 0x14, 0x14, 0x18, 0x7C, // q        51
    0x7C, 0x08, 0x04, 0x04, 0x08, // r        52
    0x48, 0x54, 0x54, 0x54, 0x20, // s        53
    0x04, 0x3F, 0x44, 0x40, 0x20, // t        54
    0x3C, 0x40, 0x40, 0x20, 0x7C, // u        55
    0x1C, 0x20, 0x40, 0x20, 0x1C, // v        56
    0x3C, 0x40, 0x30, 0x40, 0x3C, // w        57
    0x44, 0x28, 0x10, 0x28, 0x44, // x        58
    0x0C, 0x50, 0x50, 0x50, 0x3C, // y        59
    0x44, 0x64, 0x54, 0x4C, 0x44, // z        5A
    0x00, 0x08, 0x36, 0x41, 0x00, // {        5B
    0x00, 0x00, 0x7F, 0x00, 0x00, // |        5C
    0x00, 0x41, 0x36, 0x08, 0x00, // }        5D
    0x08, 0x08, 0x2A, 0x1C, 0x08, // ->       5E
    0x08, 0x1C, 0x2A, 0x08, 0x08, // <-      5F
];

/// Default address for the adafruit microdot phat
pub const DEFAULT_ADDRESS: u8 = 0x61;

/// Display current in mA
const DISPLAY_CURRENT: u8 = 0x00; // 40 mA

#[derive(Debug)]
pub struct MicroDotPHAT<I2C> {
    pub i2c: I2C,
    pub display_addresses: [u8; 3],
    pub buff: [u8; 6],
}

impl<I2C, I2cError> MicroDotPHAT<I2C>
where
    I2C: Write<Error = I2cError>,
{
    /// Make a new microdot instance
    pub fn new(address: u8, i2c: I2C) -> Self {
        MicroDotPHAT {
            i2c,
            display_addresses: [address, address + 1, address + 2],
            buff: [32, 32, 32, 32, 32, 32],
        }
    }

    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        delay.delay_ms(15);
        for display in self.display_addresses {
            self.i2c.write(display, &[0xFF, 0x00])?; // reset display
            self.i2c.write(display, &[0x00, 0x18])?; // write to config register
        }
        delay.delay_ms(15);
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<I2cError>> {
        for display in self.display_addresses {
            self.i2c.write(display, &[0x0D, DISPLAY_CURRENT])?; // write to lighting effect register
            self.i2c.write(display, &[0x19, brightness])?; // write to lighting effect register
        }
        Ok(())
    }

    pub fn update(&mut self, address: u8) -> Result<(), Error<I2cError>> {
        self.i2c.write(address, &[0x0c, 0xff])?;
        Ok(())
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
