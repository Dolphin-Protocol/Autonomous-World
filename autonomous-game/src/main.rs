use autonomous_game::{greet, log, log_player_position};
use macroquad::prelude::*;
use macroquad_tiled as tiled;
use wasm_bindgen::prelude::*;

const SPRITE_SIZE: f32 = 48.0;
const ANIMATION_SPEED: f32 = 0.1;

static mut SHARED_STATE: GameState = GameState { speed: 0.0 };

pub struct GameState {
    pub speed: f32,
}

#[wasm_bindgen]
pub fn set_player_speed(speed: f32) {
    unsafe {
        SHARED_STATE.speed = speed;
    }
}

#[wasm_bindgen]
pub fn get_player_speed() -> f32 {
    unsafe { SHARED_STATE.speed }
}

struct Player {
    position: Vec2,
    texture: Texture2D,
    animation_frame: usize,
    frame_timer: f32,
    facing: Direction,
    is_moving: bool,
    map_bounds: Rect, // Add map bounds
}

#[derive(PartialEq, Clone)]
enum Direction {
    Down = 0,
    Up = 1,
    Left = 2,
    Right = 3,
}

impl Player {
    async fn new() -> Self {
        let texture = load_texture("assets/BasicCharacterSpritesheet.png")
            .await
            .unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            texture,
            animation_frame: 0,
            frame_timer: 0.0,
            facing: Direction::Down,
            is_moving: false,
            map_bounds: Rect::new(0.0, 0.0, 0.0, 0.0), // Will be set later
        }
    }

    fn set_map_bounds(&mut self, bounds: Rect) {
        self.map_bounds = bounds;
    }

    fn clamp_position(&mut self) {
        // Account for character sprite size when clamping
        let half_sprite = SPRITE_SIZE / 2.0;

        self.position.x = self.position.x.clamp(
            self.map_bounds.x + half_sprite,
            self.map_bounds.x + self.map_bounds.w - half_sprite,
        );

        self.position.y = self.position.y.clamp(
            self.map_bounds.y + half_sprite,
            self.map_bounds.y + self.map_bounds.h - half_sprite,
        );

        // testing
        if self.position.x == self.map_bounds.x + half_sprite {
            log("to the left end");
            log_player_position(self.position.x, self.position.y);
        };

        if self.position.x == self.map_bounds.x + self.map_bounds.w - half_sprite {
            log("to the right end");
            log_player_position(self.position.x, self.position.y);
        };
    }

    fn update(&mut self, dt: f32) {
        let mut speed = 200.0;
        unsafe {
            if SHARED_STATE.speed != 0.0 {
                speed = SHARED_STATE.speed;
            };
        };
        let mut movement = Vec2::ZERO;
        self.is_moving = false;

        if is_key_down(KeyCode::Right) {
            movement.x += 1.0;
            self.facing = Direction::Right;
            self.is_moving = true;
        }
        if is_key_down(KeyCode::Left) {
            movement.x -= 1.0;
            self.facing = Direction::Left;
            self.is_moving = true;
        }
        if is_key_down(KeyCode::Up) {
            movement.y -= 1.0;
            self.facing = Direction::Up;
            self.is_moving = true;
        }
        if is_key_down(KeyCode::Down) {
            movement.y += 1.0;
            self.facing = Direction::Down;
            self.is_moving = true;
        }

        // Normalize diagonal movement
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        self.position += movement * speed * dt;

        // Update animation
        if self.is_moving {
            self.frame_timer += dt;
            if self.frame_timer >= ANIMATION_SPEED {
                self.frame_timer = 0.0;
                self.animation_frame = (self.animation_frame + 1) % 4;
            }
        } else {
            self.animation_frame = 0;
        }

        self.clamp_position();
    }

    fn draw(&self) {
        // Calculate source rectangle from sprite sheet
        let source = Rect::new(
            self.animation_frame as f32 * SPRITE_SIZE,
            self.facing.clone() as i32 as f32 * SPRITE_SIZE,
            SPRITE_SIZE,
            SPRITE_SIZE,
        );

        // Calculate destination rectangle
        let dest = Rect::new(
            self.position.x - SPRITE_SIZE / 2.0,
            self.position.y - SPRITE_SIZE / 2.0,
            SPRITE_SIZE,
            SPRITE_SIZE,
        );

        // Draw the sprite
        draw_texture_ex(
            &self.texture,
            dest.x,
            dest.y,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("Grass Tile Map")]
async fn main() {
    // Load your grass set_map_bounds
    let grass_texture = load_texture("./assets/Grass.png").await.unwrap();
    let mut player = Player::new().await;

    // Create a simple Tiled map JSON with just grass tiles
    let tiled_map_json = load_string("assets/world.json").await.unwrap();

    // Load the map
    let tiled_map = tiled::load_map(&tiled_map_json, &[("Grass.png", grass_texture)], &[]).unwrap();

    // retrieve map size
    let map_width = tiled_map.raw_tiled_map.width as f32 * tiled_map.raw_tiled_map.tilewidth as f32;
    let map_height =
        tiled_map.raw_tiled_map.height as f32 * tiled_map.raw_tiled_map.tileheight as f32;

    let map_bounds = Rect::new(
        screen_width() / 2.0 - map_width / 2.0,
        screen_height() / 2.0 - map_height / 2.0,
        map_width,
        map_height,
    );

    // updae player's value
    player.set_map_bounds(map_bounds);

    greet("game start");

    loop {
        clear_background(WHITE);

        tiled_map.draw_tiles(
            "Tile Layer 1",
            Rect::new(
                screen_width() / 2.0 - map_width / 2.0,
                screen_height() / 2.0 - map_height / 2.0,
                map_width,
                map_height,
            ),
            None,
        );

        // Update and draw player
        player.update(get_frame_time());
        player.draw();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, BLACK);

        next_frame().await;
    }
}
