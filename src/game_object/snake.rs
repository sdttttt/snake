use crate::game_object::{ GObject, Position };

pub struct Snake {
    pub display: &'static str,
    pub head_position: Position,
    pub body_position: Vec<Position>,
}

impl GObject for Snake {
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