use macroquad::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
    pos: Pos,
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            pos: Pos { x, y },
        }
    }

    fn left(&mut self, speed: f32) {
        self.pos.x -= speed;
    }

    fn right(&mut self, speed: f32) {
        self.pos.x += speed;
    }

    fn clamp(&mut self, screen_width: f32, screen_height: f32) {
        self.pos.x = self.pos.x.clamp(0.0, screen_width - self.width);
        self.pos.y = self.pos.y.clamp(0.0, screen_height - self.height);
    }
}

struct Board {
    rectangles: Vec<Rectangle>,
    score: u32,
    current_rect: Rectangle,
}

//Constants
const MOVEMENT_SPEED: f32 = 20.0;

#[macroquad::main("My First Game")]
async fn main() {
    //Constants, but have to be declared at runtime
    let screen_w: f32 = screen_width();
    let screen_h: f32 = screen_height();

    set_fullscreen(true);

    loop {
        clear_background(DARKGRAY);

        draw_text("Hello, Macroquad!", 40.0, 60.0, 40.0, WHITE);

        draw_rectangle(
            screen_width() / 2.0 - 50.0,
            screen_height() / 2.0 - 50.0,
            100.0,
            100.0,
            GREEN,
        );

        next_frame().await;
    }
}
