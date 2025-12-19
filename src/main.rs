
#[derive (Clone, Copy)]
struct Cell {
    possible_values: [bool; 9],
}

impl Cell {
    fn new() -> Cell{
        let vals: [bool; 9] = [true; 9];
        Cell{possible_values: vals}
    }
}

struct Game {
    cells: [Cell; 81]
}

impl Game {
    fn new() -> Game {
        let cells: [Cell; 81] = [Cell::new(); 81];
        Game{ cells }
            
    }
}

fn main() {
    let _game = Game::new();
    println!("Created Sudoku Game")
}
