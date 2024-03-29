const FONT_SET: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];

pub struct Ram {
    ram: [u8; 0x1000],
}

impl Ram {
    pub fn new() -> Ram {
        let mut res = Ram {
            ram: [0; 0x1000],
        };

        for i in 0..80 {
            res.set(i, FONT_SET[usize::from(i)]);
        };

        res
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[usize::from(addr)]
    }

    pub fn set(&mut self, addr: u16, new: u8) {
        self.ram[usize::from(addr)] = new;
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test_init_zero() {
        let ram = Ram::new();

        for i in 0x200..0x1000 {
            assert_eq!(ram.read(i), 0x0);
        }
    }

    #[test]
    fn test_set() {
        let mut ram = Ram::new();

        ram.set(0x400, 0x12);
        assert_eq!(ram.read(0x400), 0x12);

        ram.set(0x500, 0x50);
        assert_eq!(ram.read(0x500), 0x50);
        assert_eq!(ram.read(0x400), 0x12);
    }

    #[test]
    fn test_font_init() {
        let ram = Ram::new();

        for (i, &val) in FONT_SET.iter().enumerate() {
            assert_eq!(ram.read(u16::try_from(i).unwrap()), val);
        }
    }
}
