mod open_space;
mod snake;
mod wall;

use lazy_static::lazy_static;
use open_space::make_open_space;
use snake::Snake;
use wall::Wall;

pub const WIDTH: usize = 20;
pub const HEIGTH: usize = 50;

// this is Global variable.
lazy_static! {
    static ref WALL: Wall = Wall { display: "*" };
    static ref SNAKE: Snake = Snake {
        display: "*",
        head_position: Position(5, 5),
        body_position: vec![Position(5, 4), Position(5, 3), Position(5, 2)]
    };
}

pub trait GObject {
    fn make(&self, y: usize, x: usize) -> &'static str;
}

pub struct Position(usize, usize);

pub fn init_map() {
    // Game Map
    let mut map: Vec<Vec<&str>> = Vec::new();

    for x in 0..WIDTH {
        let mut row: Vec<&str> = Vec::new();
        for y in 0..HEIGTH {
            let wall = WALL.make(x, y);
            let snake = SNAKE.make(x, y);

            if wall != "" {
                row.push(wall);
            } else if snake != "" {
                row.push(snake);
            } else {
                row.push(make_open_space());
            }
        }
        map.push(row);
    }

    render_map(map);
}

fn render_map(map: Vec<Vec<&str>>) {
    for dd in map {
        for bb in dd {
            print!("{}", bb);
        }
        println!("");
    }
}
