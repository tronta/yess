use colored::*;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use Piece::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    NoStone,
    Stone(char),
}

pub struct Playfield {
    field: [Piece; 55],
    set_pieces: HashSet<usize>,
}

fn get_delta(y: usize) -> usize {
    match y {
        0 => 0,
        1 => 10,
        2 => 19,
        3 => 27,
        4 => 34,
        5 => 40,
        6 => 45,
        7 => 49,
        8 => 52,
        9 => 54,
        _ => panic!("{} is an invalid y value", y),
    }
}

impl Playfield {
    pub fn new() -> Playfield {
        Playfield {
            field: [NoStone; 55],
            set_pieces: HashSet::new(),
        }
    }

    fn set_stone(&mut self, x: usize, y: usize, e: Piece) -> bool {
        if x > 9 {
            return false;
        }
        if y > 9 - x {
            return false;
        }

        // Is there already a stone on that field?
        if let Stone(_) = self.field[get_delta(y) + x] {
            return false;
        }

        self.field[get_delta(y) + x] = e;
        true
    }

    pub fn field_empty(&self, x: usize, y: usize) -> bool {
        if x > 9 {
            return false;
        }
        if y > 9 - x {
            return false;
        }

        if let Stone(_) = self.field[get_delta(y) + x] {
            return false;
        } else {
            return true;
        }
    }

    /*    pub fn clear(&mut self) {
            self.field = [NoStone; 55];
            self.set_pieces.clear();
    //        self.counter = 0;
        }*/

    pub fn remove_piece(&mut self, e_nb: usize) {
        let stones = [
            Stone('A'),
            Stone('B'),
            Stone('C'),
            Stone('D'),
            Stone('E'),
            Stone('F'),
            Stone('G'),
            Stone('H'),
            Stone('I'),
            Stone('J'),
            Stone('K'),
            Stone('L'),
            Stone('T'),
        ];

        for i in 0..55 {
            if self.field[i] == stones[e_nb] {
                self.field[i] = NoStone;
            }
        }
    }

    pub fn set_piece(&mut self, co: (usize, usize), e_nb: usize, rot: u8) -> bool {
        // Check if piece has already been used, if yes remove this piece
        if self.set_pieces.contains(&e_nb) {
            self.remove_piece(e_nb);
        }

        let mut mirror = false;
        if rot > 3 {
            mirror = true;
        }

        if e_nb > 11 {
            panic!("{} is not a valid piece number!", e_nb);
        }

        let pieces = [
            [
                [Stone('A'), Stone('A'), Stone('A'), NoStone], // 0
                [Stone('A'), NoStone, NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('B'), Stone('B'), Stone('B'), NoStone], // 1
                [Stone('B'), Stone('B'), NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('C'), Stone('C'), Stone('C'), Stone('C')], // 2
                [Stone('C'), NoStone, NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('D'), Stone('D'), Stone('D'), Stone('D')], // 3
                [NoStone, Stone('D'), NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('E'), Stone('E'), Stone('E'), NoStone], // 4
                [NoStone, NoStone, Stone('E'), Stone('E')],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('F'), Stone('F'), NoStone, NoStone], // 5
                [Stone('F'), NoStone, NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('G'), Stone('G'), Stone('G'), NoStone], // 6
                [Stone('G'), NoStone, NoStone, NoStone],
                [Stone('G'), NoStone, NoStone, NoStone],
            ],
            [
                [Stone('H'), Stone('H'), NoStone, NoStone], // 7
                [NoStone, Stone('H'), Stone('H'), NoStone],
                [NoStone, NoStone, Stone('H'), NoStone],
            ],
            [
                [Stone('I'), Stone('I'), Stone('I'), NoStone], // 8
                [Stone('I'), NoStone, Stone('I'), NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('J'), Stone('J'), Stone('J'), Stone('J')], // 9
                [NoStone, NoStone, NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [Stone('K'), Stone('K'), NoStone, NoStone], // 10
                [Stone('K'), Stone('K'), NoStone, NoStone],
                [NoStone, NoStone, NoStone, NoStone],
            ],
            [
                [NoStone, Stone('L'), NoStone, NoStone], // 11
                [Stone('L'), Stone('L'), Stone('L'), NoStone],
                [NoStone, Stone('L'), NoStone, NoStone],
            ],
        ];

        let empty_rows_x = [1, 1, 0, 0, 0, 2, 1, 1, 1, 0, 2, 1]; // Number of rows which are empty on the right side
        let empty_rows_y = [1, 1, 1, 1, 1, 1, 0, 0, 1, 2, 1, 0]; // same

        let mut piece_or = [[NoStone; 4]; 4];

        // mirror piece on the x-achis if needed
        if mirror == true {
            for xs in 0..4 {
                for ys in 0..3 - empty_rows_y[e_nb] {
                    piece_or[2 - ys - empty_rows_y[e_nb]][xs] = pieces[e_nb][ys][xs];
                }
            }
        } else {
            for xs in 0..4 {
                for ys in 0..3 {
                    piece_or[ys][xs] = pieces[e_nb][ys][xs];
                }
            }
        }

        let mut piece = [[NoStone; 4]; 4];

        match rot % 4 {
            0 => {
                // No rotation
                for xs in 0..4 {
                    for ys in 0..3 {
                        piece[ys][xs] = piece_or[ys][xs];
                    }
                }
            }

            1 => {
                // Rotation 90° clockwise
                for xs in 0..4 {
                    for ys in 0..3 - empty_rows_y[e_nb] {
                        piece[xs][2 - ys - empty_rows_y[e_nb]] = piece_or[ys][xs];
                    }
                }
            }

            2 => {
                // Rotation 180° clockwise
                for xs in 0..4 - empty_rows_x[e_nb] {
                    for ys in 0..3 - empty_rows_y[e_nb] {
                        piece[2 - ys - empty_rows_y[e_nb]][3 - xs - empty_rows_x[e_nb]] =
                            piece_or[ys][xs];
                    }
                }
            }

            3 => {
                // Rotation 270° clockwise
                for xs in 0..4 - empty_rows_x[e_nb] {
                    for ys in 0..3 {
                        piece[3 - xs - empty_rows_x[e_nb]][ys] = piece_or[ys][xs];
                    }
                }
            }

            _ => {}
        }

        for xs in 0..4 {
            for ys in 0..4 {
                if let Stone(_) = piece[ys][xs] {
                    if self.field_empty(co.0 + xs, co.1 + ys) == false {
                        return false;
                    }
                }
            }
        }

        for xs in 0..4 {
            for ys in 0..4 {
                if let Stone(s) = piece[ys][xs] {
                    assert_eq!(self.set_stone(co.0 + xs, co.1 + ys, Stone(s)), true);
                }
            }
        }

        // remember that this piece has been set
        self.set_pieces.insert(e_nb);
        // successfull
        true
    }
}

impl fmt::Debug for Playfield {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut ret = 10;
        let mut ret_c = ret;
        for i in 0..55 {
            let _ = match &self.field[i] {
                NoStone => write!(f, "_,"),
                Stone(w) => write!(
                    f,
                    "{},",
                    match w {
                        'A' => "A".yellow(),
                        'B' => "B".red(),
                        'C' => "C".blue(),
                        'D' => "D".bright_magenta(),
                        'E' => "E".green(),
                        'F' => "F".cyan(),
                        'G' => "G".bright_blue(),
                        'H' => "H".bright_cyan(),
                        'I' => "I".bright_yellow(),
                        'J' => "J".on_magenta(),
                        'K' => "K".bright_green(),
                        'L' => "L".on_bright_black(),
                        _ => "X".bold(),
                    }
                ),
            };
            ret_c -= 1;
            if ret_c == 0 {
                ret -= 1;
                ret_c = ret;
                let _ = write!(f, "\n");
            }
        }
        write!(f, "")
    }
}
