use macroquad::prelude::*;
use macroquad_tiled as tiled;

const SPRITE_SIZE: f32 = 48.0;
const ANIMATION_SPEED: f32 = 0.1;

struct Resources {
    grass_texture: Texture2D,
    hills_texture: Texture2D,
    water_texture: Texture2D,
    wooden_house_wall_texture: Texture2D,
    tiled_map_json: String,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        // Load texture
        let grass_texture = load_texture("./assets/Grass.png").await?;
        let hills_texture = load_texture("./assets/Hills.png").await?;
        let water_texture = load_texture("./assets/Water.png").await?;
        let wooden_house_wall_texture = load_texture("./assets/WoodenHouseWall.png").await?;
        // Load sounds
        // Load image

        // Create a simple Tiled map JSON
        let tiled_map_json = load_string("assets/map.json").await.unwrap();
        let resources = Resources {
            grass_texture,
            hills_texture,
            water_texture,
            wooden_house_wall_texture,
            tiled_map_json,
        };

        Ok(resources)
    }
}

#[derive(Debug)]
struct GameCamera {
    position: Vec2,
    viewport_size: Vec2,
    zoom: f32,
}

impl GameCamera {
    fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            viewport_size: Vec2::new(screen_width(), screen_height()),
            zoom: 2.0,
        }
    }

    fn update_viewport_size(&mut self) {
        self.viewport_size = Vec2::new(screen_width(), screen_height());
    }

    fn update(&mut self, target_position: Vec2) {
        self.position = target_position;
    }

    fn world_to_screen(&self, world_position: Vec2) -> Vec2 {
        Vec2::new(
            (world_position.x - self.position.x) * self.zoom + self.viewport_size.x / 2.0,
            (world_position.y - self.position.y) * self.zoom + self.viewport_size.y / 2.0,
        )
    }

    fn screen_to_world(&self, screen_position: Vec2) -> Vec2 {
        Vec2::new(
            (screen_position.x - self.viewport_size.x / 2.0) / self.zoom + self.position.x,
            (screen_position.y - self.viewport_size.y / 2.0) / self.zoom + self.position.y,
        )
    }
}

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
                } else if movement.y > 0.0 {
                    self.facing = Direction::Down;
                } else {
                    self.facing = Direction::Up;
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

    fn draw_player(&self, camera: &GameCamera) {
        let player_screen_pos = camera.world_to_screen(self.position);
        draw_texture_ex(
            &self.texture,
            player_screen_pos.x - (SPRITE_SIZE * camera.zoom) / 2.0,
            player_screen_pos.y - (SPRITE_SIZE * camera.zoom) / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    self.animation_frame as f32 * SPRITE_SIZE,
                    self.facing.clone() as i32 as f32 * SPRITE_SIZE,
                    SPRITE_SIZE,
                    SPRITE_SIZE,
                )),
                dest_size: Some(Vec2::new(
                    SPRITE_SIZE * camera.zoom,
                    SPRITE_SIZE * camera.zoom,
                )),
                ..Default::default()
            },
        );
    }

    fn draw_wave_effect(&mut self, camera: &GameCamera) {
        if let Some(target) = self.target_position {
            if self.wave_active {
                // Update wave timer
                self.target_effect_timer += get_frame_time() * 2.0;

                // Create single splash effect
                let wave_time = self.target_effect_timer;
                let size = 15.0 * wave_time; // Grow from 0 to 20
                let alpha = 0.8 * (1.0 - wave_time); // Fade out as it grows

                // Only draw if alpha is still visible
                if alpha > 0.0 {
                    let target_screen_pos = camera.world_to_screen(target);
                    draw_circle_lines(
                        target_screen_pos.x,
                        target_screen_pos.y,
                        size,
                        2.0, // line thickness
                        Color::new(1.0, 1.0, 1.0, alpha),
                    );

                    // Inner wave
                    let inner_size = size * 0.5;
                    draw_circle_lines(
                        target_screen_pos.x,
                        target_screen_pos.y,
                        inner_size,
                        1.5, // slightly thinner
                        Color::new(1.0, 1.0, 1.0, alpha),
                    );
                } else {
                    self.wave_active = false;
                }
            }
        };
    }
}

#[macroquad::main("Grass Tile Map")]
async fn main() -> Result<Resources, macroquad::Error> {
    let resources = Resources::new().await?;

    let mut player = Player::new().await;
    let mut camera = GameCamera::new();

    // Load the map
    let tiled_map = tiled::load_map(
        &resources.tiled_map_json,
        &[
            ("Grass.png", resources.grass_texture),
            ("Water.png", resources.water_texture),
            ("Hills.png", resources.hills_texture),
            ("WoodenHouseWall.png", resources.wooden_house_wall_texture),
        ],
        &[],
    )
    .unwrap();

    for (x, y, tile) in tiled_map.tiles("Ocean", None) {
        println!("{}, {}, {:#?}", x, y, tile);
    }

    // set land area as boundary;
    let map_width = tiled_map.raw_tiled_map.width as f32 * tiled_map.raw_tiled_map.tilewidth as f32;
    let map_height =
        tiled_map.raw_tiled_map.height as f32 * tiled_map.raw_tiled_map.tileheight as f32;

    // Calculate the bounds for the land area (32x32 tiles in center)
    let tile_size = 16.0;
    let total_tiles = 60; // total map size in tiles
    let land_tiles = 32; // land area size in tiles

    // Calculate the offset to center the land area
    let offset = (total_tiles - land_tiles) as f32 * tile_size / 2.0;
    // Define the map bounds for just the land area
    let map_bounds = Rect::new(
        offset,                        // x start
        offset,                        // y start
        land_tiles as f32 * tile_size, // width (32 tiles * 16 pixels)
        land_tiles as f32 * tile_size, // height (32 tiles * 16 pixels)
    );

    // Set player's map bounds
    player.set_map_bounds(map_bounds);

    loop {
        clear_background(WHITE);
        camera.update_viewport_size();

        // Handle mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let screen_position = Vec2::new(mouse_position().0, mouse_position().1);
            let world_position = camera.screen_to_world(screen_position);

            // Check if click is within map bounds
            if map_bounds.contains(world_position) {
                player.target_position = Some(world_position);
                player.target_effect_timer = 0.0;
                player.wave_active = true;
            }
        }

        // Update player
        player.update(get_frame_time());

        // Update camera to follow player
        camera.update(player.position);

        // Calculate camera offset for drawing
        // Calculate camera offset for drawing
        let camera_offset = Vec2::new(
            screen_width() / 2.0 - camera.position.x * camera.zoom,
            screen_height() / 2.0 - camera.position.y * camera.zoom,
        );

        // Draw layers
        tiled_map.draw_tiles(
            "Ocean",
            Rect::new(
                camera_offset.x,
                camera_offset.y,
                map_width * camera.zoom,
                map_height * camera.zoom,
            ),
            None,
        );
        tiled_map.draw_tiles(
            "Land",
            Rect::new(
                camera_offset.x,
                camera_offset.y,
                map_width * camera.zoom,
                map_height * camera.zoom,
            ),
            None,
        );
        tiled_map.draw_tiles(
            "Floor",
            Rect::new(
                camera_offset.x,
                camera_offset.y,
                map_width * camera.zoom,
                map_height * camera.zoom,
            ),
            None,
        );
        tiled_map.draw_tiles(
            "House",
            Rect::new(
                camera_offset.x,
                camera_offset.y,
                map_width * camera.zoom,
                map_height * camera.zoom,
            ),
            None,
        );

        // Draw player at center of screen
        player.draw_player(&camera);

        // Draw target indicator if exists
        player.draw_wave_effect(&camera);

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, BLACK);

        next_frame().await;
    }
}
