use macroquad::prelude::*;

#[macroquad::main("My First Game")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        draw_text("Hello, Macroquad!", 20.0, 40.0, 30.0, WHITE);

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
