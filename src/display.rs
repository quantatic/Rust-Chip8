use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Display {
    canvas: Canvas<Window>,
    vram: [[bool; 32]; 64], //access as vram[x][y]
    changed: bool,
    scale: u8,
}

impl Display {
    pub fn new(scale: u8) -> Self {
        let vram = [[false; 32]; 64];
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8 Terminal Window", 64 * u32::from(scale), 32 * u32::from(scale))
        .position_centered()
        .build()
        .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            vram,
            changed: true,
            scale
        }
    }

    pub fn redraw(&mut self) {
        if !self.changed {
            return;
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        for y in 0u8..32 {
            for x in 0u8..64 {
                if self.vram_get(x, y) {
                    self.canvas.fill_rect(Rect::new(i32::from(x) * i32::from(self.scale), i32::from(y) * i32::from(self.scale), u32::from(self.scale), u32::from(self.scale))).unwrap();
                }
            }
        }

        self.canvas.present();
        self.changed = false;
    }

    pub fn clear(&mut self) {
        self.vram = [[false; 32]; 64];
    }

    pub fn vram_get(&self, x: u8, y: u8) -> bool {
        self.vram[usize::from(x)][usize::from(y)]
    }

    pub fn vram_set(&mut self, x: u8, y: u8, new: bool) {
        self.vram[usize::from(x)][usize::from(y)] = new;
        self.changed = true;
    }
}
