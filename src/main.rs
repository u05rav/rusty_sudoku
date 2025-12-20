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

    fn getCurrentVal(&self) -> usize {
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
    solution: [i32; 81],
}

impl Game {
    fn new() -> Game {
        let cells: [Cell; 81] = [Cell::new(); 81];
        let solution: [i32; 81] = [0; 81];
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
                .nth(i)
                .expect("should be char")
                .to_digit(10)
                .expect("shoud work");
            self.solution[i] = num as i32
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
                print!("{}", self.get(i, j).getCurrentVal());
            }

            print!("|\n");
        }
        println!("+-----------+");
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for j in 0..9 {
            for i in 0..9 {
                if self.get(i, j).getCurrentVal() == 0 {
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
                    let currentVal = self.get(col, self_row).getCurrentVal();
                    if currentVal != 0 {
                        self.cells[c].eliminate(currentVal)
                    }
                }
            }

            for row in 0..9 {
                if row != self_row {
                    let currentVal = self.get(self_col, row).getCurrentVal();
                    if currentVal != 0 {
                        self.cells[c].eliminate(currentVal)
                    }
                }
            }

            for col in self_cell_col..self_cell_col + 3 {
                for row in self_cell_row..self_cell_row + 3 {
                    if col == self_col && row == self_row {
                        continue;
                    }
                    let currentVal = self.get(col, row).getCurrentVal();
                    if currentVal != 0 {
                        self.cells[c].eliminate(currentVal)
                    }
                }
            }
        }
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
    let mut game = Game::new();
    println!("Created Sudoku Game");

    let mut loader = Loader::new("./data/small.csv");

    game.load(&loader.get_line());

    game.show();

    println!("score = {}", game.score());

    for i in 1..10 {
        game.iterate();

        game.show();

        println!("score = {}", game.score());
    }
}
