use crate::game_constants::primitive_constants;
use crate::board::{GameBoard, GameVariables};

fn evaluate_game_board(game_board: &GameBoard) -> isize {
    let array: [[u8; primitive_constants::BOARD_WIDTH]; primitive_constants::BOARD_HEIGHT] = game_board.game_board;
    let mut score: isize = 0;
    let mut consecutive_cells_score = 0;
    let mut filled_row: u8 = 0;
    for y in 0..primitive_constants::BOARD_HEIGHT {
        let mut consecutive_cells: u8 = 0;
        let mut filled_cell: usize = 0;
        let mut blank_cell: usize = 0;
        for x in 0..primitive_constants::BOARD_WIDTH {
            //exit condition, if entire row is blank
            if array[y][x] == 0 {
                consecutive_cells = 0;
                blank_cell = blank_cell + 1;
            } else if array[y][x] == 2 {
                consecutive_cells = consecutive_cells + 1;
                filled_cell = filled_cell + 1;
            }
        }
        if blank_cell == 10 {
            break;
        }
        if filled_cell == 10 {
            filled_row = filled_row + 1;
        }
    }
    return score;
}
//current piece must be generated in game_variables
pub fn generate_move_dataset(
    game_board: GameBoard,
    game_variables: GameVariables,
) {
    //find number of distinct rotations
    let distinct_rotations: usize = game_variables.current_piece.distinct_rotations;
    //first choose rotation
    for n in 0..distinct_rotations {
        //generate one data set each rotation
        let mut game_variables_rotation = game_variables;
        if n != 0 {
            game_variables_rotation.rotate_piece_ai(n);
        }
        //find max right moves
        let max_right: usize = game_board.piece_max_moves(primitive_constants::RIGHT, &game_variables_rotation);
        //find max left movse
        let max_left: usize = game_board.piece_max_moves(primitive_constants::LEFT, &game_variables_rotation);
        for r in 0..(max_right + 1) {
            let mut game_variables_right = game_variables_rotation;
            game_variables_right.piece_location[1] = game_variables_right.piece_location[1] + r;
            let mut game_board_right = game_board;
            game_board_right.move_piece_down_max(&mut game_variables_right);
            game_board_right.print_game_board();
        }
        for l in 1..(max_left + 1) {
            let mut game_variables_left = game_variables_rotation;
            game_variables_left.piece_location[1] = game_variables_left.piece_location[1] - l;
            let mut game_board_left = game_board;
            game_board_left.move_piece_down_max(&mut game_variables_left);
            game_board_left.print_game_board();
        }
    }
}