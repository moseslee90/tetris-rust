use rand::Rng;

const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;
const HOLDING_SIZE: usize = 4;

const SPAWN_X: usize = 3;
const SPAWN_Y: usize = 17;
//game_board
struct GameBoard {
    game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
}

//Tetronominoes
struct Tetronomino {
    template: [[[usize; HOLDING_SIZE]; HOLDING_SIZE]; HOLDING_SIZE],
    //position of anchor for each template
    //eg. anchor for PIECE_L for template 0 would be
    //anchor[0]
    //[0, 1]
    anchor: [[usize; 2]; 4],
}
const PIECE_L: Tetronomino = Tetronomino {
    template: [
        [[0, 4, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 4, 0, 0], [0, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 4, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[1, 1, 4, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
    anchor: [[0, 1], [0, 1], [0, 2], [0, 2]],
};
const PIECE_J: Tetronomino = Tetronomino {
    template: [
        [[0, 4, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 4, 1, 1], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 4, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 4, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
    anchor: [[0, 1], [0, 1], [0, 1], [0, 2]],
};
struct GameVariables {
    rotation_state: usize,
    current_piece: Tetronomino,
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
    generate_piece(
        game_variables.current_piece.template[game_variables.rotation_state],
        &mut game_board,
        SPAWN_X,
        SPAWN_Y,
    );
    println!("{}", game_variables.rotation_state);
    //debugging to test results
    print_game_board(&game_board);
    print_holding_board(&holding_board);
    rotate_piece(&mut game_variables, &mut game_board);
    print_game_board(&game_board);
}

fn setup_board(game_board: &mut GameBoard) {
    //create floor for game_board
    for x in 0..BOARD_WIDTH {
        game_board.game_board[0][x] = 2;
    }
}

fn generate_piece(
    template: [[usize; HOLDING_SIZE]; HOLDING_SIZE],
    game_board: &mut GameBoard,
    position_x: usize,
    position_y: usize,
) {
    for y in 0..HOLDING_SIZE {
        for x in 0..HOLDING_SIZE {
            let cell_value: usize = template[y][x];
            if cell_value == 1 || cell_value == 4 {
                let y_coordinate: usize = position_y + y;
                let x_coordinate: usize = position_x + x;
                game_board.game_board[y_coordinate][x_coordinate] = cell_value;
            }
        }
    }
}

// game_variables.current_piece = PIECE_L;
// game_variables.piece_location = [
//     PIECE_L.anchor[0][0] + SPAWN_Y,
//     PIECE_L.anchor[0][1] + SPAWN_X,
// ];

fn random_tetronomino(game_variables: &mut GameVariables) {
    let random_number = rand::thread_rng().gen_range(1, 3);
    let spawned_piece: Tetronomino = match random_number {
        1 => PIECE_L, //choose L piece
        2 => PIECE_J, //choose J piece
        _ => PIECE_L,
    };
    let location_y = spawned_piece.anchor[0][0] + SPAWN_Y;
    let location_x = spawned_piece.anchor[0][1] + SPAWN_X;
    game_variables.current_piece = spawned_piece;
    game_variables.piece_location = [location_y, location_x];
}

fn rotate_piece(
    game_variables: &mut GameVariables,
    game_board: &mut GameBoard,
) {
    let tetronomino = &game_variables.current_piece;
    let current_anchor = tetronomino.anchor[game_variables.rotation_state];
    let mut anchor_position_x: usize = 0;
    let mut anchor_position_y: usize = 0;
    let mut removal_count: usize = 0;
    //get reference of anchor and remove original piece
    'main_loop: for y in (0..BOARD_HEIGHT).rev() {
        for x in 0..BOARD_WIDTH {
            let element = game_board.game_board[y][x];
            if element == 4 {
                anchor_position_x = x;
                anchor_position_y = y;
            }
            if element == 4 || element == 1 {
                game_board.game_board[y][x] = 0;
                removal_count = removal_count + 1;
            }
            if removal_count == 4 {
                break 'main_loop;
            }
        }
    }
    //replace piece with next rotation
    let corner_position_y: usize = anchor_position_y - current_anchor[0];
    let corner_position_x: usize = anchor_position_x - current_anchor[1];
    game_variables.rotation_state = game_variables.rotation_state + 1;

    let next_template: [[usize; HOLDING_SIZE]; HOLDING_SIZE] =
        tetronomino.template[game_variables.rotation_state];
    //print next_template from corner_position of template that was just cleared.
    let mut addition_count: usize = 0;
    'add_piece_loop: for y in 0..HOLDING_SIZE {
        for x in 0..HOLDING_SIZE {
            if next_template[y][x] == 1 || next_template[y][x] == 4 {
                //in future, add collision handling code here
                game_board.game_board[corner_position_y + y][corner_position_x + x] =
                    next_template[y][x];
                addition_count = addition_count + 1;
            }
            if addition_count == 4 {
                break 'add_piece_loop;
            }
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