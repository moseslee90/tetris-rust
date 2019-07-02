mod board;
mod game_constants;

use json;
use rand::Rng;
use std::fs;

fn main() {
    //initialise game_board

    let mut game_board = board::GameBoard::new();
    game_board.print_game_board();
    let mut game_variables = board::GameVariables::new();

    game_variables.spawn_new_tetronomino_holding_board();
    game_variables.spawn_new_tetronomino_on_board();
    game_board.change_piece(
        game_constants::primitive_constants::GENERATE_PIECE,
        &game_variables,
    );
    game_board.print_game_board();
    game_board.move_piece(
        game_constants::primitive_constants::LEFT,
        2,
        &mut game_variables,
    );
    game_board.rotate_piece(&mut game_variables);
    game_board.print_game_board();

    // generate_move_dataset(game_variables, game_board);

    // let data = fs::read_to_string("data/data.json").expect("Unable to read data/data.json");
    // let parsed = json::parse(&data).unwrap();
    // let code = &parsed["code"];
    // println!("{:#}", code);
    // let json_string = json::stringify(parsed);
    // fs::write("data_output.json", json_string).expect("Unable to write to data_output.json")
}
