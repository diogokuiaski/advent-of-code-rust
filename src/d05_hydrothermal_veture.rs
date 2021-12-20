use core::fmt;
use std::{collections::HashMap, error::Error, fs, path::Path};

use crate::d00_aoc::InputReader;

#[derive(Clone, PartialEq)]
pub struct Line {
    p1: (usize, usize),
    p2: (usize, usize),
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Line")
            .field("p1", &self.p1)
            .field("p2", &self.p2)
            .finish()
    }
}

pub struct HydroThermalVenture {
    vents: HashMap<(usize, usize), i32>,
    vents_diag: HashMap<(usize, usize), i32>,
}

impl InputReader<Line> for HydroThermalVenture {
    fn string_to_vector(input_str: String) -> Vec<Line> {
        input_str
            .split("\n")
            .into_iter()
            .map(|f| {
                let p = f
                    .split(" -> ")
                    .map(|a| {
                        let a = a
                            .split(",")
                            .map(|f| -> usize { f.parse().unwrap() })
                            .collect::<Vec<usize>>();
                        (a[0], a[1])
                    })
                    .collect::<Vec<(usize, usize)>>();
                Line { p1: p[0], p2: p[1] }
            })
            .collect()
    }

    fn from_file(input_filepath: &Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let input_str = match fs::read_to_string(input_filepath) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };

        let v = Self::string_to_vector(input_str);

        Ok(HydroThermalVenture::new(v))
    }
}

impl HydroThermalVenture {
    pub fn new(lines: Vec<Line>) -> HydroThermalVenture {
        let mut vents_diag = HashMap::new();
        let mut vents = HashMap::new();
        for i in lines {
            vents = Self::fill_line(vents, &i, false);
            vents_diag = Self::fill_line(vents_diag, &i, true);
        }
        HydroThermalVenture { vents, vents_diag }
    }

    pub fn overlaps(&self) -> i32 {
        self.vents.iter().filter(|f| *f.1 > 1).count() as i32
    }

    pub fn overlaps_diag(&self) -> i32 {
        self.vents_diag.iter().filter(|f| *f.1 > 1).count() as i32
    }

    fn fill_line(
        vents: HashMap<(usize, usize), i32>,
        line: &Line,
        diagonal: bool,
    ) -> HashMap<(usize, usize), i32> {
        let mut vents = vents;
        if line.p1.0 == line.p2.0 {
            for i in Self::swap_lower(line.p1.1, line.p2.1) {
                *vents.entry((i, line.p1.0)).or_insert(0) += 1;
            }
        } else if line.p1.1 == line.p2.1 {
            for i in Self::swap_lower(line.p1.0, line.p2.0) {
                *vents.entry((line.p1.1, i)).or_insert(0) += 1;
            }
        } else if diagonal {
            let d1 = (line.p1.0 as i32 - line.p2.0 as i32).abs();
            let d2 = (line.p1.1 as i32 - line.p2.1 as i32).abs();
            if d1 == d2 {
                let id0 = Self::swap_lower(line.p1.0, line.p2.0);
                let id1 = Self::swap_lower(line.p1.1, line.p2.1);
                for (i, j) in id0.zip(id1) {
                    *vents.entry((j, i)).or_insert(0) += 1;
                }
            }
        }
        vents
    }

    fn swap_lower(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
        if a < b {
            Box::new(a..=b)
        } else {
            Box::new((b..=a).rev())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::d00_aoc::InputReader;

    use super::{HydroThermalVenture, Line};

    fn get_lines() -> Vec<Line> {
        [
            Line {
                p1: (0, 9),
                p2: (5, 9),
            },
            Line {
                p1: (8, 0),
                p2: (0, 8),
            },
            Line {
                p1: (9, 4),
                p2: (3, 4),
            },
            Line {
                p1: (2, 2),
                p2: (2, 1),
            },
            Line {
                p1: (7, 0),
                p2: (7, 4),
            },
            Line {
                p1: (6, 4),
                p2: (2, 0),
            },
            Line {
                p1: (0, 9),
                p2: (2, 9),
            },
            Line {
                p1: (3, 4),
                p2: (1, 4),
            },
            Line {
                p1: (0, 0),
                p2: (8, 8),
            },
            Line {
                p1: (5, 5),
                p2: (8, 2),
            },
        ]
        .to_vec()
    }

    #[test]
    fn test_overlap() {
        let lines = get_lines();
        let htv = HydroThermalVenture::new(lines);
        assert_eq!(htv.overlaps(), 5);
        assert_eq!(htv.overlaps_diag(), 12);
    }

    #[test]
    fn test_hydrothermal_venture() {
        let b = [
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];
        let mut d = HashMap::new();
        for i in 0..10 {
            for j in 0..10 {
                if b[i][j] > 0 {
                    d.insert((i, j), b[i][j]);
                }
            }
        }
        let lines = get_lines();
        let htv = HydroThermalVenture::new(lines);
        assert_eq!(d, htv.vents);
    }

    #[test]
    fn test_hydrothermal_venture_diagonal() {
        let b = [
            [1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            [0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            [0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            [0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            [0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];
        let mut d = HashMap::new();
        for i in 0..10 {
            for j in 0..10 {
                if b[i][j] > 0 {
                    d.insert((i, j), b[i][j]);
                }
            }
        }
        let lines = get_lines();
        let htv = HydroThermalVenture::new(lines);
        assert_eq!(d, htv.vents_diag);
    }

    #[test]
    fn test_string_to_vec() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let b = get_lines();
        let lines = HydroThermalVenture::string_to_vector(input.to_string());
        let equal_count = lines.iter().zip(&b).filter(|&(a, b)| a == b).count();

        assert_eq!(lines.len(), equal_count);
        assert_eq!(b.len(), equal_count);
    }
}
