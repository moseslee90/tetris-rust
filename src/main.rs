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
        [[0, -1], [0, -1], [1, 0], [2, 0]],
        [[2, 0], [1, 0], [0, 1], [0, 2]],
        [[-1, 0], [0, 1], [-1, 0], [-2, 0]],
        [[-1, 0], [-1, 0], [0, -1], [0, -2]],
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
    // rotate_piece(&mut game_variables, &mut game_board);
    // print_game_board(&game_board);
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
    //pixels located from 1 to 3
    for i in 1..4 {
        let location_y: isize = location[0] as isize;
        let location_x: isize = location[1] as isize;
        let pixel_absolute_pos_y: isize = current_template[i][0] + location_y;
        let pixel_absolute_pos_x: isize = current_template[i][1] + location_x;

        game_board.game_board[pixel_absolute_pos_y as usize][pixel_absolute_pos_x as usize] = 1;
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

// fn rotate_piece(
//     game_variables: &mut GameVariables,
//     game_board: &mut GameBoard,
// ) {
//     let tetronomino = &game_variables.current_piece;
//     let current_anchor = tetronomino.anchor[game_variables.rotation_state];
//     let mut anchor_position_x: usize = 0;
//     let mut anchor_position_y: usize = 0;
//     let mut removal_count: usize = 0;
//     //get reference of anchor and remove original piece
//     'main_loop: for y in (0..BOARD_HEIGHT).rev() {
//         for x in 0..BOARD_WIDTH {
//             let element = game_board.game_board[y][x];
//             if element == 4 {
//                 anchor_position_x = x;
//                 anchor_position_y = y;
//             }
//             if element == 4 || element == 1 {
//                 game_board.game_board[y][x] = 0;
//                 removal_count = removal_count + 1;
//             }
//             if removal_count == 4 {
//                 break 'main_loop;
//             }
//         }
//     }
//     //replace piece with next rotation
//     let corner_position_y: usize = anchor_position_y - current_anchor[0];
//     let corner_position_x: usize = anchor_position_x - current_anchor[1];
//     game_variables.rotation_state = game_variables.rotation_state + 1;

//     let next_template: [[usize; HOLDING_SIZE]; HOLDING_SIZE] =
//         tetronomino.template[game_variables.rotation_state];
//     //print next_template from corner_position of template that was just cleared.
//     let mut addition_count: usize = 0;
//     'add_piece_loop: for y in 0..HOLDING_SIZE {
//         for x in 0..HOLDING_SIZE {
//             if next_template[y][x] == 1 || next_template[y][x] == 4 {
//                 //in future, add collision handling code here
//                 game_board.game_board[corner_position_y + y][corner_position_x + x] =
//                     next_template[y][x];
//                 addition_count = addition_count + 1;
//             }
//             if addition_count == 4 {
//                 break 'add_piece_loop;
//             }
//         }
//     }
// }

fn move_right() {
    for y in (0..BOARD_HEIGHT).rev() {
        for x in 0..BOARD_WIDTH {}
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