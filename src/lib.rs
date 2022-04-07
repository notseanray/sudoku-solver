use core::fmt;
use std::{path::PathBuf, fs};

pub struct Board {
    grid_size: (u8, u8),
    data: [[u8 ; 9]; 9],
    iter: usize
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid_size: (9, 9),
            data: [
                [7, 0, 2, 0, 5, 0, 6, 0, 0],
                [0, 0, 0, 0, 0, 3, 0, 0, 0],
                [1, 0, 0, 0, 0, 9, 5, 0, 0],
                [8, 0, 0, 0, 0, 0, 0, 9, 0],
                [0, 4, 3, 0, 0, 0, 7, 5, 0],
                [0, 9, 0, 0, 0, 0, 0, 0, 8],
                [0, 0, 9, 7, 0, 0, 0, 0, 5],
                [0, 0, 0, 2, 0, 0, 0, 0, 0],
                [0, 0, 7, 0, 4, 0, 2, 0, 3]
            ],
            iter: 0
        }
    }
}

impl Board {
    pub fn new(s: &str) -> Self {
        let lines: Vec<&str> = s.split("\n").collect();
        let mut data = Vec::with_capacity(lines.len() * lines.len()); 
        for line in &lines {
            let mut digits: Vec<u8> = line.split("").map(|x| x.parse::<u8>().unwrap_or(10)).collect();
            digits.retain(|x| x != &10);
            if digits.len() < 1 {
                continue;
            }
            if digits.len() != 9 {
                println!("{:#?}", digits);
                panic!("invalid input! expected 9 numbers, found {}", digits.len());
            }
            data.push(digits.try_into().unwrap());
        }
        Self {
            grid_size: (lines.len() as u8, lines.len() as u8),
            data: data.try_into().unwrap(),
            iter: 0
        }
    }
    pub fn solve(&mut self) -> bool {
        self.iter += 1;
        for r in 0..self.data.len() {
            for c in 0..self.data[0].len() {
                if self.data[r][c] > 0 {
                    continue;
                }
                for i in 1..=self.data.len() {
                    if self.check_valid(Some(r), Some(c), i as u8) {
                        self.data[r][c] = i as u8;
                        if self.solve() {
                            return true;
                        } else {
                            self.data[r][c] = 0;
                        }
                    }
                }
                return false;
            }
        }
        true
    }

    pub fn save_output(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn check_valid(&self, row: Option<usize>, column: Option<usize>, target: u8) -> bool {
        match (row, column) {
            (Some(r), Some(c)) => self.check_row(r, target) && self.check_column(c, target) && self.check_square((r, c), target),
            (Some(r), None) => self.check_row(r, target),
            (None, Some(c)) => self.check_column(c, target),
            _ => false
        }
    }

    fn check_row(&self, row: usize, target: u8) -> bool {
        for num in self.data[row] {
            if num == target {
                return false;
            }
        }
        true
    }

    fn check_column(&self, column: usize, target: u8) -> bool {
        for row in self.data {
            if row[column] == target {
                return false;
            }
        }
        true
    }

    fn check_square(&self, pos: (usize, usize), target: u8) -> bool {
        let local_box = (pos.0 - pos.0 % 3, pos.1 - pos.1 % 3);
        for r in 0..=2 {
            for c in 0..=2 {
                if self.data[local_box.0 + r][local_box.1 + c] == target {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::with_capacity((self.grid_size.0 * self.grid_size.1).into()); 
        for (x, r) in self.data.iter().enumerate() {
            for (y, c) in r.iter().enumerate() {
                let line = match y == self.data[0].len() - 1 {
                    true => c.to_string(),
                    _ => {
                        match y % 3 {
                            2 => c.to_string() + " | ",
                            _ => c.to_string() + " "
                        }
                    }
                }; 
                output.push_str(&line);
            }
            let divider = {
                let length = self.data[0].len() * 2 + self.data.len() % 2 + 2;
                let mut output = String::with_capacity(length);
                for _ in 0..length {
                    output.push_str("-");
                }
                output
            };
            if x != self.data.len() - 1 {
                output.push_str(&format!("\n{divider}\n"));
            }
        } 
        write!(f, "{output}\niterations: {}", self.iter)
    }
}

pub fn read_from_file(path: PathBuf) -> Board {
    let data = match fs::read_to_string(&path) {
        Ok(v) => v,
        _ => panic!("failed to read {:#?} into a string", path)
    };
    Board::new(&data.replace("|", "").replace(" ", "").replace("-", ""))
}
