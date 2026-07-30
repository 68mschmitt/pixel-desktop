struct Position {
    x: i32,
    y: i32,
}
fn move_right(x: i32) -> i32 {
    x + 1
}

fn move_up(y: i32) -> i32 {
    y - 1
}

fn main() {
    let mut player_position = Position { x: 2, y: 3 };

    println!("Player starts at ({}, {}).", player_position.x, player_position.y);

    player_position.x = move_right(player_position.x);
    println!("Player moves right to ({}, {}).", player_position.x, player_position.y);

    player_position.y = move_up(player_position.y);
    println!("Player moves up to ({}, {}).", player_position.x, player_position.y);
}
