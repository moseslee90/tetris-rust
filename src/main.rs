const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;
const HOLDING_SIZE: usize = 4;

fn main() {
    //initialise game_board
    let game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT] = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
    let game_board = setup_board(game_board);
    //initialise holding area
    let holding_board: [[usize; HOLDING_SIZE]; HOLDING_SIZE] = [[0; HOLDING_SIZE]; HOLDING_SIZE];
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