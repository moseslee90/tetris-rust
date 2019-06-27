const BOARD_HEIGHT: usize = 21;
const BOARD_WIDTH: usize = 10;

fn main() {

    let game_board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT] = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
    let game_board = setup_board(game_board);
    print_game_board(game_board);
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
    for k in (0..BOARD_HEIGHT).rev() {
        println!("{} {:?}", k, game_board[k]);
    }
    print!("   ",);
    for k in (0..BOARD_WIDTH) {
        print!("{}, ", k);
    }
    println!("");
}