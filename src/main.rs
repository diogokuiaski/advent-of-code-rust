use std::path::Path;

use advent_of_code_rust::d01_sonar_deep::SonarDeep;
use advent_of_code_rust::d02_dive::Dive;

fn main() {
    println!("Day 1 of Advent :: Sonar Deep");
    let input_path = Path::new("./inputs/d01_input.txt");
    let a = SonarDeep::from_file(input_path).unwrap();

    println!("Sonar Deep increases = {}", a.measurements());
    println!(
        "Sonar Deep increases on 3 window = {}",
        a.measurements_window_sum(3)
    );

    println!("Day 2 of Advent :: Dive");
    let input_path = Path::new("./inputs/d02_input.txt");
    let a = Dive::from_file(input_path).unwrap();

    let dive_forward = a.forward();
    let dive_depth = a.wrong_depth();
    println!(
        "Dive Path Part 1 :: forward = {}, depth = {}, total = {}",
        dive_forward,
        dive_depth,
        dive_forward * dive_depth
    );

    let dive_depth = a.depth();
    println!(
        "Dive Path Part 2 :: forward = {}, depth = {}, total = {}",
        dive_forward,
        dive_depth,
        dive_forward * dive_depth
    );
}
