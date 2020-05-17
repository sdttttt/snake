use crate::game_object::{ GameObjectWorker, HEIGHT, WIDTH };

pub struct Wall {
    pub display: &'static str
}

impl Wall {
    pub fn new(display: &'static str) -> Self {
        Self { display }
    }
}

impl GameObjectWorker for Wall {
    fn make(&self, y: usize, x: usize) -> &'static str {
        if x == 0 || x == HEIGHT - 1 {
            &self.display
        } else if y == 0 || y == WIDTH - 1 {
            &self.display
        } else {
            ""
        }
    }
}