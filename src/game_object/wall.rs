use crate::game_object::{ GameObjectWorker, HEIGTH, WIDTH };

pub struct Wall {
    pub display: &'static str
}

impl GameObjectWorker for Wall {
    fn make(&self, y: usize, x: usize) -> &'static str {
        if x == 0 || x == HEIGTH - 1 {
            &self.display
        } else if y == 0 || y == WIDTH - 1 {
            &self.display
        } else {
            ""
        }
    }
}