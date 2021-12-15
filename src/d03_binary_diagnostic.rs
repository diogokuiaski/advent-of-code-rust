use std::{error::Error, f32::RADIX, fs, path::Path};

use crate::d00_aoc::InputReader;

pub struct BinaryDiagnostic {
    gamma: u32,
    epsilon: u32,
    oxygen: u32,
    co2: u32,
}

impl InputReader<Vec<u32>> for BinaryDiagnostic {
    fn string_to_vector(input_str: String) -> Vec<Vec<u32>> {
        let mut input_vec = vec![];
        for line in input_str.split("\n") {
            let e: Vec<u32> = line.chars().map(|a| a.to_digit(RADIX).unwrap()).collect();
            input_vec.push(e);
        }
        input_vec
    }

    fn from_file(input_filepath: &Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let input_str = match fs::read_to_string(input_filepath) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };
        let binary = Self::string_to_vector(input_str);
        Ok(BinaryDiagnostic::new(binary))
    }
}

impl BinaryDiagnostic {
    fn get_trick(search: Vec<Vec<u32>>, compare: fn(a: i32, b: i32) -> u32) -> u32 {
        let len = search[0].len();
        let mut search = search;
        for i in 0..len {
            let search_len = search.len() as i32;
            if search.len() > 1 {
                let h = search.iter().fold(vec![0; len], |mut acc, item| {
                    for i in 0..len {
                        let i = i as usize;
                        acc[i] += item[i] as i32;
                    }
                    acc
                });
                search = search
                    .into_iter()
                    .filter(|b| b[i] == compare(h[i], search_len - h[i]))
                    .collect();
            }
        }

        let trick = search[0]
            .iter()
            .enumerate()
            .map(|(i, a)| -> i32 {
                let x = (len - 1 - i) as u32;
                if a > &0 {
                    2_i32.pow(x)
                } else {
                    0
                }
            })
            .sum::<i32>() as u32;
        trick
    }

    pub fn new(diagnostic: Vec<Vec<u32>>) -> BinaryDiagnostic {
        if diagnostic.is_empty() {
            panic!("no way!")
        }
        let len = diagnostic[0].len() as i32;
        let diagnostic_len = diagnostic.len() as i32 / 2;
        let init = vec![0_i32; len as usize];
        let mut epsilon = 0;

        let hist = diagnostic.iter().fold(init, |mut acc, item| {
            for i in 0..len {
                let i = i as usize;
                acc[i] += item[i] as i32;
            }
            acc
        });

        let gamma = hist
            .iter()
            .enumerate()
            .map(|(i, a)| -> i32 {
                let x = (len - 1_i32 - i as i32) as u32;
                if a > &diagnostic_len {
                    2_i32.pow(x)
                } else {
                    epsilon += 2_i32.pow(x);
                    0
                }
            })
            .sum::<i32>() as u32;
        let epsilon = epsilon as u32;

        let oxygen = Self::get_trick(diagnostic.clone(), |a, b| if a >= b { 1 } else { 0 });

        let co2 = Self::get_trick(diagnostic.clone(), |a, b| if a >= b { 0 } else { 1 });

        BinaryDiagnostic {
            gamma,
            epsilon,
            oxygen,
            co2,
        }
    }

    pub fn gamma(&self) -> u32 {
        self.gamma
    }

    pub fn epsilon(&self) -> u32 {
        self.epsilon
    }

    pub fn oxygen(&self) -> u32 {
        self.oxygen
    }

    pub fn co2(&self) -> u32 {
        self.co2
    }
}

#[cfg(test)]
mod tests {
    use crate::{d00_aoc::InputReader, d03_binary_diagnostic::BinaryDiagnostic};

    #[test]
    fn test_string_to_vec() {
        let s = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let e = vec![
            vec![0, 0, 1, 0, 0_u32],
            vec![1, 1, 1, 1, 0_u32],
            vec![1, 0, 1, 1, 0_u32],
            vec![1, 0, 1, 1, 1_u32],
            vec![1, 0, 1, 0, 1_u32],
            vec![0, 1, 1, 1, 1_u32],
            vec![0, 0, 1, 1, 1_u32],
            vec![1, 1, 1, 0, 0_u32],
            vec![1, 0, 0, 0, 0_u32],
            vec![1, 1, 0, 0, 1_u32],
            vec![0, 0, 0, 1, 0_u32],
            vec![0, 1, 0, 1, 0_u32],
        ];
        let v = BinaryDiagnostic::string_to_vector(s);
        assert_eq!(e.len(), v.len());
        for i in 0..e.len() {
            assert_eq!(e[i].len(), v[i].len());
            for j in 0..e[i].len() {
                assert_eq!(e[i][j], v[i][j])
            }
        }
    }

    #[test]
    fn test_diagnostic() {
        let s = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let v = BinaryDiagnostic::string_to_vector(s);
        let diagnostic = BinaryDiagnostic::new(v);
        assert_eq!(diagnostic.gamma(), 22);
        assert_eq!(diagnostic.epsilon(), 9);
    }

    #[test]
    fn test_diagnostic_2() {
        let s = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let v = BinaryDiagnostic::string_to_vector(s);
        let diagnostic = BinaryDiagnostic::new(v);
        assert_eq!(diagnostic.oxygen(), 23);
        assert_eq!(diagnostic.co2(), 10);
    }
}
