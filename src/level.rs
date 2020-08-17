use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelConfig {
    max_x: isize,
    max_y: isize,
}

impl LevelConfig {
    pub fn gen_level(&self) -> Vec<LevelEntity> {
        vec![
            LevelEntity {
                pos: (0., 3.),
                kind: LevelEntityKind::Exit,
            },
            LevelEntity {
                pos: (0., 2.),
                kind: LevelEntityKind::Wall,
            },
            LevelEntity {
                pos: (2., 0.),
                kind: LevelEntityKind::Wall,
            },
        ]
    }
}

pub struct LevelEntity {
    pub pos: (f32, f32),
    pub kind: LevelEntityKind,
}

pub enum LevelEntityKind {
    Wall,
    Exit,
}
