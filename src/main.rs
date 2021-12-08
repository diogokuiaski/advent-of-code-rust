use std::path::Path;

use advent_of_code_rust::d01_sonar_deep::SonarDeep;

fn main() {
    println!("Day 1 of Advent :: Sonar Deep");

    let input_path = Path::new("./inputs/d01_input.txt");
    let a = SonarDeep::from_file(input_path).unwrap();

    println!("Sonar Deep increases = {}", a.measurements());
    println!(
        "Sonar Deep increases on 3 window = {}",
        a.measurements_window_sum(3)
    );
}
