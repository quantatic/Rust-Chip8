pub struct Ram {
    ram: [u8; 0x1000],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            ram: [0; 0x1000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[usize::from(addr)]
    }

    pub fn set(&mut self, addr: u16, new: u8) {
        self.ram[usize::from(addr)] = new;
    }
}
