
use autonomous_game::{console_log, get_state};
mod animated_gif;
mod door;

use animated_gif::AnimatedBackground;
use door::Door;
use macroquad::prelude::*;
use macroquad::ui::Skin;
use macroquad::ui::{hash, root_ui};
use macroquad_platformer::*;
use macroquad_tiled::{self as tiled, Map};

const SPRITE_SIZE: f32 = 48.0;
const ANIMATION_SPEED: f32 = 0.1;


enum GameState {
    MainMenu,
    Playing,
}

struct Resources {
    // texture
    grass_texture: Texture2D,
    water_texture: Texture2D,
    hills_texture: Texture2D,
    wooden_house_wall_texture: Texture2D,
    door_animation_texture: Texture2D,
    basic_furniture_texture: Texture2D,
    plants_texture: Texture2D,
    tilled_dirt_texture: Texture2D,
    farming_plants_texture: Texture2D,
    // map json
    tiled_map_json: String,
    bg_animation: AnimatedBackground,
    // ui
    menu_texture: Image,
    button_texture: Image,
    clicked_button_texture: Image,
    font: Vec<u8>,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        // Load texture
        let grass_texture = load_texture("./assets/Grass.png").await?;
        let hills_texture = load_texture("./assets/Hills.png").await?;
        let water_texture = load_texture("./assets/Water.png").await?;
        let wooden_house_wall_texture = load_texture("./assets/WoodenHouseWall.png").await?;
        let door_animation_texture = load_texture("./assets/DoorAnimation.png").await?;
        let basic_furniture_texture = load_texture("./assets/BasicFurniture.png").await?;
        let plants_texture = load_texture("./assets/Plants.png").await?;
        let tilled_dirt_texture = load_texture("./assets/TilledDirt.png").await?;
        let farming_plants_texture = load_texture("./assets/FarmingPlants.png").await?;
        // font
        let font = load_file("./assets/font.ttf").await.unwrap();
        // Load sounds
        // Load image
        let menu_texture = load_image("./assets/ui/Menu.png").await?;
        let button_texture = load_image("./assets/ui/Button.png").await?;
        let clicked_button_texture = load_image("./assets/ui/ClickedButton.png").await?;

        // Create a simple Tiled map JSON
        let tiled_map_json = load_string("assets/map.json").await.unwrap();

        // animated background gif
        let bg_animation = AnimatedBackground::load("./assets/animated-gif/", 64).await;

        let resources = Resources {
            grass_texture,
            hills_texture,
            water_texture,
            wooden_house_wall_texture,
            door_animation_texture,
            basic_furniture_texture,
            plants_texture,
            tilled_dirt_texture,
            farming_plants_texture,
            tiled_map_json,
            bg_animation,
            menu_texture,
            button_texture,
            clicked_button_texture,
            font,
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
            zoom: 2.5,
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
    collider: Actor,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Down = 0,
    Up = 1,
    Left = 2,
    Right = 3,
}

impl Player {
    async fn new(world: &mut World) -> Self {
        let texture = load_texture("assets/BasicCharacterSpritesheet.png")
            .await
            .unwrap();
        texture.set_filter(FilterMode::Nearest);

        // Create player collider: collision check minimize at 16px
        println!("{}, {}", screen_width(), screen_height());
        let position = Vec2::new(720.0, 720.0);
        let collider = world.add_actor(position, 16, 16);
        Self {
            position,
            texture,
            animation_frame: 0,
            frame_timer: 0.0,
            facing: Direction::Down,
            is_moving: false,
            map_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            target_position: None,
            target_effect_timer: 0.0,
            wave_active: false,
            collider,
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

    fn update(&mut self, dt: f32, world: &mut World, camera: &GameCamera) {
        let mut speed = 200.0;
        if get_state().speed != 0.0 {
            speed = get_state().speed;
        };
        let mut movement = Vec2::ZERO;
        self.is_moving = false;

        // Handle mouse events
        if is_mouse_button_pressed(MouseButton::Left) {
            let screen_position = Vec2::new(mouse_position().0, mouse_position().1);
            let world_position = camera.screen_to_world(screen_position);

            // Check if click is within map bounds
            if self.map_bounds.contains(world_position) {
                self.target_position = Some(world_position);
                self.target_effect_timer = 0.0;
                self.wave_active = true;
            }
        }

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

        // Apply movement with collision detection
        if movement.length() > 0.0 {
            movement = movement.normalize();
            let desired_position = self.position + movement * speed * dt;

            // Update collider position
            world.set_actor_position(self.collider, desired_position);

            // If no collision occurred, update player position
            let half_tile = 8.;
            if !world.collide_check(
                self.collider,
                desired_position + vec2(-half_tile, -half_tile),
            ) {
                self.position = desired_position;
            } else {
                // Reset collider position if collision occurred
                world.set_actor_position(self.collider, self.position);
            }
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Autonomous World".to_owned(),
        fullscreen: true,
        //window_height: 500,
        //window_width: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<Resources, macroquad::Error> {
    // Initialize Context
    let resources = Resources::new().await?;

    let mut game_state = GameState::MainMenu;

    // Initialize collision world
    let mut world = World::new();

    let mut player = Player::new(&mut world).await;
    let mut door = Door::new(Vec2::new(784.0, 560.0), 6).await;

    let mut camera = GameCamera::new();

    // Load the map
    let tiled_map = tiled::load_map(
        &resources.tiled_map_json,
        &[
            ("Grass.png", resources.grass_texture),
            ("Hills.png", resources.hills_texture),
            ("Water.png", resources.water_texture),
            ("WoodenHouseWall.png", resources.wooden_house_wall_texture),
            ("DoorAnimation.png", resources.door_animation_texture),
            ("BasicFurniture.png", resources.basic_furniture_texture),
            ("Plants.png", resources.plants_texture),
            ("TilledDirt.png", resources.tilled_dirt_texture),
            ("FarmingPlants.png", resources.farming_plants_texture),
        ],
        &[],
    )
    .unwrap();

    // Calculate the bounds for the land area (32x32 tiles in center)
    let tile_size = 16.0;
    let total_tiles = 90; // total map size in tiles
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

    // Create colliders for house tiles
    let map_width_tiles = tiled_map.raw_tiled_map.width as usize;
    let map_height_tiles = tiled_map.raw_tiled_map.height as usize;
    let mut static_colliders = vec![Tile::Empty; map_width_tiles * map_height_tiles];

    // Set up colliders for house tiles
    for (x, y, tile) in tiled_map.tiles("House", None) {
        if tile.is_some() {
            let index = (y as usize) * map_width_tiles + (x as usize);
            static_colliders[index] = Tile::Solid;
        }
    }

    // Add the static colliders to the world
    world.add_static_tiled_layer(static_colliders, 16.0, 16.0, map_width_tiles, 1);

    // UI
    let window_style = root_ui()
        .style_builder()
        .background(resources.menu_texture)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .background(resources.button_texture)
        .background_clicked(resources.clicked_button_texture)
        .font(&resources.font)
        .unwrap()
        .text_color(WHITE)
        .text_color_clicked(Color {
            r: 175. / 256.,
            g: 139. / 256.,
            b: 104. / 256.,
            a: 1.,
        })
        .text_color_hovered(Color {
            r: 220. / 256.,
            g: 185. / 256.,
            b: 138. / 256.,
            a: 1.,
        })
        .font_size(60)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font(&resources.font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    let window_size = vec2(370.0, 320.0);

    console_log("game start");
    let mut bg_animation = resources.bg_animation;

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        match game_state {
            GameState::MainMenu => {
                // Update animation
                bg_animation.update(dt);
                draw_texture_ex(
                    bg_animation.current_texture(),
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );

                // UI modal
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(90., -10.), "Main Menu");

                        if ui.button(vec2(65.0, 35.0), "Play") {
                            game_state = GameState::Playing;
                        }
                        if get_state().sui_address.is_empty() {
                            // disconnected
                            if ui.button(vec2(14.0, 135.0), "Connect") {}
                        } else {
                            // connected
                            if ui.button(vec2(14.0, 135.0), "Connected") {}
                        };
                    },
                );
            }
            GameState::Playing => {
                camera.update_viewport_size();

                // Update player with collision world
                player.update(dt, &mut world, &camera);

                // Update door animation
                // Toggle door when space is pressed
                if is_key_pressed(KeyCode::Space) && !door.is_animating() {
                    door.toggle();
                }
                door.update(dt);

                // Update camera to follow player
                camera.update(player.position);

                // Draw layers in order
                draw_tiled_layer(
                    &tiled_map,
                    &camera,
                    vec!["Ocean", "Land", "Floor", "House", "Furniture"],
                );

                // Draw player at center of screen
                player.draw_player(&camera);

                door.draw_door(&camera);

                // Draw target indicator if exists
                player.draw_wave_effect(&camera);

                draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, BLACK);
            }
        };

        next_frame().await;
    }
}

fn draw_tiled_layer(tiled_map: &Map, camera: &GameCamera, layers: Vec<&str>) {
    // Calculate camera offset for drawing
    let camera_offset = Vec2::new(
        screen_width() / 2.0 - camera.position.x * camera.zoom,
        screen_height() / 2.0 - camera.position.y * camera.zoom,
    );
    let map_width = tiled_map.raw_tiled_map.width as f32 * tiled_map.raw_tiled_map.tilewidth as f32;
    let map_height =
        tiled_map.raw_tiled_map.height as f32 * tiled_map.raw_tiled_map.tileheight as f32;
    for layer in layers {
        tiled_map.draw_tiles(
            layer,
            Rect::new(
                camera_offset.x,
                camera_offset.y,
                map_width * camera.zoom,
                map_height * camera.zoom,
            ),
            None,
        );
    }
}
