use advent_of_code_rust::{
    journey::{run_binary_diagnostic, run_dive, run_sonar_deep},
    utils::Puzzle,
};

fn main() {
    let track_list = vec![
        Puzzle::run("sonar_deep", Box::new(&run_sonar_deep)),
        Puzzle::run("dive", Box::new(&run_dive)),
        Puzzle::run("binary_diagnostic", Box::new(&run_binary_diagnostic)),
    ];

    println!("");

    track_list.iter().enumerate().for_each(|(i, puzzle)| {
        println!(
            "Puzzle {0} [{1:^30}] elapsed {2:>6} us",
            i,
            puzzle.name(),
            puzzle.elapsed_time().as_micros()
        )
    })
}
