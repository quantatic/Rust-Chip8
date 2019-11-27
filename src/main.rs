mod cpu;
mod display;
mod keypad;
mod ram;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::ram::Ram;

use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl_context, 20);
    let mut keypad = Keypad::new(&sdl_context);
    let mut ram = Ram::new();
    let mut cpu = Cpu::new(&mut ram, &mut display);

    //cpu.load_rom_into_ram("roms/jumping.ch8").expect("Failed to load CPU rom");
    cpu.load_rom_into_ram("roms/picture.ch8").expect("Failed to load CPU rom");
    //cpu.load_rom_into_ram("roms/invaders.ch8").expect("Failed to load CPU rom");

    cpu.disas();
    for _ in 0..5 {
        println!();
    }

    loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            if let Event::Quit{ .. } = event {
                panic!("Quitting!");
            }
        };

        cpu.tick();
        std::thread::sleep(std::time::Duration::new(0, 50_000_000));
    }
}
