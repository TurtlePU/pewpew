use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PewPewConfig {
    pub camera_fov: (f32, f32),
    pub bg_color: [f32; 4],
    pub player_speed: f32,
}
