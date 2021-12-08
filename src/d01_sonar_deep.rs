use std::{error::Error, fs, path::Path, str::FromStr};

pub struct SonarDeep {
    input: Vec<i32>,
}

impl SonarDeep {
    fn string_to_vector(input_str: String) -> Vec<i32> {
        let mut input_vec: Vec<i32> = vec![];
        for line in input_str.split("\n") {
            let e = FromStr::from_str(line).unwrap();
            input_vec.push(e)
        }
        input_vec
    }

    pub fn new(input: Vec<i32>) -> SonarDeep {
        SonarDeep { input }
    }

    pub fn from_file(input_filepath: &Path) -> Result<SonarDeep, Box<dyn Error>> {
        let input_str = match fs::read_to_string(input_filepath) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };
        let input = SonarDeep::string_to_vector(input_str);
        Ok(SonarDeep { input })
    }

    pub fn measurements(&self) -> i32 {
        let mut a = 0;
        let input = &self.input;
        for i in 1..input.len() {
            if input[i] > input[i - 1] {
                a += 1;
            }
        }
        a
    }

    pub fn measurements_window_sum(&self, window: usize) -> i32 {
        let input = &self.input;
        let mut a = 0;
        let mut last_acc = -1;
        for i in 0..=input.len() - window {
            let mut acc = 0;
            for v in &input[i..i + window] {
                acc += v
            }
            if last_acc > 0 && last_acc < acc {
                a += 1;
            }
            last_acc = acc;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::d01_sonar_deep::SonarDeep;

    #[test]
    fn test_sonar_deep() {
        let sonar = SonarDeep::new(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(sonar.measurements(), 7);
    }

    #[test]
    fn test_sonar_deep_window() {
        let sonar = SonarDeep::new(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(sonar.measurements_window_sum(3), 5);
    }

    #[test]
    fn test_string_to_vec() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        let r = SonarDeep::string_to_vector(str.to_string());
        assert_eq!(r.len(), input.len());
        for i in 0..r.len() {
            assert_eq!(r[i], input[i])
        }
    }
}
