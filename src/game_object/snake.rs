use crate::game_object::{ GameObjectWorker, Position };

pub enum Direction {
    TOP,
    LEFT,
    RIGHT,
    BUTTOM,
}

pub struct Snake {
    pub display: &'static str,
    pub head_position: Position,
    pub body_position: Vec<Position>,

    pub direct: Direction,
}

impl Snake {
    pub fn new(display: &'static str, head_position: Position, body_position: Vec<Position>) -> Self {
        let direct = Direction::RIGHT;
        Self {
            display,
            head_position,
            body_position,
            direct,
        }
    }

    pub fn change_move_direction(&mut self, dt: Direction) {
        self.direct = dt;
    }
}

impl GameObjectWorker for Snake {
    fn make(&self, y: usize, x: usize) -> &'static str {
        if y == self.head_position.0 && x == self.head_position.1 {
            self.display
        } else {
            for body_poss in &self.body_position {
                if y == body_poss.0 && x == body_poss.1 {
                    return self.display;
                };
            }
            ""
        }
    }
}