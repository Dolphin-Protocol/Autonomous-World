use macroquad::prelude::*;

// Door states
enum DoorState {
    Closed,
    Opening,
    Open,
    Closing,
}

pub struct Door {
    texture: Texture2D,
    position: Vec2,
    state: DoorState,
    frame: usize,
    frames_count: usize,
    animation_timer: f32,
    frame_time: f32,
}

impl Door {
    pub async fn new(position: Vec2, frames_count: usize) -> Self {
        let texture = load_texture("assets/DoorAnimationSpriteSheet.png")
            .await
            .unwrap();

        Self {
            texture,
            position,
            state: DoorState::Closed,
            frame: 0,
            frames_count,
            animation_timer: 0.0,
            frame_time: 0.1, // Time per animation frame in seconds
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_timer += dt;

        if self.animation_timer >= self.frame_time {
            self.animation_timer = 0.0;

            match self.state {
                DoorState::Opening => {
                    if self.frame < self.frames_count - 1 {
                        self.frame += 1;
                    } else {
                        self.state = DoorState::Open;
                    }
                }
                DoorState::Closing => {
                    if self.frame > 0 {
                        self.frame -= 1;
                    } else {
                        self.state = DoorState::Closed;
                    }
                }
                _ => {} // No animation for Closed or Open states
            }
        }
    }

    pub fn draw_door(&self, camera: &crate::GameCamera) {
        // Calculate the source rectangle from the tileset
        let src_x = self.frame as f32 * 16.0;
        let src_rect = Rect::new(src_x, 0.0, 16.0, 16.0);
        // Assuming tiles are arranged horizontally
        // Convert world position to screen position using camera
        let screen_pos = camera.world_to_screen(self.position);
        // Apply camera zoom to the destination size
        let scaled_size = vec2(16.0 * camera.zoom, 16.0 * camera.zoom);

        // Draw the door with camera transformations
        draw_texture_ex(
            &self.texture,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(src_rect),
                dest_size: Some(scaled_size),
                ..Default::default()
            },
        );
    }

    pub fn toggle(&mut self) {
        match self.state {
            DoorState::Closed => self.state = DoorState::Opening,
            DoorState::Open => self.state = DoorState::Closing,
            _ => {} // Don't interrupt animations in progress
        }
    }

    pub fn is_animating(&self) -> bool {
        matches!(self.state, DoorState::Opening | DoorState::Closing)
    }
}
