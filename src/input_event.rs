use crate::game_object::change_snake_move;

pub fn watch_player() {
    loop {
        let mut input_content = String::new();
        std::io::stdin()
            .read_line(&mut input_content)
            .expect("Oh,that is not a valid button.");

        println!(" 移动是 {}", &input_content);
        change_snake_move(input_content);
    }
}
