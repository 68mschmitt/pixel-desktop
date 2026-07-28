fn spend_energy(current_energy: i32, amount: i32) -> i32{
    current_energy - amount
}

fn main() {
    let player_name = "Ada";
    let mut energy = 10;

    println!("{player_name}, start with {energy} energy");

    energy = spend_energy(energy, 3);

    println!("{player_name}, now has {energy} energy");
}
