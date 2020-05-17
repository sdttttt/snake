mod open_space;
mod wall;
pub mod snake;

use std::sync::Mutex;
use lazy_static::lazy_static;
use open_space::make_open_space;
use snake::Snake;
use wall::Wall;

pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 50;

// this is Global variable.
// Store all the Object worker information.
// 全局变量, 存放所有的对象工人信息.
lazy_static! {
    static ref WALL: Wall = Wall::new("*");
    static ref SNAKE: Snake = Snake::new(
        "*",
        Position(5, 5),
        vec![Position(5, 4), Position(5, 3), Position(5, 2)]
    );

    static ref DIRECT: Mutex<Direction> = Mutex::new(Direction::RIGHT);
}

pub enum Direction {
    TOP,
    LEFT,
    RIGHT,
    BOTTOM,
}

// GameObject Worker.
// The Worker is responsible for controlling the Object
// and give the Object position information.
// 游戏对象工人，工人负责控制对象, 绘制出对象位置和信息等
pub trait GameObjectWorker {
    fn make(&self, y: usize, x: usize) -> &'static str;
}

// Game Object Position.
pub struct Position(usize, usize);

pub fn refresh() {
    // Game Map
    let mut map: Vec<Vec<&str>> = Vec::new();

    // look all node of Game Map.
    for x in 0..WIDTH {
        let mut row: Vec<&str> = Vec::new();
        for y in 0..HEIGHT {
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

pub fn change_snake_move(input: String) {

    let mut dt = DIRECT.lock().unwrap();

    *dt = match input.as_str() {
        "a" => Direction::LEFT,
        "w" => Direction::TOP,
        "d" => Direction::RIGHT,
        "s" => Direction::BOTTOM,
        _ => return,
    };
}

// Oh, this is a Render !!
fn render_map(map: Vec<Vec<&str>>) {
    for dd in map {
        for bb in dd {
            print!("{}", bb);
        }
        println!("");
    }
}

