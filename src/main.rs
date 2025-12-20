use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Cell {
    possible_values: [bool; 9],
}

impl Cell {
    fn new() -> Cell {
        let vals: [bool; 9] = [true; 9];
        Cell {
            possible_values: vals,
        }
    }

    fn get_current_val(&self) -> usize {
        let mut pos: Vec<usize> = Vec::new();
        for i in 0..9 {
            if self.possible_values[i] {
                pos.push(i + 1)
            }
        }
        if pos.len() == 1 {
            return pos[0];
        }
        return 0;
    }
    fn eliminate(&mut self, val: usize) {
        self.possible_values[(val as usize) - 1] = false
    }
}

struct Game {
    cells: [Cell; 81],
    solution: [usize; 81],
}

impl Game {
    fn new() -> Game {
        let cells: [Cell; 81] = [Cell::new(); 81];
        let solution: [usize; 81] = [0; 81];
        Game { cells, solution }
    }

    fn load(&mut self, data: &str) {
        for cell in 0..81 {
            let num = data
                .chars()
                .nth(cell)
                .expect("should be char")
                .to_digit(10)
                .expect("shoud work");
            if num > 0 {
                for possible_val in 1..10 {
                    if possible_val != num {
                        self.cells[cell].eliminate(possible_val as usize);
                    }
                }
            }
        }

        for i in 0..81 {
            let num = data
                .chars()
                .nth(i + 82)
                .expect("should be char")
                .to_digit(10)
                .expect("shoud work");

            self.solution[i] = num as usize
        }
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[x + y * 9]
    }

    fn show(&self) {
        for j in 0..9 {
            if j % 3 == 0 {
                println!("+-----------+");
            }
            for i in 0..9 {
                if i % 3 == 0 {
                    print!("|");
                }
                print!("{}", self.get(i, j).get_current_val());
            }

            print!("|\n");
        }
        println!("+-----------+");
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for j in 0..9 {
            for i in 0..9 {
                if self.get(i, j).get_current_val() == 0 {
                    score = score + 1
                }
            }
        }
        score
    }

    fn iterate(&mut self) {
        for c in 0..81 {
            let self_row = c / 9;
            let self_col = c % 9;

            let self_cell_row = (self_row / 3) * 3;
            let self_cell_col = (self_col / 3) * 3;

            for col in 0..9 {
                if col != self_col {
                    let current_val = self.get(col, self_row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(current_val)
                    }
                }
            }

            for row in 0..9 {
                if row != self_row {
                    let current_val = self.get(self_col, row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(current_val)
                    }
                }
            }

            for col in self_cell_col..self_cell_col + 3 {
                for row in self_cell_row..self_cell_row + 3 {
                    if col == self_col && row == self_row {
                        continue;
                    }
                    let current_val = self.get(col, row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(current_val)
                    }
                }
            }
        }
    }

    fn solve(&mut self) -> Result<&str, &str> {
        let mut last_score = self.score();

        loop {
            self.iterate();
            let score = self.score();

            if score == 0 {
                return Ok("success");
            }

            if score == last_score {
                return Err("failure");
            }

            last_score = score;
        }
    }

    fn check(&self) -> Result<&str, String> {
        for c in 0..81 {
            let col = c % 9;
            let row = c / 9;

            let expected_val = self.solution[c];
            let actual_val = self.cells[c].get_current_val();

            if expected_val != actual_val {
                return Err(format!(
                    "failure at {},{} : {} != {}",
                    col, row, actual_val, expected_val
                ));
            }
        }
        return Ok("Solution matches");
    }
}

struct Loader {
    reader: io::BufReader<File>,
}

impl Loader {
    fn new(filename: &str) -> Loader {
        let file = File::open(filename).expect("File should open");
        let reader = io::BufReader::new(file);
        Loader { reader }
    }

    fn get_line(&mut self) -> String {
        let mut line_buffer = String::new();
        let _ = self.reader.read_line(&mut line_buffer);
        line_buffer
    }
}

fn main() {
    let mut loader = Loader::new("./data/small.csv");

    for _ in 1..10 {
        let mut game = Game::new();
        game.load(&loader.get_line());

        match game.solve() {
            Ok(_) => println!("Solved"),
            Err(_) => println!("Failed"),
        }

        match game.check() {
            Ok(s) => println!("{}", s),
            Err(s) => println!("{}", s),
        }
    }
}
