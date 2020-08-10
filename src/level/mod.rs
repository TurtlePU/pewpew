use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelConfig {
    max_x: isize,
    max_y: isize,
}

impl LevelConfig {
    pub fn gen_level(&self) -> Vec<LevelEntity> {
        let width = Uniform::from(-self.max_x..(self.max_x + 1));
        let height = Uniform::from(-self.max_y..(self.max_y + 1));
        let mut rng = thread_rng();
        let pos = loop {
            let x = height.sample(&mut rng);
            let y = width.sample(&mut rng);
            if x != 0 || y != 0 {
                break (x, y);
            }
        };
        vec![LevelEntity {
            pos,
            kind: LevelEntityKind::Exit,
        }]
    }
}

pub struct LevelEntity {
    pub pos: (isize, isize),
    pub kind: LevelEntityKind,
}

pub enum LevelEntityKind {
    Wall,
    Exit,
}
