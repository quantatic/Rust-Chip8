use crate::display::Display;
use crate::keypad::Keypad;
use crate::ram::Ram;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::time::Instant;

use rand;

const TIME_PER_TIMER_DECREMENT: Duration = Duration::from_nanos(1_000_000_000 / 60);

pub struct Cpu<'a> {
    ram: &'a mut Ram,
    display: &'a mut Display,
    keypad: &'a mut Keypad,
    regs: [u8; 16],
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    i: u16,
    dt: u8,
    st: u8,
    last_timer_decrement: std::time::Instant,
}

impl<'a> Cpu<'a> {
    pub fn new(ram: &'a mut Ram, display: &'a mut Display, keypad: &'a mut Keypad) -> Self {
        Cpu {
            ram,
            display,
            keypad,
            regs: [0; 16],
            pc: 0x200,
            stack: [0; 16],
            sp: 0x0,
            i: 0x0,
            dt: 0x0,
            st: 0x0,
            last_timer_decrement: Instant::now(),
        }
    }

    pub fn load_rom_into_ram(&mut self, filename: &str) -> Result<(), std::io::Error>{
        let f = File::open(filename)?;

        for (i, byte) in f.bytes().enumerate() {
            self.ram.set(u16::try_from(i).unwrap() + 0x200, byte.unwrap());
        }

        Ok(())
    }

    pub fn disas(&self) {
        for pc in (0x200..0x1000).step_by(2) {
            print!("[0x{:04x}]: ", pc);

            let op = self.opcode_at(pc);
            self.print_opcode(op);
        }
    }

    pub fn print_opcode(&self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let nibbles = (
                (opcode & 0xF000) >> 12,
                (opcode & 0x0F00) >> 8,
                (opcode & 0x00F0) >> 4,
                (opcode & 0x000F)
            );

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => println!("CLS"),
            (0x0, 0x0, 0xE, 0xE) => println!("RET"),
            (0x1, _, _, _) => println!("JP 0x{:04x}", nnn),
            (0x2, _, _, _) => println!("CALL 0x{:04x}", nnn),
            (0x3, x, _, _) => println!("SE V{:01x}, 0x{:02x}", x, nn),
            (0x4, x, _, _) => println!("SNE V{:01x}, 0x{:02x}", x, nn),
            (0x5, x, y, 0x0) => println!("SE V{:01x}, V{:01x}", x, y),
            (0x6, x, _, _) => println!("LD V{:01x}, 0x{:02}", x, nn),
            (0x7, x, _, _) => println!("ADD V{:01x}, 0x{:02}", x, nn),
            (0x8, x, y, 0x0) => println!("LD V{:01x}, V{:01x}", x, y),
            (0x8, x, y, 0x1) => println!("OR V{:01x}, V{:01x}", x, y),
            (0x8, x, y, 0x2) => println!("AND V{:01x}, V{:01x}", x, y),
            (0x8, x, y, 0x3) => println!("XOR V{:01x}, V{:01x}", x, y),
            (0x8, x, y, 0x4) => println!("ADD V{:01x}, V{:01x}", x, y),
            (0x8, x, y, 0x5) => println!("SUB V{:01x}, V{:01x}", x, y),
            (0x8, x, _, 0x6) => println!("SHR V{:01x}", x),
            (0x8, x, y, 0x7) => println!("SUBN V{:01x}, V{:01x}", x, y),
            (0x8, x, _, 0xE) => println!("SHL V{:01x}", x),
            (0x9, x, y, 0x0) => println!("SNE V{:01x}, V{:01x}", x, y),
            (0xA, _, _, _) => println!("LD I, 0x{:04x}", nnn),
            (0xB, _, _, _) => println!("JP V00, 0x{:04x}", nnn),
            (0xC, x, _, _) => println!("RND V{:01x}, 0x{:02x}", x, nn),
            (0xD, x, y, n) => println!("DRW V{:01x}, V{:01x}, 0x{:01x}", x, y, n),
            (0xE, x, 0x9, 0xE) => println!("SKP V{:01x}", x),
            (0xE, x, 0xA, 0x1) => println!("SKNP V{:01x}", x),
            (0xF, x, 0x0, 0x7) => println!("LD V{:01x}, DT", x),
            (0xF, x, 0x0, 0xA) => println!("LD V{:01x}, K", x),
            (0xF, x, 0x1, 0x5) => println!("LD DT, V{:01x}", x),
            (0xF, x, 0x1, 0x8) => println!("LD ST, V{:01x}", x),
            (0xF, x, 0x1, 0xE) => println!("ADD I, V{:01x}", x),
            (0xF, x, 0x2, 0x9) => println!("LD F, V{:01x}", x),
            (0xF, x, 0x3, 0x3) => println!("LD B, V{:01x}", x),
            (0xF, x, 0x5, 0x5) => println!("LD [I], V{:01x}", x),
            (0xF, x, 0x6, 0x5) => println!("LD V{:01x}, [I]", x),
            (_, _, _, _) => println!("UNKNOWN OPCODE: (0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x})", nibbles.0, nibbles.1, nibbles.2, nibbles.3),
        };

        //println!("(0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x})", nibbles.0, nibbles.1, nibbles.2, nibbles.3);
    }

    fn opcode_at(&self, addr: u16) -> u16 {
        (u16::from(self.ram.read(addr)) << 8) | u16::from(self.ram.read(addr + 1))
    }

    pub fn tick(&mut self) {
        let op = self.opcode_at(self.pc);

        print!("[0x{:04x}]: ", self.pc);
        self.print_opcode(op);

        while self.last_timer_decrement.elapsed() > TIME_PER_TIMER_DECREMENT {
            if self.dt > 0 {
                self.dt -= 1;
            }

            if self.st > 0 {
                self.st -= 1;
            }

            self.last_timer_decrement += TIME_PER_TIMER_DECREMENT;
        }

        self.keypad.check_for_exit();
        self.run_opcode(op);
        self.display.redraw();
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let nibbles = (
                ((opcode & 0xF000) >> 12) as u8,
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
                (opcode & 0x000F) as u8
            );

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jp_addr(nnn),
            (0x2, _, _, _) => self.call_addr(nnn),
            (0x3, x, _, _) => self.se_vx_byte(x, nn),
            (0x4, x, _, _) => self.sne_vx_byte(x, nn),
            (0x5, x, y, 0x0) => self.se_vx_vy(x, y),
            (0x6, x, _, _) => self.ld_vx_byte(x, nn),
            (0x7, x, _, _) => self.add_vx_byte(x, nn),
            (0x8, x, y, 0x0) => self.load_vx_vy(x, y),
            (0x8, x, y, 0x1) => self.or_vx_vy(x, y),
            (0x8, x, y, 0x2) => self.and_vx_vy(x, y),
            (0x8, x, y, 0x3) => self.xor_vx_vy(x, y),
            (0x8, x, y, 0x4) => self.add_vx_vy(x, y),
            (0x8, x, y, 0x5) => self.sub_vx_vy(x, y),
            (0x8, x, _, 0x6) => self.shr_vx(x),
            (0x8, x, y, 0x7) => self.subn_vx_vy(x, y),
            (0x8, x, _, 0xE) => self.shl_vx(x),
            (0x9, x, y, 0x0) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i_addr(nnn),
            (0xB, _, _, _) => self.jp_v0_addr(nnn),
            (0xC, x, _, _) => self.rnd_vx_byte(x, nn),
            (0xD, x, y, n) => self.drw_vx_vy_nibble(x, y, n),
            (0xE, x, 0x9, 0xE) => self.skp_vx(x),
            (0xE, x, 0xA, 0x1) => self.sknp_vx(x),
            (0xF, x, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, x, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, x, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, x, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, x, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, x, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, x, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, x, 0x5, 0x5) => self.ld_i_vx(x),
            (0xF, x, 0x6, 0x5) => self.ld_vx_i(x),
            (_, _, _, _) => panic!("Unknown opcode {:02x?}", nibbles),
        };
    }

    fn nop(&mut self) {
        self.pc += 2;
    }

    fn cls(&mut self) {
        self.display.clear();
        self.pc += 2;
    }

    fn ret(&mut self) {
        assert!(self.sp > 0);

        self.sp -= 1;
        self.pc = self.stack[usize::from(self.sp)];
    }

    fn jp_addr(&mut self, addr: u16) {
        assert!(addr < 4095);

        if self.pc == addr {
            //panic!("Infinite loop detected. Will exit now.");
        }

        self.pc = addr
    }

    fn call_addr(&mut self, addr: u16) {
        assert!(addr < 4095);

        self.stack[usize::from(self.sp)] = self.pc + 2;
        self.sp += 1;

        self.pc = addr;
    }

    fn se_vx_byte(&mut self, reg: u8, byte: u8) {
        assert!(reg < 16);

        if self.regs[usize::from(reg)] == byte {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn sne_vx_byte(&mut self, reg: u8, byte: u8) {
        assert!(reg < 16);

        if self.regs[usize::from(reg)] != byte {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn se_vx_vy(&mut self, reg1: u8, reg2: u8) {
        assert!(reg1 < 16);
        assert!(reg2 < 16);

        if self.regs[usize::from(reg1)] == self.regs[usize::from(reg2)] {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn ld_vx_byte(&mut self, reg: u8, byte: u8) {
        assert!(reg < 16);

        self.regs[usize::from(reg)] = byte;

        self.pc += 2;
    }

    fn add_vx_byte(&mut self, reg: u8, byte: u8) {
        assert!(reg < 16);

        self.regs[usize::from(reg)] =
            self.regs[usize::from(reg)].wrapping_add(byte);

        self.pc += 2;
    }

    fn load_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        self.regs[usize::from(vx)] = self.regs[usize::from(vy)];

        self.pc += 2;
    }

    fn or_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        self.regs[usize::from(vx)] |= self.regs[usize::from(vy)];

        self.pc += 2;
    }

    fn and_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        self.regs[vx as usize] &= self.regs[vy as usize];

        self.pc += 2;
    }

    fn xor_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        self.regs[vx as usize] ^= self.regs[vy as usize];

        self.pc += 2;
    }

    fn add_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        let (new_val, overflow) =
            self.regs[vx as usize].overflowing_add(self.regs[vy as usize]);

        self.regs[vx as usize] = new_val;
        self.regs[0xF] = u8::from(overflow);

        self.pc += 2;
    }

    fn sub_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        let (new_val, overflow) =
            self.regs[vx as usize].overflowing_sub(self.regs[vy as usize]);

        self.regs[0xF] = u8::from(overflow);
        self.regs[vx as usize] = new_val;

        self.pc += 2;
    }

    fn shr_vx(&mut self, vx: u8) {
        assert!(vx < 16);

        self.regs[0xF] = self.regs[usize::from(vx)] & 0b00000001;
        self.regs[usize::from(vx)] >>= 1;

        self.pc += 2;
    }

    fn subn_vx_vy(&mut self, vx: u8, vy: u8) {
        assert!(vx < 16);
        assert!(vy < 16);

        let (new_val, overflow) =
            self.regs[usize::from(vy)].overflowing_sub(self.regs[usize::from(vx)]);

        self.regs[0xF] = u8::from(overflow);
        self.regs[vx as usize] = new_val;

        self.pc += 2;
    }

    fn shl_vx(&mut self, vx: u8) {
        assert!(vx < 16);

        self.regs[0xF] = self.regs[usize::from(vx)] & 0b10000000;
        self.regs[usize::from(vx)] <<= 1;

        self.pc += 2;
    }

    fn sne_vx_vy(&mut self, reg1: u8, reg2: u8) {
        assert!(reg1 < 16);
        assert!(reg2 < 16);

        if self.regs[usize::from(reg1)] != self.regs[usize::from(reg2)] {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn ld_i_addr(&mut self, val: u16) {
        self.i = val;

        self.pc += 2;
    }

    fn jp_v0_addr(&mut self, addr: u16) {
        self.pc = (self.regs[0] as u16) + addr;
    }

    fn rnd_vx_byte(&mut self, reg: u8, byte: u8) {
        assert!(reg < 16);

        self.regs[usize::from(reg)] = rand::random::<u8>() & byte;

        self.pc += 2;
    }

    fn drw_vx_vy_nibble(&mut self, reg1: u8, reg2: u8, nibble: u8) {
        assert!(reg1 < 16);
        assert!(reg2 < 16);

        let x = self.regs[usize::from(reg1)] % 64;
        let y = self.regs[usize::from(reg2)] % 32;

        let mut pixel_erased = false;
        for yy in 0..nibble {
            let row_val = self.ram.read(self.i + u16::from(yy));
            for xx in 0..8 {
                if x + xx >= 64 || y + yy >= 32 {
                    continue;
                }

                let pixel_flip = (row_val & (0x1 << (7 - xx))) != 0;
                let old_pixel = self.display.vram_get(x + xx, y + yy);

                pixel_erased |= pixel_flip && old_pixel; // pixel erased when old pixel and pixel will be flipped
                self.display.vram_set(x + xx, y + yy, old_pixel ^ pixel_flip);
            }
        }

        self.regs[0xF] = u8::from(pixel_erased);

        self.pc += 2;
    }

    fn skp_vx(&mut self, vx: u8) {
        if self.keypad.button_is_pressed(self.regs[usize::from(vx)]) {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn sknp_vx(&mut self, vx: u8) {
        if !self.keypad.button_is_pressed(self.regs[usize::from(vx)]) {
            self.pc += 2;
        }

        self.pc += 2;
    }

    fn ld_vx_dt(&mut self, vx: u8) {
        assert!(vx < 16);

        self.regs[usize::from(vx)] = self.dt;

        self.pc += 2;
    }

    fn ld_vx_k(&mut self, vx: u8) {
        assert!(vx < 16);

        self.regs[usize::from(vx)] = self.keypad.next_button_pressed();

        self.pc += 2;
    }

    fn ld_dt_vx(&mut self, vx: u8) {
        assert!(vx < 16);

        self.dt = self.regs[usize::from(vx)];

        self.pc += 2;
    }

    fn ld_st_vx(&mut self, vx: u8) {
        assert!(vx < 16);

        self.st = self.regs[usize::from(vx)];

        self.pc += 2;
    }

    fn add_i_vx(&mut self, vx: u8) {
        assert!(vx < 16);

        self.i += self.regs[usize::from(vx)] as u16;

        self.pc += 2;
    }

    fn ld_f_vx(&mut self, vx: u8) {
        println!("Getting font for: {} at: {}", self.regs[usize::from(vx)], self.regs[usize::from(vx)] * 5);
        self.i = u16::from(self.regs[usize::from(vx)] * 5);

        self.pc += 2;
    }

    fn ld_b_vx(&mut self, vx: u8) {
        assert!(self.i < 4095);
        assert!(vx < 16);

        let val = self.regs[usize::from(vx)];

        self.ram.set(self.i, val / 100);
        self.ram.set(self.i + 1, (val % 100) / 10);
        self.ram.set(self.i + 2, val % 10);

        self.pc += 2;
    }

    fn ld_i_vx(&mut self, vx: u8) {
        assert!(vx < 16);
        assert!(self.i + (vx as u16) < 4095);

        for i in 0..=vx {
            self.ram.set(self.i + i as u16, self.regs[usize::from(i)]);
        }

        self.pc += 2;
    }

    fn ld_vx_i(&mut self, vx: u8) {
        assert!(vx < 16);
        assert!(self.i + (vx as u16) < 4095);

        for i in 0..=vx {
            self.regs[usize::from(i)] = self.ram.read(self.i + (i as u16));
        }

        self.pc += 2;
    }
}
