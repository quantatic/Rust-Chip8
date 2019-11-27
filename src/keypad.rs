use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub enum KeypadButton {
    Num1,
    Num2,
    Num3,
    C,
    Num4,
    Num5,
    Num6,
    D,
    Num7,
    Num8,
    Num9,
    E,
    A,
    Num0,
    B,
    F
}

pub struct Keypad {
    events: sdl2::EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {

        Keypad {
            events: sdl_context.event_pump().unwrap()
        }
    }

    pub fn keypad_button_is_pressed(&mut self, key: KeypadButton) -> bool {
        self.events.pump_events();
        self.events.keyboard_state()
            .is_scancode_pressed(Keypad::keypad_to_scancode(key))
    }

    fn keypad_to_scancode(key: KeypadButton) -> sdl2::keyboard::Scancode {
        match key {
            KeypadButton::Num1 => Scancode::Num1,
            KeypadButton::Num2 => Scancode::Num2,
            KeypadButton::Num3 => Scancode::Num3,
            KeypadButton::C => Scancode::Num4,
            KeypadButton::Num4 => Scancode::Q,
            KeypadButton::Num5 => Scancode::W,
            KeypadButton::Num6 => Scancode::E,
            KeypadButton::D => Scancode::R,
            KeypadButton::Num7 => Scancode::A,
            KeypadButton::Num8 => Scancode::S,
            KeypadButton::Num9 => Scancode::D,
            KeypadButton::E => Scancode::F,
            KeypadButton::A => Scancode::Z,
            KeypadButton::Num0 => Scancode::X,
            KeypadButton::B => Scancode::C,
            KeypadButton::F => Scancode::V,
        }
    }

    fn scancode_to_keypad(code: Scancode) -> Option<KeypadButton> {
        match code {
            Scancode::Num1 => Some(KeypadButton::Num1),
            Scancode::Num2 => Some(KeypadButton::Num2),
            Scancode::Num3 => Some(KeypadButton::Num3),
            Scancode::Num4 => Some(KeypadButton::C),
            Scancode::Q => Some(KeypadButton::Num4),
            Scancode::W => Some(KeypadButton::Num5),
            Scancode::E => Some(KeypadButton::Num6),
            Scancode::R => Some(KeypadButton::D),
            Scancode::A => Some(KeypadButton::Num7),
            Scancode::S => Some(KeypadButton::Num8),
            Scancode::D => Some(KeypadButton::Num9),
            Scancode::F => Some(KeypadButton::E),
            Scancode::Z => Some(KeypadButton::A),
            Scancode::X => Some(KeypadButton::Num0),
            Scancode::C => Some(KeypadButton::B),
            Scancode::V => Some(KeypadButton::F),
            _ => None,
        }
    }
}
