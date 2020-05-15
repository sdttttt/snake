use std::time::SystemTime;

mod game_object;

fn main() {
    let sys_time = SystemTime::now();
    game_object::init_map();
    let sys_time_end = SystemTime::now();   
    println!("{:?}", &sys_time);
    println!("{:?}", &sys_time_end);
}