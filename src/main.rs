fn move_right(x: i32) -> i32 {
    x + 1
}

fn move_up(y: i32) -> i32 {
    y - 1
}

fn main() {
    let mut player_x = 2;
    let mut player_y = 3;

    println!("Player starts at ({player_x}, {player_y}).");

    player_x = move_right(player_x);
    println!("Player moves right to ({player_x}, {player_y}).");

    player_y = move_up(player_y);
    println!("Player moves up to ({player_x}, {player_y}).");
}
