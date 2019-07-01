use crate::game_constants::primitive_constants::*;
use crate::game_constants::tetronominoes::*;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct GameBoard {
    pub game_board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}
#[derive(Copy, Clone)]
pub struct GameVariables<'a> {
    pub rotation_state: usize,
    pub holding_piece: &'a Tetronomino,
    pub current_piece: &'a Tetronomino,
    //position of anchor on board [y,x]
    pub piece_location: [usize; 2],
}
impl GameBoard {
    pub fn new() -> GameBoard {
        let mut game_board = GameBoard {
            game_board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
        };
        for x in 0..BOARD_WIDTH {
            game_board.game_board[0][x] = 2;
        }
        return game_board;
    }
    pub fn change_piece(
        &mut self,
        change_type: &str,
        game_variables: &GameVariables,
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
        self.game_board[location[0]][location[1]] = anchor_value;
        //change 3 pixels to fixed
        //find correct template based on rotation state
        let current_template: [[i8; 2]; 4] = current_piece[rotation_state];
        //pixels located from 1 to 3 of array
        for i in 1..4 {
            let location_y: i8 = location[0] as i8;
            let location_x: i8 = location[1] as i8;
            let pixel_absolute_pos_y: i8 = current_template[i][0] + location_y;
            let pixel_absolute_pos_x: i8 = current_template[i][1] + location_x;

            self.game_board[pixel_absolute_pos_y as usize][pixel_absolute_pos_x as usize] =
                pixel_value;
        }
    }

    pub fn move_piece(
        &mut self,
        direction: &str,
        amount: usize,
        game_variables: &mut GameVariables,
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
        if direction == DOWN && is_floor(&proposed_variables, &self) {
            //is floor, turn piece into fixed
            self.change_piece(FLOOR_FOUND, &game_variables);
            self.update_game_board();
        } else if self.no_collision(&proposed_variables) {
            //remove piece before moved state
            self.change_piece(REMOVE_PIECE, game_variables);
            //update game_variables
            game_variables.piece_location = proposed_location;
            //generate piece in new moved state
            self.change_piece(GENERATE_PIECE, game_variables);
        };
    }

    //for use in particular for AI for computing data sets
    pub fn move_piece_down_max(
        &mut self,
        game_variables: &mut GameVariables,
    ) {
        let down_moves = self.piece_max_moves(DOWN, game_variables);
        let location = game_variables.piece_location;
        //down moves based on entire tetronomino has been found
        //translate tetronomino based on down moves

        //first remove old piece
        self.change_piece(REMOVE_PIECE, game_variables);
        //update game_variables location to new max down location
        let new_anchor_location_y: usize = location[0] - down_moves;
        game_variables.piece_location[0] = new_anchor_location_y;
        self.change_piece(GENERATE_PIECE, game_variables);
        self.change_piece(FLOOR_FOUND, game_variables);
    }

    pub fn no_collision(
        &self,
        game_variables: &GameVariables,
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
        //collision with fixed pieces needs to be added in the future for human playability
    }
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

    pub fn piece_max_moves(
        &self,
        direction: &str,
        game_variables: &GameVariables,
    ) -> usize {
        let rotation_state = game_variables.rotation_state;
        let current_piece = game_variables.current_piece.template;
        let location = game_variables.piece_location;

        //check anchor
        let mut moves: usize = self.pixel_max_moves(direction, location);
        //check pixels

        let current_template: [[i8; 2]; 4] = current_piece[rotation_state];

        for i in 1..4 {
            let location_y: i8 = location[0] as i8;
            let location_x: i8 = location[1] as i8;
            let pixel_absolute_pos_y: usize = (current_template[i][0] + location_y) as usize;
            let pixel_absolute_pos_x: usize = (current_template[i][1] + location_x) as usize;
            let pixel_position: [usize; 2] = [pixel_absolute_pos_y, pixel_absolute_pos_x];

            let pixel_moves = self.pixel_max_moves(direction, pixel_position);
            if pixel_moves < moves {
                moves = pixel_moves;
            }
        }

        return moves;
    }

    pub fn pixel_max_moves(
        &self,
        direction: &str,
        pixel_location: [usize; 2],
    ) -> usize {
        match direction {
            LEFT => pixel_location[1],
            RIGHT => BOARD_WIDTH - 1 - pixel_location[1],
            DOWN => {
                let mut down_moves: usize = 0;
                for y in (0..(pixel_location[0] - 1)).rev() {
                    if self.game_board[y][pixel_location[1]] == 2 {
                        down_moves = pixel_location[0] - y - 1;
                        break;
                    }
                }
                down_moves
            }
            _ => panic!("unhandled direction constant in pixel_max_side_moves"),
        }
    }

    pub fn rotate_piece(
        &mut self,
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

        if self.no_collision(&proposed_variables) {
            //remove current piece in present rotation from game_board
            self.change_piece(REMOVE_PIECE, game_variables);
            //update game variables to current state
            game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
            game_variables.rotation_state = rotation_state_end;
            //replace piece with next rotation
            self.change_piece(GENERATE_PIECE, game_variables);
        }
    }
}

impl<'a> GameVariables<'a> {
    pub fn new() -> GameVariables<'a> {
        let mut game_variables = GameVariables {
            rotation_state: 0usize,
            holding_piece: &PIECE_J,
            current_piece: &PIECE_J,
            piece_location: [0, 0],
        };
        return game_variables;
    }
    pub fn spawn_tetronomino_holding_board(&mut self) {
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
        self.holding_piece = spawned_piece;
    }

    pub fn spawn_tetronomino_on_board(
        &mut self,
    ) {
        self.current_piece = self.holding_piece;
        self.piece_location = [SPAWN_Y, SPAWN_X];
        self.rotation_state = 0;
        self.spawn_tetronomino_holding_board();
        //to "see" tetronomino on game_board.game_board,
        //change_piece(GENERATE_PIECE, game_variables, game_board);
        //needs to be called
    }

    pub fn rotate_piece_ai(
        &mut self,
        rotation_state_end: usize,
    ) {
        //get current tetronomino
        let tetronomino: &Tetronomino = self.current_piece;
        //get current location of anchor
        let anchor_position_y_start: i8 = self.piece_location[0] as i8;
        let anchor_position_x_start: i8 = self.piece_location[1] as i8;
        //get current rotation state
        let rotation_state_start: usize = self.rotation_state;
        //find relative coordinates of next anchor position after rotation
        let anchor_next_y: i8 = tetronomino.template[rotation_state_start][0][0];
        let anchor_next_x: i8 = tetronomino.template[rotation_state_start][0][1];
        //find absolute coordinates of next anchor position after rotation on game_board
        let anchor_position_y_end: usize = (anchor_position_y_start + anchor_next_y) as usize;
        let anchor_position_x_end: usize = (anchor_position_x_start + anchor_next_x) as usize;
        //update game variables to current state
        self.piece_location = [anchor_position_y_end, anchor_position_x_end];
        self.rotation_state = rotation_state_end;
    }
}