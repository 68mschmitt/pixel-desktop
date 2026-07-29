// fn spend_energy(current_energy: i32, amount: i32) -> i32{
//     current_energy - amount
// }

fn move_right(x: i32) -> i32 {
    x + 1
}

fn main() {
    // let player_name = "Ada";
    // let mut energy = 10;
    let mut player_x = 2;
    let player_y = 3;

    // println!("{player_name}, start with {energy} energy");
    println!("Player starts at ({player_x}, {player_y})");

    // energy = spend_energy(energy, 3);
    player_x = move_right(player_x);

    // println!("{player_name}, now has {energy} energy");
    println!("Player moves to ({player_x}, {player_y})");
}
