pub const BOARD_HEIGHT: usize = 21;
pub const BOARD_WIDTH: usize = 10;
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
//constants for is_row function
const BLANK: &str = "blank";
const FILLED: &str = "filled";
const PARTIAL_FILL: &str = "partial_fill";