use std::time::SystemTime;

mod game_object;

fn main() {
    let sys_time = SystemTime::now();
    game_object::refresh();
    let sys_time_end = SystemTime::now();   
    println!("{:?}", &sys_time);
    println!("{:?}", &sys_time_end);
}