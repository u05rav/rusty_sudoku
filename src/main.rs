use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::process::exit;

#[derive(PartialEq)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

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
        for i in 0_usize..9_usize {
            if self.possible_values[i] {
                pos.push(i + 1)
            }
        }
        if pos.len() == 1 {
            return pos[0];
        }
        return 0;
    }
    fn eliminate(&mut self, val: &usize) {
        self.possible_values[val - 1] = false
    }

    fn allows(&self, val: &usize) -> bool {
        self.possible_values[val - 1]
    }

    fn set(&mut self, val: &usize) {
        for i in 0..9 {
            self.possible_values[i] = false;
        }
        self.possible_values[val - 1] = true
    }

    fn possible_values(&self) -> HashSet<usize> {
        let mut hs: HashSet<usize> = HashSet::new();
        for v in 0_usize..9_usize {
            if self.possible_values[v] {
                hs.insert(v + 1);
            }
        }
        hs
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
        for cell in 0_usize..81_usize {
            let num = data
                .chars()
                .nth(cell)
                .expect("should be char")
                .to_digit(10)
                .expect("shoud work") as usize;

            if num > 0 {
                for possible_val in 1_usize..10_usize {
                    if possible_val != num {
                        self.cells[cell].eliminate(&possible_val);
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

    fn get(&self, x: &usize, y: &usize) -> &Cell {
        &self.cells[x + y * 9]
    }

    fn get_mut(&mut self, x: &usize, y: &usize) -> &mut Cell {
        &mut self.cells[x + y * 9]
    }

    fn show_notes(&self) {
        for row in 0..9 {
            for line in 0..3 {
                if line == 0 {
                    if row % 3 == 0 {
                        println!("+------+------+------+------+------+------+------+------+------+")
                    } else {
                        println!("+......+......+......+......+......+......+......+......+......+")
                    }
                }
                for col in 0..9 {
                    let cell = self.get(&col, &row);
                    for num in 0_usize..3_usize {
                        if num == 0 {
                            if col % 3 == 0 {
                                print!("|");
                            } else {
                                print!(":");
                            }
                        }
                        let num = (line * 3) + num + 1;
                        if cell.allows(&num) {
                            print!("{} ", num);
                        } else {
                            print!("  ");
                        }
                    }
                }
                println!("|")
            }
        }
        println!("+------+------+------+------+------+------+------+------+------+")
    }

    fn show(&self) {
        for j in 0_usize..9_usize {
            if j % 3 == 0 {
                println!("+-----------+");
            }
            for i in 0_usize..9_usize {
                if i % 3 == 0 {
                    print!("|");
                }
                print!("{}", self.get(&i, &j).get_current_val());
            }

            print!("|\n");
        }
        println!("+-----------+");
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for j in 0_usize..9_usize {
            for i in 0_usize..9_usize {
                score = score + self.get(&i, &j).possible_values().len()
            }
        }
        score
    }

    fn iterate(&mut self) {
        for c in 0..81 {
            let self_row = c / 9;
            let self_col = c % 9;

            let self_block_row = (self_row / 3) * 3;
            let self_block_col = (self_col / 3) * 3;

            for col in 0_usize..9_usize {
                if col != self_col {
                    let current_val = self.get(&col, &self_row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(&current_val)
                    }
                }
            }

            for row in 0_usize..9_usize {
                if row != self_row {
                    let current_val = self.get(&self_col, &row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(&current_val)
                    }
                }
            }

            for col in self_block_col..self_block_col + 3 {
                for row in self_block_row..self_block_row + 3 {
                    if col == self_col && row == self_row {
                        continue;
                    }
                    let current_val = self.get(&col, &row).get_current_val();
                    if current_val != 0 {
                        self.cells[c].eliminate(&current_val)
                    }
                }
            }
        }

        for col in 0_usize..9_usize {
            for num in 1_usize..10_usize {
                let mut count = 0;
                let mut hit = 10_usize;
                for row in 0_usize..9_usize {
                    if self.get(&col, &row).allows(&num) {
                        count = count + 1;
                        hit = row;
                    }
                }
                if count == 1 {
                    self.set(&col, &hit, &num)
                }
            }
        }

        for row in 0_usize..9_usize {
            for num in 1_usize..10_usize {
                let mut count = 0;
                let mut hit = 10_usize;
                for col in 0_usize..9_usize {
                    if self.get(&col, &row).allows(&num) {
                        count = count + 1;
                        hit = col;
                    }
                }
                if count == 1 {
                    self.set(&hit, &row, &num)
                }
            }
        }

        for block_col_start in [0_usize, 3_usize, 6_usize] {
            for block_row_start in [0_usize, 3_usize, 6_usize] {
                for num in 1_usize..10_usize {
                    let mut count = 0;
                    let mut hit_col = 10;
                    let mut hit_row = 10;
                    for col_offset in [0_usize, 1_usize, 2_usize] {
                        for row_offset in [0_usize, 1_usize, 2_usize] {
                            let x = block_col_start + col_offset;
                            let y = block_row_start + row_offset;
                            if self.get(&x, &y).allows(&num) {
                                count = count + 1;
                                hit_col = x;
                                hit_row = y;
                            }
                        }
                    }
                    if count == 1 {
                        self.set(&hit_col, &hit_row, &num)
                    }
                }
            }
        }

        for c in 0_usize..81_usize {
            let self_row = c / 9;
            let self_col = c % 9;

            let self_block_row = (self_row / 3) * 3;
            let self_block_col = (self_col / 3) * 3;

            let possible_values = self.cells[c].possible_values();

            let mut matches: Vec<Coord> = Vec::new();
            for col in 0_usize..9_usize {
                let compare_possible_values = self.get(&col, &self_row).possible_values();
                if compare_possible_values
                    .difference(&possible_values)
                    .collect::<Vec<&usize>>()
                    .len()
                    == 0
                {
                    matches.push(Coord {
                        x: col,
                        y: self_row,
                    });
                }
            }
            if matches.len() > 1 && matches.len() == possible_values.len() {
                for col in 0_usize..0_usize {
                    let coord = Coord {
                        x: col,
                        y: self_row,
                    };

                    if !matches.contains(&coord) {
                        for n in &possible_values {
                            self.get_mut(&col, &self_row).eliminate(&n);
                        }
                    }
                }
            }

            let mut matches: Vec<Coord> = Vec::new();
            for row in 0_usize..9_usize {
                let compare_possible_values = self.get(&self_col, &row).possible_values();
                if compare_possible_values
                    .difference(&possible_values)
                    .collect::<Vec<&usize>>()
                    .len()
                    == 0
                {
                    matches.push(Coord {
                        x: self_col,
                        y: row,
                    });
                }
            }
            if matches.len() > 1 && matches.len() == possible_values.len() {
                for row in 0_usize..0_usize {
                    let coord = Coord {
                        x: self_col,
                        y: row,
                    };

                    if !matches.contains(&coord) {
                        for n in &possible_values {
                            self.get_mut(&self_col, &row).eliminate(&n);
                        }
                    }
                }
            }

            let mut matches: Vec<Coord> = Vec::new();
            for col in self_block_col..self_block_col + 3 {
                for row in self_block_row..self_block_row + 3 {
                    let compare_possible_values = self.get(&col, &row).possible_values();
                    if compare_possible_values
                        .difference(&possible_values)
                        .collect::<Vec<&usize>>()
                        .len()
                        == 0
                    {
                        matches.push(Coord { x: col, y: row });
                    }
                }
            }

            if matches.len() > 1 && matches.len() == possible_values.len() {
                for col in self_block_col..self_block_col + 3 {
                    for row in self_block_row..self_block_row + 3 {
                        let coord = Coord { x: col, y: row };

                        if !matches.contains(&coord) {
                            for n in &possible_values {
                                self.get_mut(&col, &row).eliminate(&n);
                            }
                        }
                    }
                }
            }
        }
    }

    fn set(&mut self, col: &usize, row: &usize, num: &usize) {
        self.get_mut(col, row).set(num);
    }

    fn solve(&mut self) -> Result<&str, &str> {
        let mut last_score = self.score();

        loop {
            self.iterate();
            let score = self.score();

            if score == 9 * 9 {
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

    fn get_line(&mut self) -> Result<String, &str> {
        let mut line_buffer = String::new();
        match self.reader.read_line(&mut line_buffer) {
            Ok(len) => {
                if len > 0 {
                    return Ok(line_buffer);
                }
                return Err("");
            }
            Err(_) => return Err(""),
        }
    }
}

fn solve(data: &str) -> bool {
    let mut game = Game::new();
    game.load(&data);

    let solved = match game.solve() {
        Ok(_) => true,
        Err(_) => false,
    };

    let correct = match game.check() {
        Ok(_) => true,
        Err(_) => false,
    };

    return solved && correct;
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() -> std::io::Result<()> {
    //let mut loader = Loader::new("./data/small.csv");
    //let mut loader = Loader::new("./data/sudoku.csv");
    //let mut loader = Loader::new("./data/sudoku-3m.csv");

    let args = Args::parse();

    println!("loading from {}", args.filename);
    let mut loader = Loader::new(&args.filename);

    let mut failure_file = OpenOptions::new()
        .append(false) // Enable append mode
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .open("failures.csv")?;

    let mut passed = 0;
    let mut total = 0;
    loop {
        println!("{},{}", total, total - passed);

        match loader.get_line() {
            Ok(data) => {
                if solve(&data) {
                    passed = passed + 1
                } else {
                    write!(&failure_file, "{}", data)?
                }
                total = total + 1
            }
            Err(_) => {
                println!("results = {}/{}", passed, total);
                exit(0);
            }
        }
    }
}
