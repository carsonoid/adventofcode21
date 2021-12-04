use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Board {
    rows: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn new() -> Board {
        Board { rows: Vec::new() }
    }
    pub fn add_row(&mut self, row: Vec<(u32, bool)>) -> bool {
        self.rows.push(row);
        true
    }
    pub fn print(&self) {
        for row in self.rows.iter() {
            for item in row {
                // if picked
                if item.1 {
                    print!("X{}", item.0);
                } else {
                    print!("{}", item.0);
                }
                print!(" ");
            }
            println!();
        }
    }
    pub fn is_complete(&self) -> bool {
        let mut columns = vec![true; self.rows[0].len()];
        for row in self.rows.iter() {
            let mut row_complete = true;
            for (i, item) in row.iter().enumerate() {
                // if picked
                if !item.1 {
                    columns[i] = false;
                    row_complete = false;
                }
            }
            if row_complete {
                return true;
            }
        }

        for column_complete in columns {
            if column_complete {
                return true;
            }
        }

        return false;
    }
    pub fn get_score(&self, n: u32) -> u32 {
        let mut s = 0;
        for (i, row) in self.rows.iter().enumerate() {
            for item in row {
                if !item.1 {
                    s += item.0;
                }
            }
        }
        return s * n;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("filename arg is required");
        return;
    }

    let filename = &args[1];

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut chosen_numbers: Vec<u32> = Vec::new();
    for num in lines[0].trim().split(",") {
        chosen_numbers.push(num.parse().expect("expected number"));
    }

    println!("{:?}\n", chosen_numbers);

    let mut boards: Vec<Board> = Vec::new();
    let mut board = Board::new();
    let mut num_rows = 0;
    for line in lines[2..lines.len()].iter() {
        if *line == "" {
            let mut add = false;
            if num_rows > 0 {
                add = true;
            }

            if add {
                boards.push(board.clone());
                board = Board::new();
                num_rows = 0;
            } else {
                continue;
            }
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        let mut row: Vec<(u32, bool)> = Vec::new();
        for num in parts {
            if num == "" {
                continue;
            }
            row.push((num.parse().expect("expected number"), false));
        }
        if row.len() > 0 {
            board.add_row(row);
        }
        num_rows += 1;
    }

    for board in boards.iter() {
        board.print();
        println!();
    }

    for chosen in chosen_numbers {
        for board in boards.iter_mut() {
            for row in board.rows.iter_mut() {
                for elem in row.iter_mut() {
                    if elem.0 == chosen {
                        println!("{} picked!", chosen);
                        elem.1 = true;
                    }
                }
            }
        }
        for (i, board) in boards.iter().enumerate() {
            if board.is_complete() {
                println!("board {} is complete!", i);
                board.print();
                println!("score: {}", board.get_score(chosen));
                return;
            }
        }
    }
}
