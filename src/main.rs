mod cpu;
mod display;
mod keypad;
mod ram;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::ram::Ram;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl_context, 20);
    let mut keypad = Keypad::new(&sdl_context);
    let mut ram = Ram::new();
    let mut cpu = Cpu::new(&mut ram, &mut display, &mut keypad);

    cpu.load_rom_into_ram("roms/keypad.ch8").expect("Failed to load CPU rom");

    cpu.disas();
    for _ in 0..5 {
        println!();
    }

    loop {
        cpu.tick();
        std::thread::sleep(std::time::Duration::new(0, 5_000_000));
    }
}
