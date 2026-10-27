use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Block {
    width: f32,
    height: f32,
    pos: Pos,
    direction: f32, // 1.0 for right, -1.0 for left
}

impl Block {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            pos: Pos { x, y },
            direction: 1.0,
        }
    }

    fn update(&mut self, screen_w: f32, speed: f32) {
        self.pos.x += self.direction * speed;
        if self.pos.x + self.width >= screen_w {
            self.direction = -1.0;
        } else if self.pos.x <= 0.0 {
            self.direction = 1.0;
        }
    }

    fn overlaps_x(&self, other: &Block) -> (f32, f32) {
        let left = self.pos.x.max(other.pos.x);
        let right = (self.pos.x + self.width).min(other.pos.x + other.width);
        (left, right)
    }
}

fn draw_text_centered(text: &str, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dims.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

const BLOCK_HEIGHT_RATIO: f32 = 20.0;
const START_WIDTH_RATIO: f32 = 3.0;
const INITIAL_SPEED: f32 = 4.0;
const SPEED_INCREMENT: f32 = 0.2;

#[macroquad::main("Stacking Tower")]
async fn main() {
    let mut camera_y: f32 = 0.0;
    let mut speed: f32 = INITIAL_SPEED;
    let mut score: u32 = 0;
    let mut game_over: bool = false;

    let screen_w = screen_width();
    let screen_h = screen_height();

    let block_height = screen_h / BLOCK_HEIGHT_RATIO;
    let start_width = screen_w / START_WIDTH_RATIO;

    let mut stack: Vec<Block> = Vec::new();
    let base_y = screen_h - block_height - 10.0;
    stack.push(Block {
        width: start_width,
        height: block_height,
        pos: Pos {
            x: (screen_w - start_width) / 2.0,
            y: base_y,
        },
        direction: 1.0,
    });

    let mut current = Block {
        width: start_width,
        height: block_height,
        pos: Pos {
            x: 0.0,
            y: base_y - block_height,
        },
        direction: 1.0,
    };

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        if !game_over {
            current.update(screen_w, speed);

            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                let last = stack[stack.len() - 1];
                let (overlap_left, overlap_right) = current.overlaps_x(&last);

                if overlap_right <= overlap_left {
                    game_over = true;
                } else {
                    let overlap_width = overlap_right - overlap_left;
                    let placed = Block {
                        width: overlap_width,
                        height: block_height,
                        pos: Pos {
                            x: overlap_left,
                            y: current.pos.y,
                        },
                        direction: 1.0,
                    };
                    stack.push(placed);
                    score += 1;
                    speed += SPEED_INCREMENT;

                    if stack.len() as f32 * block_height > screen_h * 0.6 {
                        camera_y += block_height;
                    }

                    current = Block {
                        width: overlap_width,
                        height: block_height,
                        pos: Pos {
                            x: 0.0,
                            y: current.pos.y - block_height,
                        },
                        direction: 1.0,
                    };
                }
            }
        } else if is_key_pressed(KeyCode::R) || is_mouse_button_pressed(MouseButton::Left) {
            stack.clear();
            speed = INITIAL_SPEED;
            score = 0;
            camera_y = 0.0;
            game_over = false;
            let base_y = screen_h - block_height - 10.0;
            stack.push(Block {
                width: start_width,
                height: block_height,
                pos: Pos {
                    x: (screen_w - start_width) / 2.0,
                    y: base_y,
                },
                direction: 1.0,
            });
            current = Block {
                width: start_width,
                height: block_height,
                pos: Pos {
                    x: 0.0,
                    y: base_y - block_height,
                },
                direction: 1.0,
            };
        }

        let camera_offset = camera_y;
        for block in &stack {
            draw_rectangle(
                block.pos.x,
                block.pos.y - camera_offset,
                block.width,
                block.height,
                Color::new(0.2, 0.8, 0.4, 1.0),
            );
        }
        if !game_over {
            draw_rectangle(
                current.pos.x,
                current.pos.y - camera_offset,
                current.width,
                current.height,
                Color::new(0.4, 0.9, 0.6, 1.0),
            );
        }

        draw_text_centered(&format!("Score: {}", score), 40.0, 40.0, WHITE);
        if !game_over {
            draw_text_centered(
                "Press SPACE or CLICK to stack",
                screen_h - 40.0,
                24.0,
                WHITE,
            );
        } else {
            draw_text_centered("GAME OVER", screen_h / 2.0 - 20.0, 60.0, RED);
            draw_text_centered(
                &format!("Final Score: {}", score),
                screen_h / 2.0 + 30.0,
                36.0,
                WHITE,
            );
            draw_text_centered(
                "Press R or CLICK to restart",
                screen_h / 2.0 + 80.0,
                28.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
