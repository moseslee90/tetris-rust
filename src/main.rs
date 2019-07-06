mod ai;
mod board;
mod game_constants;
use std::io;

use game_constants::primitive_constants;

fn main() {

    println!("Welcome to tetris rust ai!");
    println!("init-pop: initialise a random population");
    println!("read-pop: read from current population and begin evaluation of population");
    println!("next-pop: repopulate via top 1% of evaluated population");
    println!("cycle-pop: cycle through a few generations of read-pop and next-pop");
    println!("play-top: run game using top individual currently stored. game will be printed out");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    match command.trim() {
        "init-pop" => ai::initialise_random_population(),
        "read-pop" => {
            ai::write_population_to_file(ai::read_population(primitive_constants::DATA_PATH))
        }
        "next-pop" => ai::next_generation(
            primitive_constants::DATA_OUTPUT_PATH,
            primitive_constants::DATA_PATH,
        ),
        "cycle-pop" => {
            let mut num_generations = String::new();
            println!("Enter a number for the number of generations to run:");
            io::stdin()
                .read_line(&mut num_generations)
                .expect("Failed to read line");
            let num_generations: usize =
                num_generations.trim().parse().expect("please use a number");
            for _i in 0..num_generations {
                ai::write_population_to_file(ai::read_population(primitive_constants::DATA_PATH));
                ai::next_generation(
                    primitive_constants::DATA_OUTPUT_PATH,
                    primitive_constants::DATA_PATH,
                );
            }
        }
        "play-top" => {
            let population = ai::get_population_json_from_file("data/best_individual.json");
            let best_genes = &population["individuals"][1]["genes"];
            let best_individual: ai::Baby = ai::baby_from_json_baby(best_genes);
            let score = ai::play_game_for_individual(&best_individual, true);
            println!("Individual cleared {} lines total", score);
        }
        _ => (),
    };

    // println!("You guessed: {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => {
    //         println!("You win!");
    //         break;
    //     }
    // }

}
