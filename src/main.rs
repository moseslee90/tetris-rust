const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;
const HOLDING_SIZE: usize = 4;

const SPAWN_X: usize = 3;
const SPAWN_Y: usize = 17;

//Tetronominoes
struct Tetronomino {
    template: [[[usize; HOLDING_SIZE]; HOLDING_SIZE]; HOLDING_SIZE],
}
const PIECE_L: Tetronomino = Tetronomino {
    template: [
        [[0, 4, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 4, 0, 0], [0, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 4, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[1, 1, 4, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
};
const PIECE_J: Tetronomino = Tetronomino {
    template: [
        [[0, 4, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 4, 1, 1], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 4, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 4, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
};

fn main() {
    //initialise game_board
    let mut game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT] = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
    game_board = setup_board(game_board);
    //initialise holding area
    let holding_board: [[usize; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
    game_board = generate_piece(PIECE_J.template[0], game_board, SPAWN_X, SPAWN_Y);
    print_game_board(game_board);
    print_holding_board(holding_board);
}

fn setup_board(
    mut game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT]
) -> [[usize; BOARD_WIDTH]; BOARD_HEIGHT] {
    //create floor for game_board
    for k in 0..BOARD_WIDTH {
        game_board[0][k] = 2;
    }
    return game_board;
}

fn generate_piece(
    template: [[usize; HOLDING_SIZE]; HOLDING_SIZE],
    mut game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
    position_x: usize,
    position_y: usize,
) -> [[usize; BOARD_WIDTH]; BOARD_HEIGHT] {
    for y in 0..HOLDING_SIZE {
        for x in 0..HOLDING_SIZE {
            let cell_value: usize = template[y][x];
            if cell_value == 1 || cell_value == 4 {
                let y_coordinate: usize = position_y + y;
                let x_coordinate: usize = position_x + x;
                game_board[y_coordinate][x_coordinate] = cell_value;
            }
        }
    }
    return game_board;
}

fn print_game_board(game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT]) {
    println!("",);
    for k in (0..BOARD_HEIGHT).rev() {
        if k < 10 {
            print!(" ");
        }
        println!("{} {:?}", k, game_board[k]);
    }
    print!("    ",);
    for k in 0..BOARD_WIDTH {
        print!("{}  ", k);
    }
    println!("");
}

fn print_holding_board(holding_board: [[usize; HOLDING_SIZE]; HOLDING_SIZE]) {
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