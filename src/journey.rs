use std::path::Path;

use crate::{
    d00_aoc::InputReader, d01_sonar_deep::SonarDeep, d02_dive::Dive,
    d03_binary_diagnostic::BinaryDiagnostic, d04_giant_squid::GiantSquid,
    d05_hydrothermal_veture::HydroThermalVenture,
};

pub fn run_sonar_deep() {
    println!("Day 1 of Advent :: Sonar Deep");
    let input_path = Path::new("./inputs/d01_input.txt");
    let a = SonarDeep::from_file(input_path).unwrap();
    println!("  Sonar Deep increases = {}", a.measurements());
    println!(
        "  Sonar Deep increases on 3 window = {}",
        a.measurements_window_sum(3)
    );
}

pub fn run_dive() {
    println!("Day 2 of Advent :: Dive");
    let input_path = Path::new("./inputs/d02_input.txt");
    let a = Dive::from_file(input_path).unwrap();
    let dive_forward = a.forward();
    let dive_depth = a.wrong_depth();
    println!(
        "  Part 1 :: forward = {}, depth = {}, total = {}",
        dive_forward,
        dive_depth,
        dive_forward * dive_depth
    );
    let dive_depth = a.depth();
    println!(
        "  Part 2 :: forward = {}, depth = {}, total = {}",
        dive_forward,
        dive_depth,
        dive_forward * dive_depth
    );
}

pub fn run_binary_diagnostic() {
    println!("Day 3 of Advent :: Binary Diagnostic");
    let input_path = Path::new("./inputs/d03_input.txt");
    let a = BinaryDiagnostic::from_file(input_path).unwrap();
    println!(
        "  Binary Diagnostic Part 1 :: gama = {}, epsilon = {}, diagnostic = {}",
        a.gamma(),
        a.epsilon(),
        a.gamma() * a.epsilon()
    );
    println!(
        "  Binary Diagnostic Part 2 :: oxygen = {}, co2 = {}, diagnostic = {}",
        a.oxygen(),
        a.co2(),
        a.oxygen() * a.co2()
    );
}

pub fn run_giant_squid() {
    println!("Day 4 of Advent :: Giant Squid");
    let input_path = Path::new("./inputs/d04_input.txt");
    let mut a = GiantSquid::from_file(input_path).unwrap();
    println!(
        "  Bingo Part 1 :: winner code {}",
        a.find_first_winner_code()
    );
    println!(
        "  Bingo Part 2 :: winner code {}",
        a.find_last_winner_code()
    );
}

pub fn run_hydrothermal_venture() {
    println!("Day 5 of Advent :: Hydrothermal Venture");
    let input_path = Path::new("./inputs/d05_input.txt");
    let a = HydroThermalVenture::from_file(input_path).unwrap();
    println!("  Vents Part 1 :: overlaps {}", a.overlaps());
    println!("  Vents Part 2 :: overlaps diagonal {}", a.overlaps_diag());
}
