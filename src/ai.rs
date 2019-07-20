use crate::board::{GameBoard, GameVariables};
use crate::game_constants::{primitive_constants, tetronominoes::Tetronomino};
use json::{array, object, JsonValue};

use rand::Rng;
use std::{cmp, f64, fs, usize};

#[derive(Copy, Clone)]
pub struct Genes {
    consecutive_x: f64,
    one_row_filled: f64,
    two_rows_filled: f64,
    three_rows_filled: f64,
    four_rows_filled: f64,
    gaps_vertical: f64,
    height: f64,
    border: f64,
}

impl Genes {
    fn new() -> Genes {
        Genes {
            consecutive_x: rand::thread_rng().gen::<f64>() * 2.0,
            one_row_filled: rand::thread_rng().gen_range(0.0, 100.0),
            two_rows_filled: rand::thread_rng().gen_range(0.0, 200.0),
            three_rows_filled: rand::thread_rng().gen_range(0.0, 400.0),
            four_rows_filled: rand::thread_rng().gen_range(200.0, 1000.0),
            gaps_vertical: -rand::thread_rng().gen_range(0.0, 100.0),
            height: rand::thread_rng().gen::<f64>() + 1.0,
            border: rand::thread_rng().gen_range(0.0, 100.0),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Baby {
    pub genes: Genes,
    pub fitness: usize,
}

impl Baby {
    pub fn new() -> Baby {
        Baby {
            genes: Genes::new(),
            fitness: 0,
        }
    }
    pub fn new_with_values(
        consecutive_x: f64,
        one_row_filled: f64,
        two_rows_filled: f64,
        three_rows_filled: f64,
        four_rows_filled: f64,
        gaps_vertical: f64,
        height: f64,
        border: f64,
    ) -> Baby {
        Baby {
            genes: Genes {
                consecutive_x,
                one_row_filled,
                two_rows_filled,
                three_rows_filled,
                four_rows_filled,
                gaps_vertical,
                height,
                border,
            },
            fitness: 0,
        }
    }
}
//Struct to store decision that AI could or will take
#[derive(Debug)]
pub struct Decision<'a> {
    //if this decision is towards the left or right
    pub x_direction: &'a str,
    //number of moves in the x_direction
    pub moves: usize,
    //how many rotations
    pub rotations: usize,
    //score for the decision. highest scored decision will be chosen
    pub score: f64,
}

impl<'a> Decision<'a> {
    pub fn new(
        x_direction: &str,
        moves: usize,
        rotations: usize,
        score: f64,
    ) -> Decision {
        Decision {
            x_direction,
            moves,
            rotations,
            score,
        }
    }
}

//evaluation shld happen before update of game_board and filled cells are cleared
fn evaluate_game_board(
    game_board: &GameBoard,
    genes: &Genes,
) -> f64 {
    let game_board_array: [[u8; primitive_constants::BOARD_WIDTH];
        primitive_constants::BOARD_HEIGHT] = game_board.game_board;

    //score given based on consecutive groups of cells horizontally
    let mut con_cell_x_score: f64 = 0.0;
    //array to keep track of number of gaps in each column
    let mut gaps_score: f64 = 0.0;
    //score given based on whether piece is occupying the sides of the board
    let mut border_score: f64 = 0.0;
    //score, demerit, given based on how tall the tetris board is
    let mut height_score: f64 = 0.0;

    let mut gaps_array: [f64; primitive_constants::BOARD_WIDTH] =
        [0.0; primitive_constants::BOARD_WIDTH];
    let mut num_of_filled_rows: u8 = 0;
    for y in 1..primitive_constants::BOARD_HEIGHT {
        let mut num_of_con_x_cells: f64 = 0.0;
        let mut filled_cell: u8 = 0;
        let mut blank_cell: u8 = 0;
        for x in 0..primitive_constants::BOARD_WIDTH {
            let cell_value = game_board_array[y][x];
            update_for_con_cell_x(
                &mut con_cell_x_score,
                &mut num_of_con_x_cells,
                genes,
                cell_value,
            );
            update_for_vertical_gaps(&mut gaps_array, &mut gaps_score, genes, cell_value, x);
            update_for_border_piece(&mut border_score, genes, cell_value, x);
            update_for_height(&mut height_score, genes, cell_value, y);
            //exit condition, if entire row is blank
            if cell_value == 0 {
                blank_cell += 1;
                gaps_array[x] += 1.0;
            } else if cell_value == 2 {
                if y == primitive_constants::SPAWN_Y {
                    return f64::NEG_INFINITY / 2.0;
                }
                filled_cell += 1;
            }
        }
        if blank_cell == 10 {
            break;
        }
        if filled_cell == 10 {
            num_of_filled_rows = num_of_filled_rows + 1;
        }
    }
    let score: f64 = con_cell_x_score + gaps_score + border_score + height_score;
    return score;
}

fn evaluate_game_board_lines_cleared(
    game_board: &GameBoard,
    genes: &Genes,
) -> f64 {
    let game_board_array: [[u8; primitive_constants::BOARD_WIDTH];
        primitive_constants::BOARD_HEIGHT] = game_board.game_board;
    //score given based on filled rows
    let mut filled_rows_score: f64 = 0.0;
    let mut num_of_filled_rows: u8 = 0;
    for y in 1..primitive_constants::BOARD_HEIGHT {
        let mut filled_cell: u8 = 0;
        let mut blank_cell: u8 = 0;
        for x in 0..primitive_constants::BOARD_WIDTH {
            let cell_value = game_board_array[y][x];

            if cell_value == 0 {
                blank_cell += 1;
            } else if cell_value == 2 {
                filled_cell += 1;
            }
        }
        if blank_cell == 10 {
            break;
        }
        if filled_cell == 10 {
            num_of_filled_rows = num_of_filled_rows + 1;
        }
    }
    update_for_filled_rows(&mut filled_rows_score, num_of_filled_rows, genes);
    return filled_rows_score;
}
//current piece must be generated in game_variables
pub fn generate_move_dataset<'a>(
    current_or_holding: &str,
    game_board: GameBoard,
    game_variables: GameVariables,
    genes: &Genes,
    mut decision_final: Decision<'a>,
) -> Decision<'a> {
    //find number of distinct rotations
    let distinct_rotations: usize = game_variables.current_piece.distinct_rotations;
    //first choose rotation
    for n in 0..distinct_rotations {
        //generate one data set each rotation
        let mut game_variables_rotation = game_variables;

        rotate_piece_ai(&mut game_variables_rotation, n);

        //find max right moves
        let max_right: usize =
            game_board.piece_max_moves(primitive_constants::RIGHT, &game_variables_rotation);
        //find max left movse
        let max_left: usize =
            game_board.piece_max_moves(primitive_constants::LEFT, &game_variables_rotation);

        for r in 0..(max_right + 1) {
            evaluate_move(
                &mut decision_final,
                current_or_holding,
                primitive_constants::RIGHT,
                r,
                n,
                game_variables_rotation,
                game_board,
                genes,
            );
        }
        for l in 1..(max_left + 1) {
            evaluate_move(
                &mut decision_final,
                current_or_holding,
                primitive_constants::LEFT,
                l,
                n,
                game_variables_rotation,
                game_board,
                genes,
            );
        }
    }
    return decision_final;
}

fn evaluate_move<'a>(
    decision_final: &mut Decision<'a>,
    current_or_holding: &str,
    direction: &'a str,
    moves: usize,
    rotations: usize,
    mut game_variables: GameVariables, //copy
    mut game_board: GameBoard,         //copy
    genes: &Genes,
) {
    let mut decision: Decision =
        Decision::new(direction, moves, rotations, f64::NEG_INFINITY / 3.0);
    move_piece_x_ai(direction, moves, &mut game_variables);
    game_board.move_piece_down_max(&mut game_variables);
    //evaluation of the gameboard happens here
    match current_or_holding {
        primitive_constants::CURRENT_PIECE => {
            let first_piece_lines_filled_score =
                evaluate_game_board_lines_cleared(&game_board, genes);
            //update game board
            game_board.update_game_board();
            //reset game_variables for holding piece
            game_variables.spawn_new_tetronomino_on_board(primitive_constants::SIMULATION);
            decision = generate_move_dataset(
                primitive_constants::HOLDING_PIECE,
                game_board,
                game_variables,
                genes,
                decision,
            );
            decision.score = decision.score + first_piece_lines_filled_score;
            if decision.score > decision_final.score {
                *decision_final = decision;
            }
        }
        primitive_constants::HOLDING_PIECE => {
            //evaluate game_board a second time in full and update the decision score accordingly but
            //keep decision moves, rotation and direction the
            //same so decision can be made only on current piece
            let second_piece_lines_filled_score =
                evaluate_game_board_lines_cleared(&game_board, genes);
            game_board.update_game_board();
            decision.score =
                evaluate_game_board(&game_board, genes) + second_piece_lines_filled_score;
            if decision.score > decision_final.score {
                decision_final.score = decision.score;
            }
        }
        _ => (),
    }
}

//bug here, need to rewrite portion of finding anchor next so that
//code iterates through every preceding anchor next to find the final relative anchor position

pub fn rotate_piece_ai(
    game_variables: &mut GameVariables,
    rotation_state_end: usize,
) {
    if rotation_state_end == 0 {
        game_variables.rotation_state = rotation_state_end;
        return;
    }
    //get current tetronomino
    let tetronomino: &Tetronomino = game_variables.current_piece;
    //get current location of anchor
    let anchor_position_y_start: i8 = game_variables.piece_location[0] as i8;
    let anchor_position_x_start: i8 = game_variables.piece_location[1] as i8;
    //find relative coordinates of next anchor position after rotation
    let mut anchor_position_y_end = anchor_position_y_start;
    let mut anchor_position_x_end = anchor_position_x_start;
    for i in 0..rotation_state_end {
        let anchor_next_y: i8 = tetronomino.anchor_next[i][0];
        let anchor_next_x: i8 = tetronomino.anchor_next[i][1];

        anchor_position_y_end = anchor_position_y_end + anchor_next_y;
        anchor_position_x_end = anchor_position_x_end + anchor_next_x;
    }
    //find absolute coordinates of next anchor position after rotation on game_board
    let anchor_position_y_end: usize = anchor_position_y_end as usize;
    let anchor_position_x_end: usize = anchor_position_x_end as usize;
    //update game variables to current state
    game_variables.piece_location = [anchor_position_y_end, anchor_position_x_end];
    game_variables.rotation_state = rotation_state_end;
}

pub fn move_piece_x_ai(
    direction: &str,
    moves: usize,
    game_variables: &mut GameVariables,
) {
    if direction == primitive_constants::RIGHT {
        game_variables.piece_location[1] += moves;
    } else if direction == primitive_constants::LEFT {
        game_variables.piece_location[1] -= moves;
    } else {
        panic!(
            "unknown x movement constant given for move_piece_x_ai: {}",
            direction
        );
    }
}

fn update_for_con_cell_x(
    con_cell_x_score: &mut f64,
    num_of_con_x_cells: &mut f64,
    genes: &Genes,
    cell_value: u8,
) {
    if cell_value == 0 {
        //update score for horizontal consecutive cells
        if *num_of_con_x_cells != 0.0 {
            *con_cell_x_score += genes.consecutive_x.powf(*num_of_con_x_cells);
            *num_of_con_x_cells = 0.0;
        }
    } else if cell_value == 2 {
        *num_of_con_x_cells += 1.0;
    }
}
fn update_for_filled_rows(
    filled_rows_score: &mut f64,
    num_of_filled_rows: u8,
    genes: &Genes,
) {
    *filled_rows_score += match num_of_filled_rows {
        0u8 => 0.0,
        1u8 => genes.one_row_filled,
        2u8 => genes.two_rows_filled,
        3u8 => genes.three_rows_filled,
        4u8 => genes.four_rows_filled,
        _ => panic!("unhandled number of rows filled!: {}", num_of_filled_rows),
    }
}
fn update_for_vertical_gaps(
    gaps_array: &mut [f64; 10],
    gaps_score: &mut f64,
    genes: &Genes,
    cell_value: u8,
    x: usize,
) {
    if cell_value == 0 {
        gaps_array[x] += 1.0;
    } else if cell_value == 2 {
        *gaps_score += gaps_array[x] * genes.gaps_vertical;
    }
}
fn update_for_border_piece(
    border_score: &mut f64,
    genes: &Genes,
    cell_value: u8,
    x: usize,
) {
    if cell_value == 2 && (x == 0 || x == primitive_constants::BOARD_WIDTH - 1) {
        *border_score += genes.border;
    }
}
fn update_for_height(
    height_score: &mut f64,
    genes: &Genes,
    cell_value: u8,
    y: usize,
) {
    if cell_value == 2 {
        *height_score -= genes.height.powf(y as f64);
    }
}
pub fn initialise_random_population() {
    let mut parsed = object! {
        "individuals" => array![]
    };
    for i in 0..primitive_constants::POPULATION_SIZE {
        let baby = Baby::new();
        parsed["individuals"][i] = object! {
            "genes" => object!{
                "consecutive_x" => baby.genes.consecutive_x,
                "one_row_filled" => baby.genes.one_row_filled,
                "two_rows_filled" => baby.genes.two_rows_filled,
                "three_rows_filled" => baby.genes.three_rows_filled,
                "four_rows_filled" => baby.genes.four_rows_filled,
                "gaps_vertical" => baby.genes.gaps_vertical,
                "height" => baby.genes.height,
                "border" => baby.genes.border,
            },
            "fitness" => 0,
        };
    }
    let json_string = json::stringify(parsed);
    fs::write(primitive_constants::DATA_PATH, json_string)
        .expect("Unable to write to data_output.json");
}
pub fn write_population_to_file(
    array_individuals: [Baby; primitive_constants::TOP_INDIVIDUALS_SIZE]
) {
    let mut parsed = object! {
        "individuals" => array![]
    };
    for i in 0..primitive_constants::TOP_INDIVIDUALS_SIZE {
        let baby = array_individuals[i];
        parsed["individuals"][i] = object! {
            "genes" => object!{
                "consecutive_x" => baby.genes.consecutive_x,
                "one_row_filled" => baby.genes.one_row_filled,
                "two_rows_filled" => baby.genes.two_rows_filled,
                "three_rows_filled" => baby.genes.three_rows_filled,
                "four_rows_filled" => baby.genes.four_rows_filled,
                "gaps_vertical" => baby.genes.gaps_vertical,
                "height" => baby.genes.height,
                "border" => baby.genes.border,
            },
            "fitness" => baby.fitness,
        };
    }
    let json_string = json::stringify(parsed);
    fs::write(primitive_constants::DATA_OUTPUT_PATH, json_string)
        .expect("Unable to write to data_output.json");
}
pub fn get_population_json_from_file(file_path: &str) -> JsonValue {
    let data = fs::read_to_string(file_path).expect("Unable to read data/data.json");
    let parsed = json::parse(&data).unwrap();
    return parsed;
    // let population = &parsed["individuals"];
    // let baby = population
}
pub fn read_population(file_path: &str) -> [Baby; primitive_constants::TOP_INDIVIDUALS_SIZE] {
    //read population data from json file;
    let parsed = get_population_json_from_file(file_path);
    let population = &parsed["individuals"];
    //initialise random array of 10 individuals with 0 fitness
    //array is for keeping track of the top 10 individuals in the population during evaluation
    let mut top_individuals: [Baby; primitive_constants::TOP_INDIVIDUALS_SIZE] =
        [Baby::new(); primitive_constants::TOP_INDIVIDUALS_SIZE];

    let mut lowest_fitness: usize = usize::max_value();
    let mut lowest_index: usize = primitive_constants::TOP_INDIVIDUALS_SIZE;
    //initialise top_ten with first 3 in population
    for i in 0..3 {
        let genes = &population[i]["genes"];
        let mut baby = baby_from_json_baby(genes);
        baby.fitness = play_game_for_individual(&baby, false);
        top_individuals[i] = baby;
        if baby.fitness < lowest_fitness {
            lowest_fitness = baby.fitness;
            lowest_index = i;
        }
    }

    for i in 3..population.len() {
        let genes = &population[i]["genes"];
        let mut baby = baby_from_json_baby(genes);
        baby.fitness = play_game_for_individual(&baby, false);
        if baby.fitness > lowest_fitness {
            top_individuals[lowest_index] = baby;
            lowest_fitness = usize::max_value();
            for j in 0..primitive_constants::TOP_INDIVIDUALS_SIZE {
                if top_individuals[j].fitness < lowest_fitness {
                    lowest_fitness = top_individuals[j].fitness;
                    lowest_index = j;
                }
            }
        }
    }

    return top_individuals;
}

pub fn baby_from_json_baby(genes: &JsonValue) -> Baby {
    Baby::new_with_values(
        genes["consecutive_x"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["one_row_filled"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["two_rows_filled"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["three_rows_filled"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["four_rows_filled"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["gaps_vertical"]
            .as_f64()
            .expect("non-f64 value found"),
        genes["height"].as_f64().expect("non-f64 value found"),
        genes["border"].as_f64().expect("non-f64 value found"),
    )
}

pub fn play_game_for_individual(
    ai_baby: &Baby,
    print: bool,
) -> usize {
    let mut fitness_min = usize::max_value();
    let iterations: usize = if print { 1 } else { 3 };
    let line_limit: usize = if print { 2000 } else { 1500 };
    for _i in 0..iterations {
        let mut game_board = GameBoard::new();
        let mut game_variables = GameVariables::new();
        //generates random ai baby with random set of genes and 0 initial fitness
        game_variables.spawn_new_tetronomino_holding_board();
        let mut fitness = 0;
        while !game_board.is_game_over() && fitness < line_limit && fitness < fitness_min {
            let mut decision: Decision =
                Decision::new(primitive_constants::LEFT, 0, 0, f64::NEG_INFINITY);
            game_variables.spawn_new_tetronomino_on_board(primitive_constants::NOT_SIMULATION);
            game_board.change_piece(primitive_constants::GENERATE_PIECE, &game_variables);
            if print {
                game_board.pretty_print_game_board();
            }
            game_board.change_piece(primitive_constants::REMOVE_PIECE, &game_variables);
            decision = generate_move_dataset(
                primitive_constants::CURRENT_PIECE,
                game_board,
                game_variables,
                &ai_baby.genes,
                decision,
            );
            // println!("{:?}", decision);
            //decision generated, act on decision
            //first rotate piece based on decision
            rotate_piece_ai(&mut game_variables, decision.rotations);
            //second, move piece based on decision
            move_piece_x_ai(decision.x_direction, decision.moves, &mut game_variables);
            //move piece all the way down on game_board
            game_board.move_piece_down_max(&mut game_variables);
            fitness += game_board.update_game_board();
            //print move made by random ai
            if print {
                game_board.pretty_print_game_board();
                println!("lines cleared: {}", fitness);
            }
        }
        // println!("fitness_min is {} fitness is {}", fitness_min, fitness);
        fitness_min = cmp::min(fitness_min, fitness);
    }
    println!("{}", fitness_min);
    return fitness_min;
}

pub fn breed_individuals(
    mut baby_1: Baby,
    baby_2: &Baby,
) -> Baby {
    //select random number of genes
    let number_of_genes_to_swap: usize =
        rand::thread_rng().gen_range(1, primitive_constants::MAX_GENES_SWAP);
    for _i in 0..number_of_genes_to_swap {
        match rand::thread_rng().gen_range(0, 8) {
            0 => baby_1.genes.consecutive_x = baby_2.genes.consecutive_x,
            1 => baby_1.genes.one_row_filled = baby_2.genes.one_row_filled,
            2 => baby_1.genes.two_rows_filled = baby_2.genes.two_rows_filled,
            3 => baby_1.genes.three_rows_filled = baby_2.genes.three_rows_filled,
            4 => baby_1.genes.four_rows_filled = baby_2.genes.four_rows_filled,
            5 => baby_1.genes.gaps_vertical = baby_2.genes.gaps_vertical,
            6 => baby_1.genes.height = baby_2.genes.height,
            7 => baby_1.genes.border = baby_2.genes.border,
            _ => (),
        }
    }
    random_mutation(&mut baby_1);
    return baby_1;
}

fn random_mutation(baby_1: &mut Baby) {
    let dice_roll = rand::thread_rng().gen_range(0, 100);

    if dice_roll == 99 {
        let baby_2 = Baby::new();
        //mutate
        let number_of_genes_to_swap: usize =
            rand::thread_rng().gen_range(1, primitive_constants::MAX_GENES_SWAP);
        for _i in 0..number_of_genes_to_swap {
            match rand::thread_rng().gen_range(0, 8) {
                0 => baby_1.genes.consecutive_x = baby_2.genes.consecutive_x,
                1 => baby_1.genes.one_row_filled = baby_2.genes.one_row_filled,
                2 => baby_1.genes.two_rows_filled = baby_2.genes.two_rows_filled,
                3 => baby_1.genes.three_rows_filled = baby_2.genes.three_rows_filled,
                4 => baby_1.genes.four_rows_filled = baby_2.genes.four_rows_filled,
                5 => baby_1.genes.gaps_vertical = baby_2.genes.gaps_vertical,
                6 => baby_1.genes.height = baby_2.genes.height,
                7 => baby_1.genes.border = baby_2.genes.border,
                _ => (),
            }
        }
    }
}

pub fn next_generation(
    source_path: &str,
    output_path: &str,
) {
    let parsed = get_population_json_from_file(source_path);
    let population = &parsed["individuals"];

    let mut result = object! {
        "individuals" => array![]
    };

    for i in 0..primitive_constants::TOP_INDIVIDUALS_SIZE {
        let baby = baby_from_json_baby(&population[i]["genes"]);
        result["individuals"][i] = object! {
            "genes" => object!{
                "consecutive_x" => baby.genes.consecutive_x,
                "one_row_filled" => baby.genes.one_row_filled,
                "two_rows_filled" => baby.genes.two_rows_filled,
                "three_rows_filled" => baby.genes.three_rows_filled,
                "four_rows_filled" => baby.genes.four_rows_filled,
                "gaps_vertical" => baby.genes.gaps_vertical,
                "height" => baby.genes.height,
                "border" => baby.genes.border,
            },
            "fitness" => baby.fitness,
        };
    }

    for i in primitive_constants::TOP_INDIVIDUALS_SIZE
        ..(primitive_constants::POPULATION_SIZE - primitive_constants::RANDOM_INDIVIDUALS)
    {
        let baby_1_genes =
            &population[rand::thread_rng().gen_range(0, population.len()) as usize]["genes"];
        let baby_1 = baby_from_json_baby(baby_1_genes);
        let baby_2_genes =
            &population[rand::thread_rng().gen_range(0, population.len()) as usize]["genes"];
        let baby_2 = baby_from_json_baby(baby_2_genes);
        let baby = breed_individuals(baby_1, &baby_2);
        result["individuals"][i] = object! {
            "genes" => object!{
                "consecutive_x" => baby.genes.consecutive_x,
                "one_row_filled" => baby.genes.one_row_filled,
                "two_rows_filled" => baby.genes.two_rows_filled,
                "three_rows_filled" => baby.genes.three_rows_filled,
                "four_rows_filled" => baby.genes.four_rows_filled,
                "gaps_vertical" => baby.genes.gaps_vertical,
                "height" => baby.genes.height,
                "border" => baby.genes.border,
            },
            "fitness" => baby.fitness,
        };
    }

    for i in (primitive_constants::POPULATION_SIZE - primitive_constants::RANDOM_INDIVIDUALS)
        ..primitive_constants::POPULATION_SIZE
    {
        let baby = Baby::new();
        result["individuals"][i] = object! {
            "genes" => object!{
                "consecutive_x" => baby.genes.consecutive_x,
                "one_row_filled" => baby.genes.one_row_filled,
                "two_rows_filled" => baby.genes.two_rows_filled,
                "three_rows_filled" => baby.genes.three_rows_filled,
                "four_rows_filled" => baby.genes.four_rows_filled,
                "gaps_vertical" => baby.genes.gaps_vertical,
                "height" => baby.genes.height,
                "border" => baby.genes.border,
            },
            "fitness" => baby.fitness,
        };
    }

    let json_string = json::stringify(result);
    fs::write(output_path, json_string).expect("Unable to write to data.json");
}