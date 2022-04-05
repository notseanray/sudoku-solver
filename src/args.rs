use std::path::PathBuf;

use sudoku_solver::{read_from_file, Board};

pub struct Args {
    output: Option<PathBuf>,
    input: Option<PathBuf>,
    total: Vec<String>,
    example: bool
}

impl Args {
    pub fn parse(args: Vec<String>) -> Self {
        let mut result = Self {
            output: None,
            input: None,
            total: Vec::new(),
            example: false
        };
        match args.len() {
            1 => {
                result.example = true;
                return result;
            },
            _ => {}
        };
        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-o" => {
                    if args.len() < i + 2 {
                        panic!("expected argument for -o");
                    }
                    result.output = Some(PathBuf::from(&args[i + 1]));
                },
                "-i" => {
                    if args.len() < i + 2 {
                        panic!("expected argument for -i");
                    }
                    result.input = Some(PathBuf::from(&args[i + 1]));
                },
                _ => result.total.push(arg.to_string())
            };
        }
        result
    }

    pub fn handle_board(&self) {
        let mut board = match self.example {
            true => Board::default(),
            _ => {
                match &self.input {
                    Some(v) => read_from_file(PathBuf::from(v)),
                    None => {
                        let mut board = String::with_capacity(81);
                        let mut sep = 0;
                        println!("{:#?}", self.total);
                        for i in self.total[1].split("").into_iter() {
                            sep += 1;
                            let data = match sep % 9 {
                                0 => format!("{i}\n"),
                                _ => i.to_string()
                            };
                            board.push_str(&data);
                        }
                        Board::new(&board)
                    }
                }
                    
            }
        };
        println!("Initial board: \n{board}");
        board.solve();
        println!("\nSolved:\n{board}");
    }
}
