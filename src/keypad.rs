use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Keypad {
    events: sdl2::EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {

        Keypad {
            events: sdl_context.event_pump().unwrap()
        }
    }

    pub fn check_for_exit(&mut self) {
        for event in self.events.poll_iter() {
            if let Event::Quit{ .. } = event {
                panic!("Exiting!");
            }
        }
    }

    pub fn button_is_pressed(&mut self, key: u8) -> bool {
        self.events.pump_events();
        match Keypad::keypad_to_scancode(key) {
            Some(code) => self.events.keyboard_state().is_scancode_pressed(code),
            None => {
                panic!("Unrecognized keypad button queried: {}", key)
            }
        }
    }

    pub fn next_button_pressed(&mut self) -> u8 {
        loop {
            self.check_for_exit();
            for code in self.events.keyboard_state().pressed_scancodes() {
                if let Some(keypad) = Keypad::scancode_to_keypad(code) {
                    return keypad;
                }
            }
        }
    }

    fn keypad_to_scancode(key: u8) -> Option<sdl2::keyboard::Scancode> {
        match key {
            0x1 => Some(Scancode::Num1),
            0x2 => Some(Scancode::Num2),
            0x3 => Some(Scancode::Num3),
            0xC => Some(Scancode::Num4),
            0x4 => Some(Scancode::Q),
            0x5 => Some(Scancode::W),
            0x6 => Some(Scancode::E),
            0xD => Some(Scancode::R),
            0x7 => Some(Scancode::A),
            0x8 => Some(Scancode::S),
            0x9 => Some(Scancode::D),
            0xE => Some(Scancode::F),
            0xA => Some(Scancode::Z),
            0x0 => Some(Scancode::X),
            0xB => Some(Scancode::C),
            0xF => Some(Scancode::V),
            _ => None,
        }
    }

    fn scancode_to_keypad(code: Scancode) -> Option<u8> {
        match code {
            Scancode::Num1 => Some(0x1),
            Scancode::Num2 => Some(0x2),
            Scancode::Num3 => Some(0x3),
            Scancode::Num4 => Some(0xC),
            Scancode::Q => Some(0x4),
            Scancode::W => Some(0x5),
            Scancode::E => Some(0x6),
            Scancode::R => Some(0xD),
            Scancode::A => Some(0x7),
            Scancode::S => Some(0x8),
            Scancode::D => Some(0x9),
            Scancode::F => Some(0xE),
            Scancode::Z => Some(0xA),
            Scancode::X => Some(0x0),
            Scancode::C => Some(0xB),
            Scancode::V => Some(0xF),
            _ => None,
        }
    }
}
