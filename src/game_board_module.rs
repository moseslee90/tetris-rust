use crate::game_constants::BOARD_HEIGHT;
use crate::game_constants::BOARD_WIDTH;
#[derive(Copy, Clone)]
pub struct GameBoard {
    pub game_board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}
impl GameBoard {
    pub fn print_game_board(&self) {
        println!("",);
        for k in (0..BOARD_HEIGHT).rev() {
            if k < 10 {
                print!(" ");
            }
            println!("{} {:?}", k, self.game_board[k]);
        }
        print!("    ",);
        for k in 0..BOARD_WIDTH {
            print!("{}  ", k);
        }
        println!("");
    }
    pub fn new() -> GameBoard {
        let mut game_board = GameBoard {
            game_board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
        };
        for x in 0..BOARD_WIDTH {
            game_board.game_board[0][x] = 2;
        };
        return game_board;
    }
}