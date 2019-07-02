pub mod primitive_constants {
    pub const BOARD_HEIGHT: usize = 21;
    pub const BOARD_WIDTH: usize = 10;
    pub const HOLDING_SIZE: usize = 4;

    pub const SPAWN_X: usize = 4;
    pub const SPAWN_Y: usize = 17;
    //constants for move_piece function
    pub const DOWN: &str = "down";
    pub const RIGHT: &str = "right";
    pub const LEFT: &str = "left";
    //constants for change_piece function
    pub const REMOVE_PIECE: &str = "remove_piece";
    pub const GENERATE_PIECE: &str = "generate_piece";
    pub const FLOOR_FOUND: &str = "floor_found";
    //constants for is_row function
    pub const BLANK: &str = "blank";
    pub const FILLED: &str = "filled";
    pub const PARTIAL_FILL: &str = "partial_fill";
}

pub mod tetronominoes {
    //Tetronominoes
    pub struct Tetronomino {
        pub template: [[[i8; 2]; 3]; 4],
        pub anchor_next: [[i8; 2]; 4],
        //each template will have the formate
        //[[anchor_next],[pix1],[pix2],[pix3]]
        pub distinct_rotations: usize,
    }

    pub const PIECE_L: Tetronomino = Tetronomino {
        template: [
            [[2, 0], [1, 0], [0, 1]],
            [[1, 0], [1, 1], [1, 2]],
            [[0, 1], [-1, 1], [-2, 1]],
            [[0, -1], [0, 1], [1, 1]],
        ],
        anchor_next: [[0, 0], [2, 0], [-2, 0], [0, 0]],
        distinct_rotations: 4,
    };

    pub const PIECE_J: Tetronomino = Tetronomino {
        template: [
            [[0, 1], [1, 1], [2, 1]],
            [[1, 0], [0, 1], [0, 2]],
            [[1, 0], [2, 0], [2, 1]],
            [[0, -1], [0, 1], [-1, 1]],
        ],
        anchor_next: [[0, 0], [0, 0], [1, 0], [-1, 0]],
        distinct_rotations: 4,
    };

    pub const PIECE_T: Tetronomino = Tetronomino {
        template: [
            [[1, 0], [0, -1], [0, 1]],
            [[0, 1], [1, 0], [-1, 0]],
            [[-1, 0], [0, -1], [0, 1]],
            [[0, -1], [1, 0], [-1, 0]],
        ],
        anchor_next: [[0, 0], [0, 0], [0, 0], [0, 0]],
        distinct_rotations: 4,
    };

    pub const PIECE_Z: Tetronomino = Tetronomino {
        template: [
            [[1, -1], [1, 0], [0, 1]],
            [[1, 0], [1, 1], [2, 1]],
            [[1, -1], [1, 0], [0, 1]],
            [[1, 0], [1, 1], [2, 1]],
        ],
        anchor_next: [[0, 0], [0, 0], [0, 0], [0, 0]],
        distinct_rotations: 2,
    };

    pub const PIECE_S: Tetronomino = Tetronomino {
        template: [
            [[0, 1], [1, 1], [1, 2]],
            [[1, 0], [1, -1], [2, -1]],
            [[0, 1], [1, 1], [1, 2]],
            [[1, 0], [1, -1], [2, -1]],
        ],
        anchor_next: [[0, 1], [0, -1], [0, 1], [0, -1]],
        distinct_rotations: 2,
    };

    pub const PIECE_O: Tetronomino = Tetronomino {
        template: [
            [[1, 0], [1, 1], [0, 1]],
            [[1, 0], [1, 1], [0, 1]],
            [[1, 0], [1, 1], [0, 1]],
            [[1, 0], [1, 1], [0, 1]],
        ],
        anchor_next: [[0, 0], [0, 0], [0, 0], [0, 0]],
        distinct_rotations: 1,
    };

    pub const PIECE_I: Tetronomino = Tetronomino {
        template: [
            [[0, -1], [0, 1], [0, 2]],
            [[1, 0], [2, 0], [3, 0]],
            [[0, -1], [0, 1], [0, 2]],
            [[1, 0], [2, 0], [3, 0]],
        ],
        anchor_next: [[0, 0], [0, 0], [0, 0], [0, 0]],
        distinct_rotations: 2,
    };
}
