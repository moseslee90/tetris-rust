use rand::Rng;

const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;
const HOLDING_SIZE: usize = 4;

const SPAWN_X: usize = 4;
const SPAWN_Y: usize = 17;
//game_board
struct GameBoard {
    game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
}

//Tetronominoes
struct Tetronomino {
    template: [[[isize; 2]; 4]; 4],
    //each template will have the formate
    //[[anchor_next],[pix1],[pix2],[pix3]]
}

const PIECE_L: Tetronomino = Tetronomino {
    template: [
        [[1, 0], [0, 1], [1, 0], [2, 0]],
        [[1, 1], [-1, 0], [0, 1], [0, 2]],
        [[-2, 0], [0, -1], [-1, 0], [-2, 0]],
        [[0, -1], [1, 0], [0, -1], [0, -2]],
    ],
};

const PIECE_J: Tetronomino = Tetronomino {
    template: [
        [[0, 0], [0, 1], [1, 1], [2, 1]],
        [[2, 0], [1, 0], [0, 1], [0, 2]],
        [[-1, 1], [0, 1], [-1, 0], [-2, 0]],
        [[-1, -1], [-1, 0], [0, -1], [0, -2]],
    ],
};

struct GameVariables {
    rotation_state: usize,
    current_piece: Tetronomino,
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
    let holding_board: [[usize; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
    //declare initial rotation state
    let mut game_variables = GameVariables {
        rotation_state: 0usize,
        current_piece: PIECE_J,
        piece_location: [0, 0],
    };

    //generate first piece on board
    random_tetronomino(&mut game_variables);
    generate_piece(&game_variables, &mut game_board);
    println!("{}", game_variables.rotation_state);
    //debugging to test results
    print_game_board(&game_board);
    print_holding_board(&holding_board);
    rotate_piece(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_right(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_right(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_right(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
    move_right(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
}

fn setup_board(game_board: &mut GameBoard) {
    //create floor for game_board
    for x in 0..BOARD_WIDTH {
        game_board.game_board[0][x] = 2;
    }
}

//function used to generate tetronomino based on game_variables
//assume collision is already handled
fn generate_piece(
    game_variables: &GameVariables,
    game_board: &mut GameBoard,
) {
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //print anchor
    game_board.game_board[location[0]][location[1]] = 4;
    //print 3 pixels
    //find correct template base on rotation_state
    let current_template: [[isize; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: isize = location[0] as isize;
        let location_x: isize = location[1] as isize;
        let pixel_absolute_pos_y: isize = current_template[i][0] + location_y;
        let pixel_absolute_pos_x: isize = current_template[i][1] + location_x;

        game_board.game_board[pixel_absolute_pos_y as usize][pixel_absolute_pos_x as usize] = 1;
    }
}

fn remove_piece(
    game_variables: &GameVariables,
    game_board: &mut GameBoard,
) {
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //remove anchor
    game_board.game_board[location[0]][location[1]] = 0;
    //remove 3 pixels
    //find correct template based on rotation state
    let current_template: [[isize; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: isize = location[0] as isize;
        let location_x: isize = location[1] as isize;
        let pixel_absolute_pos_y: isize = current_template[i][0] + location_y;
        let pixel_absolute_pos_x: isize = current_template[i][1] + location_x;

        game_board.game_board[pixel_absolute_pos_y as usize][pixel_absolute_pos_x as usize] = 0;
    }
}

fn random_tetronomino(game_variables: &mut GameVariables) {
    let random_number = rand::thread_rng().gen_range(1, 3);
    let spawned_piece: Tetronomino = match random_number {
        1 => PIECE_L, //choose L piece
        2 => PIECE_J, //choose J piece
        _ => PIECE_L,
    };
    game_variables.current_piece = spawned_piece;
    game_variables.piece_location = [SPAWN_Y, SPAWN_X];
}

fn rotate_piece(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {

    //get current tetronomino
    remove_piece(game_variables, game_board);
    let tetronomino: &Tetronomino = &game_variables.current_piece;
    //get current location of anchor
    let anchor_position_y_start: isize = game_variables.piece_location[0] as isize;
    let anchor_position_x_start: isize = game_variables.piece_location[1] as isize;
    //get current rotation state
    let rotation_state_start: usize = game_variables.rotation_state;
    //remove current piece in present rotation from game_board
    //find relative coordinates of next anchor position after rotation
    let anchor_next_y: isize = tetronomino.template[rotation_state_start][0][0];
    let anchor_next_x: isize = tetronomino.template[rotation_state_start][0][1];
    //find absolute coordinates of next anchor position after rotation on game_board
    let anchor_position_y_end: usize = if (anchor_position_y_start + anchor_next_y) > 0 {
        (anchor_position_y_start + anchor_next_y) as usize
    } else {
        SPAWN_Y
    };
    let anchor_position_x_end: usize = if (anchor_position_x_start + anchor_next_x) > 0 {
        (anchor_position_x_start + anchor_next_x) as usize
    } else {
        SPAWN_X
    };
    //update game variables to current state
    game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
    game_variables.rotation_state = if (game_variables.rotation_state + 1) > 3 {
        0
    } else {
        game_variables.rotation_state + 1
    };

    //replace piece with next rotation
    generate_piece(game_variables, game_board);

}

fn move_right(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    game_variables.piece_location[1] = game_variables.piece_location[1] + 1;
    if is_collision(game_variables, game_board) {
        game_variables.piece_location[1] = game_variables.piece_location[1] - 1;
    } else {
        game_variables.piece_location[1] = game_variables.piece_location[1] - 1;
        remove_piece(game_variables, game_board);
        game_variables.piece_location[1] = game_variables.piece_location[1] + 1;
        //update game_variables
        generate_piece(game_variables, game_board);
    }
}

fn is_collision(
    game_variables: &GameVariables,
    game_board: &GameBoard,
) -> bool {
    //check out of bounds
    let current_piece = game_variables.current_piece.template;
    let location = game_variables.piece_location;
    let rotation_state = game_variables.rotation_state;

    //check anchor location
    if location[0] >= BOARD_HEIGHT || location[1] >= BOARD_WIDTH {
        return true;
    }
    //print 3 pixels
    //find correct template base on rotation_state
    let current_template: [[isize; 2]; 4] = current_piece[rotation_state];
    //pixels located from 1 to 3 of array
    for i in 1..4 {
        let location_y: isize = location[0] as isize;
        let location_x: isize = location[1] as isize;
        let pixel_absolute_pos_y: usize = (current_template[i][0] + location_y) as usize;
        let pixel_absolute_pos_x: usize = (current_template[i][1] + location_x) as usize;

        if pixel_absolute_pos_y >= BOARD_HEIGHT || pixel_absolute_pos_x >= BOARD_WIDTH {
            return true;
        }
    }
    return false;
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

fn print_holding_board(holding_board: &[[usize; HOLDING_SIZE]; HOLDING_SIZE]) {
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