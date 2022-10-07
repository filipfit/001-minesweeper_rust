#![allow(dead_code, unused_variables)]
use rand::Rng;
use std::fmt;
use std::io;

// Minesweeper
/*
- visual representation
- state representation
    - real board and mask for board?
- cascade algorithm
 */

fn main() {
    // println!("Hello, world!");

    // fn print_board(size_x: usize, size_y: usize) -> () {
    //     let horizontal_line = "=".repeat(size_x);
    //     println!("{horizontal_line}");
    // }

    // let mut inp = String::new();

    // io::stdin()
    //     .read_line(&mut inp)
    //     .expect("could not read line");

    // let inp: usize = inp.trim().parse().expect("Couldnt parse user input!");

    // print_board(inp, inp);

    // println!("users input was: {inp}")

    let mut board = Board::new(20, 10);
    let coords = board.create_mines(10);

    // println!("{:?}", board.state);
    println!("{board:?}");

    println!("TEEEEST");

    let v = vec![vec![false; 10]; 5];
    // println!("{v:#?}");
}

struct Board {
    width: i32,
    height: i32,
    state: Vec<Vec<bool>>,
    mine_coords: Vec<i32>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("");

        // TODO make numbers be written from top to bottom
        for i in 1..=self.width {
            output.push_str(&(i.to_string()));
        }

        output.push('\n');

        for y in &self.state {
            for x in y {
                if *x {
                    output.push('*')
                } else {
                    output.push('.');
                }
            }
            output.push('\n')
        }
        return write!(f, "{output}");
    }
}

/**
 * @param width: i32 - Width of the board to be created
 * @param height: i32 - Height of the board to be created
 * @returns: Board - Board of specified width and height
 */
impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let state = Board::create_blank_state(width, height);

        return Board {
            width,
            height,
            state,
            mine_coords: vec![],
        };
    }

    /**
     * @param amount: i32 - Amount of mines to generate
     * @return: Vec<(i32, i32)> - Vector of tuples, each tuple containing the x and y coordinate of a
     * generated  mine
     */
    pub fn create_mines(&mut self, amount: i32) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let mut rand_coords: Vec<(i32, i32)> = Vec::with_capacity(amount as usize);

        let mut counter = 0;
        while counter < amount {
            let x = rng.gen_range(0..=self.width - 1);
            let y = rng.gen_range(0..=self.height - 1);
            // TODO what happens here...
            let cell = &mut self.state[y as usize][x as usize];

            if *cell {
                continue;
            }

            // TODO ... and here?
            *cell = true;
            rand_coords.push((x, y));

            counter = counter + 1;
        }
        return rand_coords;
    }

    fn create_blank_state(width: i32, height: i32) -> Vec<Vec<bool>> {
        let blank_state: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];
        return blank_state;
    }
}

fn take_user_input() {
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("could not read line");
}
