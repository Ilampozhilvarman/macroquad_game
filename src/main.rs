use std::io::SeekFrom::Current;

use macroquad::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
    pos: Pos,
    moving: bool,
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            pos: Pos { x, y },
            moving: false,
        }
    }

    fn left(&mut self, speed: f32) {
        if !self.moving {
            self.pos.x -= speed;
        }
    }

    fn right(&mut self, speed: f32) {
        if !self.moving {
            self.pos.x += speed;
        }
    }

    fn start_moving(&mut self) {
        self.moving = true;
    }

    fn stop_moving(&mut self) {
        self.moving = false;
    }

    fn clamp(&mut self, screen_width: f32, screen_height: f32) {
        self.pos.x = self.pos.x.clamp(0.0, screen_width - self.width);
        self.pos.y = self.pos.y.clamp(0.0, screen_height - self.height);
    }

    fn draw(&mut self, color: Color) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, color);
    }
}

struct Board {
    rectangles: Vec<Rectangle>,
    score: u32,
    current_rect: Option<Rectangle>,
}

fn draw_text_h_centered(text: &str, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dims.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

//Constants
const MOVEMENT_SPEED: f32 = 20.0;
const START_RECT_DIV_W: f32 = 10.0;
const START_RECT_DIV_H: f32 = 10.0;
const COLOR: Color = GREEN;

#[macroquad::main("Stacking Tower")]
async fn main() {
    //Constants, but have to be declared at runtime
    let screen_w: f32 = screen_width();
    let screen_h: f32 = screen_height();

    //Constant based
    let rect_width: f32 = screen_w / START_RECT_DIV_W;
    let rect_height: f32 = screen_h / START_RECT_DIV_H;
    let current_rect: Rect = Rect {
        x: screen_w / 2.0,
        y: screen_h / 2.0,
        w: rect_width,
        h: rect_height,
    };

    //Variables
    let mut game_manager: Board = Board {
        rectangles: Vec::new(),
        score: 0,
        current_rect: None,
    };

    set_fullscreen(true);

    loop {
        //Moving square code
        current_rect.left();
        clear_background(DARKGRAY);

        draw_text_h_centered("Hello, Macroquad!", 60.0, 40.0, WHITE);

        for rect in &mut game_manager.rectangles {
            rect.draw(COLOR);
        }

        next_frame().await;
    }
}
