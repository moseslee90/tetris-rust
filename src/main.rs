use rand::Rng;

const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;
const HOLDING_SIZE: usize = 4;

const SPAWN_X: usize = 4;
const SPAWN_Y: usize = 17;
//constants for move_piece function
const DOWN: &str = "down";
const RIGHT: &str = "right";
const LEFT: &str = "left";
//constants for change_piece function
const REMOVE_PIECE: &str = "remove_piece";
const GENERATE_PIECE: &str = "generate_piece";
const FLOOR_FOUND: &str = "floor_found";
//constants for is_row function
const BLANK: &str = "blank";
const FILLED: &str = "filled";
const PARTIAL_FILL: &str = "partial_fill";

//game_board
#[derive(Copy, Clone)]
struct GameBoard {
    game_board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}

//Tetronominoes
struct Tetronomino {
    template: [[[i8; 2]; 4]; 4],
    //each template will have the formate
    //[[anchor_next],[pix1],[pix2],[pix3]]
    distinct_rotations: usize,
}

//Game Variables to be held to keep track of board activities
#[derive(Copy, Clone)]
struct GameVariables<'a> {
    rotation_state: usize,
    holding_piece: &'a Tetronomino,
    current_piece: &'a Tetronomino,
    //position of anchor on board [y,x]
    piece_location: [usize; 2],
}

const PIECE_L: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [2, 0], [1, 0], [0, 1]],
        [[2, 0], [1, 0], [1, 1], [1, 2]],
        [[-2, 0], [0, 1], [-1, 1], [-2, 1]],
        [[0, 0], [0, -1], [0, 1], [1, 1]],
    ],
    distinct_rotations: 4,
};

const PIECE_J: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [0, 1], [1, 1], [2, 1]],
        [[0, 0], [1, 0], [0, 1], [0, 2]],
        [[1, 0], [1, 0], [2, 0], [2, 1]],
        [[-1, 0], [0, -1], [0, 1], [-1, 1]],
    ],
    distinct_rotations: 4,
};

const PIECE_T: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [1, 0], [0, -1], [0, 1]],
        [[0, 0], [0, 1], [1, 0], [-1, 0]],
        [[0, 0], [-1, 0], [0, -1], [0, 1]],
        [[0, 0], [0, -1], [1, 0], [-1, 0]],
    ],
    distinct_rotations: 4,
};

const PIECE_Z: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [1, -1], [1, 0], [0, 1]],
        [[0, 0], [1, 0], [1, 1], [2, 1]],
        [[0, 0], [1, -1], [1, 0], [0, 1]],
        [[0, 0], [1, 0], [1, 1], [2, 1]],
    ],
    distinct_rotations: 2,
};

const PIECE_S: Tetronomino = Tetronomino {
    template: [
        [[0, 1], [0, 1], [1, 1], [1, 2]],
        [[0, -1], [1, 0], [1, -1], [2, -1]],
        [[0, 1], [0, 1], [1, 1], [1, 2]],
        [[0, -1], [1, 0], [1, -1], [2, -1]],
    ],
    distinct_rotations: 2,
};

const PIECE_O: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [1, 0], [1, 1], [0, 1]],
        [[0, 0], [1, 0], [1, 1], [0, 1]],
        [[0, 0], [1, 0], [1, 1], [0, 1]],
        [[0, 0], [1, 0], [1, 1], [0, 1]],
    ],
    distinct_rotations: 1,
};

const PIECE_I: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [0, -1], [0, 1], [0, 2]],
        [[0, 0], [1, 0], [2, 0], [3, 0]],
        [[0, 0], [0, -1], [0, 1], [0, 2]],
        [[0, 0], [1, 0], [2, 0], [3, 0]],
    ],
    distinct_rotations: 2,
};

fn main() {
    //initialise game_board
    let mut game_board = GameBoard {
        game_board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
    };
    setup_board(&mut game_board);
    //initialise holding area
    let mut holding_board: [[u8; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
    //declare initial rotation state
    let mut game_variables = GameVariables {
        rotation_state: 0usize,
        holding_piece: &PIECE_J,
        current_piece: &PIECE_J,
        piece_location: [0, 0],
    };

    //generate first piece on board
    //first iteration requires an explicit call to ready a holding piece
    spawn_tetronomino_holding_board(&mut game_variables);
    //loop shld start around here
    spawn_tetronomino_on_board(&mut game_variables, &mut game_board);
    change_piece(GENERATE_PIECE, &game_variables, &mut game_board);
    println!("{}", game_variables.rotation_state);
    //debugging to test results
    print_game_board(&game_board);
    print_holding_board(&holding_board);
    change_piece(REMOVE_PIECE, &game_variables, &mut game_board);

    generate_move_dataset(game_variables, game_board);

    //example of one iteration where piece is moved to the left, rotated once clockwise 90 degrees
    //and placed all the way down and game board is updated
    move_piece(DOWN, 1, &mut game_variables, &mut game_board);
    print_game_board(&game_board);
    rotate_piece(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_piece_down_max(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    //for ai simulation, a result of number of rows cleared might be
    //required to be returned from update_game_board
    update_game_board(&mut game_board);
    print_game_board(&game_board);
    //iteration ends
}

fn setup_board(game_board: &mut GameBoard) {
    //create floor for game_board
    for x in 0..BOARD_WIDTH {
        game_board.game_board[0][x] = 2;
    }
}

fn change_piece(
    change_type: &str,
    game_variables: &GameVariables,
    game_board: &mut GameBoard,
) {
    let anchor_value: u8;
    let pixel_value: u8;
    match change_type {
        REMOVE_PIECE => {
            anchor_value = 0;
            pixel_value = 0;
        }
        GENERATE_PIECE => {
            anchor_value = 4;
            pixel_value = 1;
        }
        FLOOR_FOUND => {
            anchor_value = 2;
            pixel_value = 2;
        }
        _ => panic!("change_piece given unhandled change_type constant"),
    };
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //change anchor to fixed state
    game_board.game_board[location[0]][location[1]] = anchor_value;
    //change 3 pixels to fixed
    //find correct template based on rotation state
    let current_template: [[i8; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: i8 = location[0] as i8;
        let location_x: i8 = location[1] as i8;
        let pixel_absolute_pos_y: i8 = current_template[i][0] + location_y;
        let pixel_absolute_pos_x: i8 = current_template[i][1] + location_x;

        game_board.game_board[pixel_absolute_pos_y as usize][pixel_absolute_pos_x as usize] =
            pixel_value;
    }
}

fn spawn_tetronomino_holding_board(game_variables: &mut GameVariables) {
    let random_number = rand::thread_rng().gen_range(1, 8);
    let spawned_piece: &Tetronomino = match random_number {
        1 => &PIECE_L, //choose L piece
        2 => &PIECE_J, //choose J piece
        3 => &PIECE_T,
        4 => &PIECE_Z,
        5 => &PIECE_S,
        6 => &PIECE_O,
        7 => &PIECE_I,
        _ => panic!("unhandled number for spawn_tetronomino generated!"),
    };
    game_variables.holding_piece = spawned_piece;
}

fn spawn_tetronomino_on_board(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    game_variables.current_piece = game_variables.holding_piece;
    game_variables.piece_location = [SPAWN_Y, SPAWN_X];
    game_variables.rotation_state = 0;
    spawn_tetronomino_holding_board(game_variables);
    //to "see" tetronomino on game_board.game_board,
    //change_piece(GENERATE_PIECE, game_variables, game_board);
    //needs to be called
}

fn rotate_piece(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {

    //get current tetronomino
    let tetronomino: &Tetronomino = game_variables.current_piece;
    //get current location of anchor
    let anchor_position_y_start: i8 = game_variables.piece_location[0] as i8;
    let anchor_position_x_start: i8 = game_variables.piece_location[1] as i8;
    //get current rotation state
    let rotation_state_start: usize = game_variables.rotation_state;
    //find relative coordinates of next anchor position after rotation
    let anchor_next_y: i8 = tetronomino.template[rotation_state_start][0][0];
    let anchor_next_x: i8 = tetronomino.template[rotation_state_start][0][1];
    //find absolute coordinates of next anchor position after rotation on game_board
    let anchor_position_y_end: usize = (anchor_position_y_start + anchor_next_y) as usize;
    let anchor_position_x_end: usize = (anchor_position_x_start + anchor_next_x) as usize;
    //create proposed_location to test in no_collision
    let proposed_location: [usize; 2] = [anchor_position_y_end, anchor_position_x_end];

    let rotation_state_end: usize = if (rotation_state_start + 1) > 3 {
        0
    } else {
        rotation_state_start + 1
    };

    let proposed_variables = GameVariables {
        rotation_state: rotation_state_end,
        holding_piece: game_variables.holding_piece,
        current_piece: tetronomino,
        piece_location: proposed_location,
    };

    if no_collision(&proposed_variables, game_board) {
        //remove current piece in present rotation from game_board
        change_piece(REMOVE_PIECE, game_variables, game_board);
        //update game variables to current state
        game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
        game_variables.rotation_state = rotation_state_end;
        //replace piece with next rotation
        change_piece(GENERATE_PIECE, game_variables, game_board);
    }
}

fn rotate_piece_ai(
    rotation_state_end: usize,
    game_variables: &mut GameVariables,
) {
    //get current tetronomino
    let tetronomino: &Tetronomino = game_variables.current_piece;
    //get current location of anchor
    let anchor_position_y_start: i8 = game_variables.piece_location[0] as i8;
    let anchor_position_x_start: i8 = game_variables.piece_location[1] as i8;
    //get current rotation state
    let rotation_state_start: usize = game_variables.rotation_state;
    //find relative coordinates of next anchor position after rotation
    let anchor_next_y: i8 = tetronomino.template[rotation_state_start][0][0];
    let anchor_next_x: i8 = tetronomino.template[rotation_state_start][0][1];
    //find absolute coordinates of next anchor position after rotation on game_board
    let anchor_position_y_end: usize = (anchor_position_y_start + anchor_next_y) as usize;
    let anchor_position_x_end: usize = (anchor_position_x_start + anchor_next_x) as usize;
    //update game variables to current state
    game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
    game_variables.rotation_state = rotation_state_end;
}

fn move_piece(
    direction: &str,
    amount: usize,
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    let mut proposed_location = game_variables.piece_location;
    match direction {
        RIGHT => {
            proposed_location[1] = proposed_location[1] + amount;
        }
        LEFT => {
            proposed_location[1] = proposed_location[1] - amount;
        }
        DOWN => {
            proposed_location[0] = proposed_location[0] - amount;
        }
        _ => panic!("move_piece: unhandled direction constant provided"),
    }
    let proposed_variables = GameVariables {
        rotation_state: game_variables.rotation_state,
        holding_piece: game_variables.holding_piece,
        current_piece: game_variables.current_piece,
        piece_location: proposed_location,
    };
    if direction == DOWN && is_floor(&proposed_variables, &game_board) {
        //is floor, turn piece into fixed
        change_piece(FLOOR_FOUND, &game_variables, game_board);
        update_game_board(game_board);
    } else if no_collision(&proposed_variables, game_board) {
        //remove piece before moved state
        change_piece(REMOVE_PIECE, game_variables, game_board);
        //update game_variables
        game_variables.piece_location = proposed_location;
        //generate piece in new moved state
        change_piece(GENERATE_PIECE, game_variables, game_board);
    };
}

//for use in particular for AI for computing data sets
fn move_piece_down_max(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    let down_moves = piece_max_moves(DOWN, game_variables, game_board);
    let location = game_variables.piece_location;
    //down moves based on entire tetronomino has been found
    //translate tetronomino based on down moves

    //first remove old piece
    change_piece(REMOVE_PIECE, game_variables, game_board);
    //update game_variables location to new max down location
    let new_anchor_location_y: usize = location[0] - down_moves;
    game_variables.piece_location[0] = new_anchor_location_y;
    change_piece(GENERATE_PIECE, game_variables, game_board);
    change_piece(FLOOR_FOUND, game_variables, game_board);
}

fn piece_max_moves(
    direction: &str,
    game_variables: &GameVariables,
    game_board: &GameBoard,
) -> usize {
    let rotation_state = game_variables.rotation_state;
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;

    //check anchor
    let mut moves: usize = pixel_max_moves(direction, location, game_board);
    //check pixels

    let current_template: [[i8; 2]; 4] = current_piece[rotation_state];

    for i in 1..4 {
        let location_y: i8 = location[0] as i8;
        let location_x: i8 = location[1] as i8;
        let pixel_absolute_pos_y: usize = (current_template[i][0] + location_y) as usize;
        let pixel_absolute_pos_x: usize = (current_template[i][1] + location_x) as usize;
        let pixel_position: [usize; 2] = [pixel_absolute_pos_y, pixel_absolute_pos_x];

        let pixel_moves = pixel_max_moves(direction, pixel_position, game_board);
        if pixel_moves < moves {
            moves = pixel_moves;
        }
    }

    return moves;
}

fn pixel_max_moves(
    direction: &str,
    pixel_location: [usize; 2],
    game_board: &GameBoard,
) -> usize {
    match direction {
        LEFT => pixel_location[1],
        RIGHT => BOARD_WIDTH - 1 - pixel_location[1],
        DOWN => {
            let mut down_moves: usize = 0;
            for y in (0..(pixel_location[0] - 1)).rev() {
                if game_board.game_board[y][pixel_location[1]] == 2 {
                    down_moves = pixel_location[0] - y - 1;
                    break;
                }
            }
            down_moves
        }
        _ => panic!("unhandled direction constant in pixel_max_side_moves"),
    }
}

fn is_floor(
    game_variables: &GameVariables,
    game_board: &GameBoard,
) -> bool {
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //check if anchor is adjacent to floor
    if game_board.game_board[location[0]][location[1]] == 2 {
        return true;
    }
    //check 3 pixels
    //find correct template base on rotation_state
    let current_template: [[i8; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: i8 = location[0] as i8;
        let location_x: i8 = location[1] as i8;
        let pixel_absolute_pos_y: usize = (current_template[i][0] + location_y) as usize;
        let pixel_absolute_pos_x: usize = (current_template[i][1] + location_x) as usize;

        if game_board.game_board[pixel_absolute_pos_y][pixel_absolute_pos_x] == 2 {
            return true;
        }
    }

    return false;
}

fn no_collision(
    game_variables: &GameVariables,
    game_board: &GameBoard,
) -> bool {
    //check out of bounds
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //check anchor location
    if location[0] >= BOARD_HEIGHT || location[1] >= BOARD_WIDTH {
        return false;
    }
    //check 3 pixels
    //find correct template base on rotation_state
    let current_template: [[i8; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: i8 = location[0] as i8;
        let location_x: i8 = location[1] as i8;
        let pixel_absolute_pos_y: usize = (current_template[i][0] + location_y) as usize;
        let pixel_absolute_pos_x: usize = (current_template[i][1] + location_x) as usize;

        if pixel_absolute_pos_y >= BOARD_HEIGHT || pixel_absolute_pos_x >= BOARD_WIDTH {
            return false;
        }
    }
    return true;
}

fn move_row_down(
    game_board: &mut GameBoard,
    row_index: usize,
    rows_filled: usize,
) {
    for x in 0..BOARD_WIDTH {
        game_board.game_board[row_index - rows_filled][x] = game_board.game_board[row_index][x];
        game_board.game_board[row_index][x] = 0;
    }
}

fn clear_row(
    game_board: &mut GameBoard,
    row_index: usize,
) {
    for x in 0..BOARD_WIDTH {
        game_board.game_board[row_index][x] = 0;
    }
}

fn row_is(row: &[u8; BOARD_WIDTH]) -> &str {
    let mut blank: u8 = 0;
    let mut filled: u8 = 0;

    for element in row.iter() {
        if element == &0u8 {
            blank = blank + 1;
        } else if element == &2u8 {
            filled = filled + 1;
        }
        if blank > 0 && filled > 0 {
            // println!("partial fill",);
            return PARTIAL_FILL;
        }
    }
    if blank == 10 {
        // println!("blank row",);
        return BLANK;
    } else if filled == 10 {
        // println!("filled row",);
        return FILLED;
    } else {
        return PARTIAL_FILL;
    }
}

//function to call right after piece has been placed and turned into a fixed piece
fn update_game_board(game_board: &mut GameBoard) {
    //iterate through rows from bottom skipping row 0
    //declare counter to keep track of rows filled
    let mut rows_filled: usize = 0;
    for row_index in 1..BOARD_HEIGHT {
        let row_reference: &[u8; BOARD_WIDTH] = &game_board.game_board[row_index];
        //row is will compute if row_reference given is a blank, filled or partial filled row
        match row_is(row_reference) {
            BLANK => return,
            FILLED => {
                rows_filled = rows_filled + 1;
                clear_row(game_board, row_index);
            }
            PARTIAL_FILL => {
                if rows_filled > 0 {
                    move_row_down(game_board, row_index, rows_filled)
                }
            }
            _ => panic!("unhandled match pattern in update_game_board"),
        }
    }
}

fn print_game_board(game_board: &GameBoard) {
    println!("",);
    for k in (0..BOARD_HEIGHT).rev() {
        if k < 10 {
            print!(" ");
        }
        println!("{} {:?}", k, game_board.game_board[k]);
    }
    print!("    ",);
    for k in 0..BOARD_WIDTH {
        print!("{}  ", k);
    }
    println!("");
}

fn print_holding_board(holding_board: &[[u8; HOLDING_SIZE]; HOLDING_SIZE]) {
    println!("",);
    for k in (0..HOLDING_SIZE).rev() {
        if k < 10 {
            print!(" ");
        }
        println!("{} {:?}", k, holding_board[k]);
    }
    print!("    ",);
    for k in 0..HOLDING_SIZE {
        print!("{}  ", k);
    }
    println!("");
}
//current piece must be generated in game_variables
fn generate_move_dataset(
    game_variables: GameVariables,
    game_board: GameBoard,
) {
    //find number of distinct rotations
    let distinct_rotations: usize = game_variables.current_piece.distinct_rotations;
    //first choose rotation
    for n in 0..distinct_rotations {
        //generate one data set each rotation
        let mut game_variables_rotation = game_variables;
        rotate_piece_ai(n, &mut game_variables_rotation);
        //find max right moves
        let max_right: usize = piece_max_moves(RIGHT, &game_variables_rotation, &game_board);
        //find max left movse
        let max_left: usize = piece_max_moves(LEFT, &game_variables_rotation, &game_board);
        for r in 0..(max_right + 1) {
            let mut game_variables_right = game_variables_rotation;
            game_variables_right.piece_location[1] = game_variables_right.piece_location[1] + r;
            let mut game_board_right = game_board;
            move_piece_down_max(&mut game_variables_right, &mut game_board_right);
            print_game_board(&game_board_right);
        }
        for l in 1..(max_left + 1) {
            let mut game_variables_left = game_variables_rotation;
            game_variables_left.piece_location[1] = game_variables_left.piece_location[1] - l;
            let mut game_board_left = game_board;
            move_piece_down_max(&mut game_variables_left, &mut game_board_left);
            print_game_board(&game_board_left);
        }
    }
}