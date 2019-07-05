mod ai;
mod board;
mod game_constants;

use game_constants::primitive_constants;

fn main() {

    println!("ai fitness is: {}", ai::play_game_for_individual(&ai::read_population()));

}
