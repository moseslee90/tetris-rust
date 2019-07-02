use crate::game_constants::{primitive_constants, tetronominoes::Tetronomino};
use crate::board::{GameBoard, GameVariables};

struct Genes {
    consecutive_x: f64,
}

struct Baby {
    genes: Genes,
    fitness: usize,
}
//evaluation shld happen before update of game_board and filled cells are cleared
fn evaluate_game_board(game_board: &GameBoard, genes: &Genes) -> f64 {
    let array: [[u8; primitive_constants::BOARD_WIDTH]; primitive_constants::BOARD_HEIGHT] =
    game_board.game_board;

    let mut score: f64 = 0.0;
    let mut consecutive_cells_score: f64 = 0.0;
    let mut filled_score: f64 = 0.0;

    let mut filled_row: f64 = 0.0;
    for y in 0..primitive_constants::BOARD_HEIGHT {
        let mut consecutive_cells: f64 = 0.0;
        let mut filled_cell: f64 = 0.0;
        let mut blank_cell: f64 = 0.0;
        for x in 0..primitive_constants::BOARD_WIDTH {
            //exit condition, if entire row is blank
            if array[y][x] == 0 {
                //update score for horizontal consecutive cells
                if consecutive_cells != 0.0 {
                    consecutive_cells_score += genes.consecutive_x.powf(consecutive_cells);
                    consecutive_cells = 0.0;
                }
                blank_cell = blank_cell + 1.0;
            } else if array[y][x] == 2 {
                consecutive_cells = consecutive_cells + 1.0;
                filled_cell = filled_cell + 1.0;
            }
        }
        if blank_cell == 10.0 {
            break;
        }
        if filled_cell == 10.0 {
            filled_row = filled_row + 1.0;
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
            rotate_piece_ai(&mut game_variables_rotation, n);
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

fn rotate_piece_ai(
    game_variables: &mut GameVariables,
    rotation_state_end: usize,
) {
    //get current tetronomino
    let tetronomino: &Tetronomino = game_variables.current_piece;
    //get current location of anchor
    let anchor_position_y_start: i8 = game_variables.piece_location[0] as i8;
    let anchor_position_x_start: i8 = game_variables.piece_location[1] as i8;
    //find relative coordinates of next anchor position after rotation
    let anchor_next_y: i8 = tetronomino.anchor_next[0][0];
    let anchor_next_x: i8 = tetronomino.anchor_next[0][1];
    //find absolute coordinates of next anchor position after rotation on game_board
    let anchor_position_y_end: usize = (anchor_position_y_start + anchor_next_y) as usize;
    let anchor_position_x_end: usize = (anchor_position_x_start + anchor_next_x) as usize;
    //update game variables to current state
    game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
    game_variables.rotation_state = rotation_state_end;
}