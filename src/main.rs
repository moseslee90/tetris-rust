mod ai;
mod board;
mod game_constants;
use std::io;

use game_constants::primitive_constants;

fn main() {

    println!("Welcome to tetris rust ai!");
    println!("To initialise a random population, type: init-pop");
    println!("To read from current population and begin evaluation of population, type: read-pop");
    println!("To repopulate via top 1% of evaluated population, type: next-pop");
    println!("To cycle through a few generations of read-pop and next-pop, type: cycle-pop");

    let mut command = String::new();

    io::stdin().read_line(&mut command)
        .expect("Failed to read line");

    match command.trim() {
        "init-pop" => println!("you entered init-pop"),
        "read-pop" => println!("you entered read-pop"),
        "next-pop" => println!("you entered next-pop"),
        "cycle-pop" => println!("you entered cycle-pop"),
        _ => ()
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
