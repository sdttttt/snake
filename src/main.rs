mod game_object;
mod timer;

use timer::Timer;

fn main() {
    let game_timer = Timer::new(1000);
    game_timer.execute();
}