use macroquad::texture::{load_texture, Texture2D};

pub struct AnimatedBackground {
    frames: Vec<Texture2D>,
    current_frame: usize,
    frame_time: f32,
    elapsed: f32,
}

impl AnimatedBackground {
    pub async fn load(base_path: &str, count: usize) -> Self {
        // Pre-calculate all paths first
        let paths: Vec<String> = (0..count)
            .map(|i| format!("{}frame_{:02}_delay-0.1s.png", base_path, i))
            .collect();

        // Load textures sequentially but more efficiently
        let mut frames = Vec::with_capacity(count);
        for path in paths {
            let texture = load_texture(&path).await.unwrap();
            frames.push(texture);
        }

        AnimatedBackground {
            frames,
            current_frame: 0,
            frame_time: 0.1, // seconds per frame
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.elapsed >= self.frame_time {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.elapsed = 0.0;
        }
    }

    pub fn current_texture(&self) -> &Texture2D {
        &self.frames[self.current_frame]
    }
}
