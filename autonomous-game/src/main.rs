use macroquad::{prelude::*, Error};
use macroquad_tiled as tiled;

const SPRITE_SIZE: f32 = 48.0;
const ANIMATION_SPEED: f32 = 0.1;

struct Player {
    position: Vec2,
    texture: Texture2D,
    animation_frame: usize,
    frame_timer: f32,
    facing: Direction,
    is_moving: bool,
    map_bounds: Rect,
    target_position: Option<Vec2>,
    target_effect_timer: f32,
    wave_active: bool,
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
            map_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            target_position: None,
            target_effect_timer: 0.0,
            wave_active: false,
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
    }

    fn update(&mut self, dt: f32) {
        let speed = 200.0;
        let mut movement = Vec2::ZERO;
        self.is_moving = false;

        // Handle keyboard input
        if is_key_down(KeyCode::Right) {
            movement.x += 1.0;
            self.facing = Direction::Right;
            self.is_moving = true;
            self.target_position = None; // Cancel mouse movement when using keyboard
        }
        if is_key_down(KeyCode::Left) {
            movement.x -= 1.0;
            self.facing = Direction::Left;
            self.is_moving = true;
            self.target_position = None;
        }
        if is_key_down(KeyCode::Up) {
            movement.y -= 1.0;
            self.facing = Direction::Up;
            self.is_moving = true;
            self.target_position = None;
        }
        if is_key_down(KeyCode::Down) {
            movement.y += 1.0;
            self.facing = Direction::Down;
            self.is_moving = true;
            self.target_position = None;
        }

        // Handle mouse movement if we have a target position
        if let Some(target) = self.target_position {
            let to_target = target - self.position;
            let distance = to_target.length();

            // If we're close enough to the target, stop moving
            if distance < 2.0 {
                self.target_position = None;
            } else {
                movement = to_target.normalize();
                self.is_moving = true;

                // Update facing direction based on movement
                if movement.x.abs() > movement.y.abs() {
                    if movement.x > 0.0 {
                        self.facing = Direction::Right;
                    } else {
                        self.facing = Direction::Left;
                    }
                } else {
                    if movement.y > 0.0 {
                        self.facing = Direction::Down;
                    } else {
                        self.facing = Direction::Up;
                    }
                }
            }
        }

        // Apply movement
        if movement.length() > 0.0 {
            movement = movement.normalize();
            self.position += movement * speed * dt;
        }

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
async fn main(){
    // Load textures
    let grass_texture = load_texture("./assets/Grass.png").await.unwrap();
    let hill_texture = load_texture("./assets/Hills.png").await.unwrap();
    let water_texture = load_texture("./assets/Water.png").await.unwrap();

    let mut player = Player::new().await;

    // Create a simple Tiled map JSON with just grass tiles
    let tiled_map_json = load_string("assets/map.json").await.unwrap();

    // Load the map
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
            ("Grass.png", grass_texture),
            ("Water.png", water_texture),
            ("Hills.png", hill_texture),
        ],
        &[],
    ).unwrap();

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

    loop {
        clear_background(WHITE);

        // Handle mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = Vec2::new(mouse_position().0, mouse_position().1);

            // Check if click is within map bounds
            if map_bounds.contains(mouse_position) {
                player.target_position = Some(mouse_position);
                player.target_effect_timer = 0.0;
                player.wave_active = true;
            }
        }

        tiled_map.draw_tiles(
            "Ocean",
            Rect::new(
                screen_width() / 2.0 - map_width / 2.0,
                screen_height() / 2.0 - map_height / 2.0,
                map_width,
                map_height,
            ),
            None,
        );

        tiled_map.draw_tiles(
            "land",
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

        // Draw target indicator if exists
        if let Some(target) = player.target_position {
            if player.wave_active {
                // Update wave timer
                player.target_effect_timer += get_frame_time() * 2.0;

                // Create single splash effect
                let wave_time = player.target_effect_timer;
                let size = 15.0 * wave_time; // Grow from 0 to 20
                let alpha = 0.8 * (1.0 - wave_time); // Fade out as it grows

                // Only draw if alpha is still visible
                if alpha > 0.0 {
                    draw_circle_lines(
                        target.x,
                        target.y,
                        size,
                        2.0, // line thickness
                        Color::new(1.0, 1.0, 1.0, alpha),
                    );

                    // Inner wave
                    let inner_size = size * 0.5;
                    draw_circle_lines(
                        target.x,
                        target.y,
                        inner_size,
                        1.5, // slightly thinner
                        Color::new(1.0, 1.0, 1.0, alpha),
                    );
                } else {
                    player.wave_active = false;
                }
            }
        }

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, BLACK);

        next_frame().await;
    }
}
