mod cpu;
mod display;
mod ram;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::ram::Ram;

fn main() {
    let mut display = Display::new(20);
    let mut ram = Ram::new();
    let mut cpu = Cpu::new(&mut ram, &mut display);

    //cpu.load_rom_into_ram("roms/jumping.ch8").expect("Failed to load CPU rom");
    cpu.load_rom_into_ram("roms/picture.ch8").expect("Failed to load CPU rom");
    //cpu.load_rom_into_ram("roms/invaders.ch8").expect("Failed to load CPU rom");

    cpu.disas();
    for _ in 0..5 {
        println!();
    }

    for _ in 0..500 {
        cpu.tick();
        std::thread::sleep(std::time::Duration::new(0, 30_000_000));
    }
}
