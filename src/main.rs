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

//game_board
struct GameBoard {
    game_board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}

//Tetronominoes
struct Tetronomino {
    template: [[[i8; 2]; 4]; 4],
    //each template will have the formate
    //[[anchor_next],[pix1],[pix2],[pix3]]
    distinct_rotations: u8,
}

const PIECE_L: Tetronomino = Tetronomino {
    template: [
        [[1, 0], [0, 1], [1, 0], [2, 0]],
        [[1, 1], [-1, 0], [0, 1], [0, 2]],
        [[-2, 0], [0, -1], [-1, 0], [-2, 0]],
        [[0, -1], [1, 0], [0, -1], [0, -2]],
    ],
    distinct_rotations: 4,
};

const PIECE_J: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [0, 1], [1, 1], [2, 1]],
        [[2, 0], [1, 0], [0, 1], [0, 2]],
        [[-1, 1], [0, 1], [-1, 0], [-2, 0]],
        [[-1, -1], [-1, 0], [0, -1], [0, -2]],
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

struct GameVariables<'a> {
    rotation_state: usize,
    current_piece: &'a Tetronomino,
    //position of anchor on board [y,x]
    piece_location: [usize; 2],
}

fn main() {
    //initialise game_board
    let mut game_board = GameBoard {
        game_board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
    };
    setup_board(&mut game_board);
    //initialise holding area
    let holding_board: [[u8; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
    //declare initial rotation state
    let mut game_variables = GameVariables {
        rotation_state: 0usize,
        current_piece: &PIECE_J,
        piece_location: [0, 0],
    };

    //generate first piece on board
    spawn_tetronomino(&mut game_variables);
    change_piece(GENERATE_PIECE, &game_variables, &mut game_board);
    println!("{}", game_variables.rotation_state);
    //debugging to test results
    print_game_board(&game_board);
    print_holding_board(&holding_board);
    move_piece(DOWN, &mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_piece(LEFT, &mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_piece(RIGHT, &mut game_variables, &mut game_board);
    print_game_board(&game_board);
    rotate_piece(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
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
    let mut anchor_value: u8 = 5;
    let mut pixel_value: u8 = 5;
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
        _ => (),
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

fn spawn_tetronomino(game_variables: &mut GameVariables) {
    let random_number = rand::thread_rng().gen_range(1, 8);
    let spawned_piece: &Tetronomino = match random_number {
        1 => &PIECE_L, //choose L piece
        2 => &PIECE_J, //choose J piece
        3 => &PIECE_T,
        4 => &PIECE_Z,
        5 => &PIECE_S,
        6 => &PIECE_O,
        7 => &PIECE_I,
        _ => &PIECE_L,
    };
    game_variables.current_piece = spawned_piece;
    game_variables.piece_location = [SPAWN_Y, SPAWN_X];
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

fn move_piece(
    direction: &str,
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    let mut proposed_location = game_variables.piece_location;
    match direction {
        RIGHT => {
            proposed_location[1] = proposed_location[1] + 1;
        }
        LEFT => {
            proposed_location[1] = proposed_location[1] - 1;
        }
        DOWN => {
            proposed_location[0] = proposed_location[0] - 1;
        }
        _ => (),
    }
    let proposed_variables = GameVariables {
        rotation_state: game_variables.rotation_state,
        current_piece: game_variables.current_piece,
        piece_location: proposed_location,
    };
    if direction == DOWN && is_floor(&proposed_variables, &game_board) {
        //is floor, turn piece into fixed
        change_piece(FLOOR_FOUND, &game_variables, game_board);

    } else if no_collision(&proposed_variables, game_board) {
        //remove piece before moved state
        change_piece(REMOVE_PIECE, game_variables, game_board);
        //update game_variables
        game_variables.piece_location = proposed_location;
        //generate piece in new moved state
        change_piece(GENERATE_PIECE, game_variables, game_board);
    };
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