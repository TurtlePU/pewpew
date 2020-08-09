use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PewPewConfig {
    pub camera_fov: (f32, f32)
}