use std::{collections::HashMap, fs};

use crate::d00_aoc::InputReader;

type Board = [[i32; 5]; 5];

struct Place {
    value: i32,
    position: (usize, usize),
    taken: bool,
}
type TakenMap = HashMap<i32, Place>;

pub struct GiantSquid {
    numbers: Vec<i32>,
    boards: Vec<(Board, TakenMap)>,
    winners: Vec<(usize, i32)>,
}

impl InputReader<Board> for GiantSquid {
    fn string_to_vector(input_str: String) -> Vec<Board> {
        let mut input_vec: Vec<Board> = vec![];
        for table in input_str.split("\n\n").into_iter().skip(1) {
            let mut board = [[0; 5]; 5];
            let mut i = 0;
            for line in table.split("\n") {
                line.split_whitespace()
                    .enumerate()
                    .for_each(|(j, b)| board[i][j] = b.parse().unwrap());
                i += 1;
            }
            input_vec.push(board);
        }
        input_vec
    }

    fn from_file(input_filepath: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let input_str = match fs::read_to_string(input_filepath) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };
        let number_str = input_str
            .split("\n\n")
            .into_iter()
            .take(1)
            .collect::<String>();
        let numbers = number_str.split(",").map(|n| n.parse().unwrap()).collect();
        let boards = Self::string_to_vector(input_str);

        Ok(GiantSquid::new(numbers, boards))
    }
}

impl GiantSquid {
    fn new(numbers: Vec<i32>, boards: Vec<Board>) -> GiantSquid {
        let b = boards
            .iter()
            .map(|a| {
                let mut map = TakenMap::new();
                for i in 0..5 {
                    for j in 0..5 {
                        map.insert(
                            a[i][j],
                            Place {
                                value: a[i][j],
                                position: (i, j),
                                taken: false,
                            },
                        );
                    }
                }
                (a.clone(), map)
            })
            .collect();
        GiantSquid {
            numbers,
            boards: b,
            winners: vec![],
        }
    }

    pub fn find_first_winner_code(&mut self) -> i32 {
        if self.numbers.len() < 7 {
            return -1;
        }
        for i in 0..5 {
            let n = self.numbers[i];
            self.boards.iter_mut().for_each(|f| match f.1.get_mut(&n) {
                Some(e) => e.taken = true,
                None => return,
            });
        }

        let mut i = 5;
        let mut n = 0;
        let mut board_winner = 0;
        while i < self.numbers.len() {
            n = self.numbers[i];
            self.boards.iter_mut().for_each(|f| match f.1.get_mut(&n) {
                Some(e) => e.taken = true,
                None => return,
            });
            i += 1;
            let winner = self.check_winner(n);
            if winner.is_none() {
                continue;
            };
            board_winner = winner.unwrap().first().unwrap().clone();
            break;
        }
        n * self.get_not_taken(board_winner)
    }

    pub fn find_last_winner_code(&mut self) -> i32 {
        if self.numbers.len() < 7 {
            return -1;
        }
        for i in 0..5 {
            let n = self.numbers[i];
            self.boards.iter_mut().for_each(|f| match f.1.get_mut(&n) {
                Some(e) => e.taken = true,
                None => return,
            });
        }

        let mut i = 5;
        let mut n = 0;
        while i < self.numbers.len() {
            n = self.numbers[i];
            self.boards.iter_mut().for_each(|f| match f.1.get_mut(&n) {
                Some(e) => e.taken = true,
                None => return,
            });
            i += 1;
            let winner = self.check_winner(n);
            if winner.is_none() {
                continue;
            };
            winner.unwrap().iter_mut().for_each(|a| {
                let i = *a;
                self.winners.push((i, self.get_not_taken(i)));
            });
            if self.winners.len() == self.boards.len() {
                break;
            }
        }
        n * self.winners.last().unwrap().1
    }

    fn check_winner(&self, n: i32) -> Option<Vec<usize>> {
        let mut result = vec![];

        self.boards.iter().enumerate().for_each(|(i, f)| {
            if self.winners.iter().find(|a| a.0 == i).is_some() {
                return;
            }
            let map = &f.1;
            let (i0, j0) = match map.get(&n) {
                Some(e) => e.position,
                None => return,
            };
            let table = &f.0;
            let mut row = true;
            let mut column = true;
            for x in 0..5 {
                let v1 = table[i0][x];
                let v2 = table[x][j0];
                row &= map.get(&v1).unwrap().taken;
                column &= map.get(&v2).unwrap().taken;
            }
            if row || column {
                result.push(i);
            }
        });
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn get_not_taken(&self, board_winner: usize) -> i32 {
        let (_, m) = &self.boards[board_winner];
        m.iter()
            .filter(|p| !p.1.taken)
            .fold(0, |acc, x| acc + x.1.value)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::d00_aoc::InputReader;

    use super::GiantSquid;

    fn get_board() -> GiantSquid {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let boards = vec![
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ],
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ],
        ];
        GiantSquid::new(numbers, boards)
    }

    fn get_string_board() -> String {
        String::from(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#,
        )
    }

    #[test]
    fn test_first_winner() {
        let mut squid = get_board();
        let result = squid.find_first_winner_code();
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_last_winner() {
        let mut squid = get_board();
        let result = squid.find_last_winner_code();
        assert_eq!(result, 1924);
    }

    #[test]
    fn test_string_to_vec() {
        let s = get_string_board();
        let s_boards = GiantSquid::string_to_vector(s);

        let squid = get_board();
        let boards = squid.boards;

        assert_eq!(boards.len(), s_boards.len());
        for k in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    assert_eq!(boards[k].0[i][j], s_boards[k][i][j]);
                }
            }
        }
    }

    #[test]
    fn test_input_read() {
        let squid_load =
            GiantSquid::from_file(Path::new("src/test_inputs/d04_test_input.txt")).unwrap();

        let squid = get_board();
        let boards = squid.boards;
        let s_boards = squid_load.boards;

        assert_eq!(squid.numbers.len(), squid_load.numbers.len());
        for k in 0..boards.len() {
            assert_eq!(squid.numbers[k], squid_load.numbers[k]);
        }
        assert_eq!(boards.len(), s_boards.len());
        for k in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    assert_eq!(boards[k].0[i][j], s_boards[k].0[i][j]);
                }
            }
        }
    }
}
