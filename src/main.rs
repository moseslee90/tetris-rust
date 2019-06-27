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

fn main() {
    //initialise game_board
    let mut game_board = GameBoard {
        game_board: [[0; BOARD_WIDTH]; BOARD_HEIGHT]
    };
    setup_board(&mut game_board);
    //initialise holding area
    let holding_board: [[usize; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
    //declare initial rotation state
    let mut rotation_state: usize = 0;
    //generate first piece on board
    generate_piece(
        PIECE_J.template[rotation_state],
        &mut game_board,
        SPAWN_X,
        SPAWN_Y,
    );
    println!("{}", rotation_state);
    //debugging to test results
    print_game_board(&game_board);
    print_holding_board(&holding_board);

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

fn rotate_piece(rotation_state: &mut usize, tetronomino: Tetronomino, game_board: &mut GameBoard) {

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