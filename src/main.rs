mod ai;
mod board;
mod game_constants;

use game_constants::primitive_constants;
use json;
use std::fs;

fn main() {
    //initialise game_board
    let mut game_board = board::GameBoard::new();
    let mut game_variables = board::GameVariables::new();
    //generates random ai baby with random set of genes and 0 initial fitness
    let mut ai_baby: ai::Baby = ai::Baby::new();
    game_variables.spawn_new_tetronomino_holding_board();
    while !game_board.is_game_over() {
        let mut decision: ai::Decision = ai::Decision::new(primitive_constants::NONE, 0, 0);
        game_variables.spawn_new_tetronomino_on_board(primitive_constants::NOT_SIMULATION);
        game_board.change_piece(primitive_constants::GENERATE_PIECE, &game_variables);
        game_board.print_game_board();
        game_board.change_piece(primitive_constants::REMOVE_PIECE, &game_variables);
        decision = ai::generate_move_dataset(
            primitive_constants::CURRENT_PIECE,
            game_board,
            game_variables,
            &ai_baby.genes,
            decision,
        );
        println!("{:?}", decision);
        //decision generated, act on decision
        //first rotate piece based on decision
        ai::rotate_piece_ai(&mut game_variables, decision.rotations);
        //second, move piece based on decision
        ai::move_piece_x_ai(decision.x_direction, decision.moves, &mut game_variables);
        //move piece all the way down on game_board
        game_board.move_piece_down_max(&mut game_variables);
        ai_baby.fitness += game_board.update_game_board();
        //print move made by random ai
        game_board.print_game_board();
    }
    println!("ai fitness is: {}", ai_baby.fitness);

    // generate_move_dataset(game_variables, game_board);

    // let data = fs::read_to_string("data/data.json").expect("Unable to read data/data.json");
    // let parsed = json::parse(&data).unwrap();
    // let code = &parsed["code"];
    // println!("{:#}", code);
    // let json_string = json::stringify(parsed);
    // fs::write("data_output.json", json_string).expect("Unable to write to data_output.json")
}
