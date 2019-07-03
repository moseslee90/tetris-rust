use crate::board::{GameBoard, GameVariables};
use crate::game_constants::{primitive_constants, tetronominoes::Tetronomino};
use rand::Rng;

struct Genes {
    consecutive_x: f64,
    one_row_filled: f64,
    two_rows_filled: f64,
    three_rows_filled: f64,
    four_rows_filled: f64,
    gaps_vertical: f64,
    height: f64,
    border: f64,
}

impl Genes {
    fn new() -> Genes {
        Genes {
            consecutive_x: rand::thread_rng().gen_range(0.0, 100.0),
            one_row_filled: rand::thread_rng().gen_range(0.0, 100.0),
            two_rows_filled: rand::thread_rng().gen_range(0.0, 100.0),
            three_rows_filled: rand::thread_rng().gen_range(0.0, 100.0),
            four_rows_filled: rand::thread_rng().gen_range(0.0, 100.0),
            gaps_vertical: rand::thread_rng().gen_range(0.0, 100.0),
            height: rand::thread_rng().gen_range(0.0, 100.0),
            border: rand::thread_rng().gen_range(0.0, 100.0),
        }
    }
}

struct Baby {
    genes: Genes,
    fitness: usize,
}
//evaluation shld happen before update of game_board and filled cells are cleared
fn evaluate_game_board(
    game_board: &GameBoard,
    genes: &Genes,
) -> f64 {
    let game_board_array: [[u8; primitive_constants::BOARD_WIDTH];
        primitive_constants::BOARD_HEIGHT] = game_board.game_board;

    let mut score: f64 = 0.0;
    //score given based on consecutive groups of cells horizontally
    let mut con_cell_x_score: f64 = 0.0;
    //score given based on filled rows
    let mut filled_rows_score: f64 = 0.0;
    //array to keep track of number of gaps in each column
    let mut gaps_score: f64 = 0.0;
    //score given based on whether piece is occupying the sides of the board
    let mut border_score: f64 = 0.0;

    let mut gaps_array: [f64; primitive_constants::BOARD_WIDTH] =
        [0.0; primitive_constants::BOARD_WIDTH];
    let mut num_of_filled_rows: u8 = 0;
    for y in 0..primitive_constants::BOARD_HEIGHT {
        let mut num_of_con_x_cells: f64 = 0.0;
        let mut filled_cell: u8 = 0;
        let mut blank_cell: u8 = 0;
        for x in 0..primitive_constants::BOARD_WIDTH {
            let cell_value = game_board_array[y][x];
            update_for_con_cell_x(
                &mut con_cell_x_score,
                &mut num_of_con_x_cells,
                genes,
                cell_value,
            );
            update_for_vertical_gaps(&mut gaps_array, &mut gaps_score, genes, cell_value, x);
            update_for_border_piece(&mut border_score, genes, cell_value, x);
            //exit condition, if entire row is blank
            if cell_value == 0 {
                blank_cell += 1;
                gaps_array[x] += 1.0;
            } else if cell_value == 2 {
                filled_cell += 1;
            }
        }
        if blank_cell == 10 {
            break;
        }
        if filled_cell == 10 {
            num_of_filled_rows = num_of_filled_rows + 1;
        }
    }
    update_for_filled_rows(&mut filled_rows_score, num_of_filled_rows, genes);
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
        let max_right: usize =
            game_board.piece_max_moves(primitive_constants::RIGHT, &game_variables_rotation);
        //find max left movse
        let max_left: usize =
            game_board.piece_max_moves(primitive_constants::LEFT, &game_variables_rotation);
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

fn update_for_con_cell_x(
    con_cell_x_score: &mut f64,
    num_of_con_x_cells: &mut f64,
    genes: &Genes,
    cell_value: u8,
) {
    if cell_value == 0 {
        //update score for horizontal consecutive cells
        if *num_of_con_x_cells != 0.0 {
            *con_cell_x_score += genes.consecutive_x.powf(*num_of_con_x_cells);
            *num_of_con_x_cells = 0.0;
        }
    } else if cell_value == 2 {
        *num_of_con_x_cells += 1.0;
    }
}
fn update_for_filled_rows(
    filled_rows_score: &mut f64,
    num_of_filled_rows: u8,
    genes: &Genes,
) {
    *filled_rows_score += match num_of_filled_rows {
        1u8 => genes.one_row_filled,
        2u8 => genes.two_rows_filled,
        3u8 => genes.three_rows_filled,
        4u8 => genes.four_rows_filled,
        _ => panic!("more rows filled than possible!"),
    }
}
fn update_for_vertical_gaps(
    gaps_array: &mut [f64; 10],
    gaps_score: &mut f64,
    genes: &Genes,
    cell_value: u8,
    x: usize,
) {
    if cell_value == 0 {
        gaps_array[x] += 1.0;
    } else if cell_value == 2 {
        *gaps_score += gaps_array[x] * genes.gaps_vertical;
    }
}
fn update_for_border_piece(
    border_score: &mut f64,
    genes: &Genes,
    cell_value: u8,
    x: usize,
) {
    if cell_value == 2 && (x == 0 || x == primitive_constants::BOARD_WIDTH - 1) {
        *border_score += genes.border;
    }
}