use std::{error::Error, fmt, fs, path::Path, str::FromStr};

#[derive(PartialEq, Eq)]
enum Command {
    Forward = 0,
    Down,
    Up,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self {
            Command::Forward => "forward",
            Command::Down => "down",
            Command::Up => "up",
        };
        write!(f, "{}", r)
    }
}

pub struct DiveCommand {
    command: Command,
    value: i32,
}

pub struct Dive {
    commands: Vec<DiveCommand>,
}

impl Dive {
    fn string_to_command(input_str: &str) -> Command {
        match input_str {
            "forward" => Command::Forward,
            "down" => Command::Down,
            "up" => Command::Up,
            _ => panic!("Error"),
        }
    }

    fn string_to_vector(input_str: String) -> Vec<DiveCommand> {
        let mut input_vec: Vec<DiveCommand> = vec![];
        for line in input_str.split("\n") {
            let e = line.split(" ").collect::<Vec<&str>>();
            input_vec.push(DiveCommand {
                command: Self::string_to_command(e[0]),
                value: FromStr::from_str(e[1]).unwrap(),
            });
        }
        input_vec
    }

    pub fn new(commands: Vec<DiveCommand>) -> Dive {
        Dive { commands }
    }

    pub fn from_file(input_path: &Path) -> Result<Dive, Box<dyn Error>> {
        let input_str = match fs::read_to_string(input_path) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };
        let commands = Self::string_to_vector(input_str);
        Ok(Dive { commands })
    }

    pub fn forward(&self) -> i32 {
        self.commands
            .iter()
            .filter(|a| a.command == Command::Forward)
            .map(|a| a.value)
            .sum()
    }

    pub fn wrong_depth(&self) -> i32 {
        self.commands
            .iter()
            .filter(|a| a.command != Command::Forward)
            .map(|a| {
                if a.command == Command::Up {
                    a.value * -1
                } else {
                    a.value
                }
            })
            .sum()
    }

    pub fn depth(&self) -> i32 {
        let mut depth = 0;
        let mut aim = 0;
        for a in &self.commands {
            match a.command {
                Command::Up => aim -= a.value,
                Command::Down => aim += a.value,
                Command::Forward => depth += aim * a.value,
            }
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use crate::d02_dive::{Command, Dive, DiveCommand};

    fn get_commands() -> Vec<DiveCommand> {
        // forward 5
        // down 5
        // forward 8
        // up 3
        // down 8
        // forward 2
        let mut dive_commands = vec![];
        dive_commands.push(DiveCommand {
            command: Command::Forward,
            value: 5,
        });
        dive_commands.push(DiveCommand {
            command: Command::Down,
            value: 5,
        });
        dive_commands.push(DiveCommand {
            command: Command::Forward,
            value: 8,
        });
        dive_commands.push(DiveCommand {
            command: Command::Up,
            value: 3,
        });
        dive_commands.push(DiveCommand {
            command: Command::Down,
            value: 8,
        });
        dive_commands.push(DiveCommand {
            command: Command::Forward,
            value: 2,
        });
        dive_commands
    }

    #[test]
    fn test_dive_forward() {
        let dive_commands = get_commands();
        let dive = Dive::new(dive_commands);
        assert_eq!(dive.forward(), 15);
    }

    #[test]
    fn test_dive_wrong_depth() {
        let dive_commands = get_commands();
        let dive = Dive::new(dive_commands);
        assert_eq!(dive.wrong_depth(), 10);
    }

    #[test]
    fn test_dive_depth() {
        let dive_commands = get_commands();
        let dive = Dive::new(dive_commands);
        assert_eq!(dive.depth(), 60);
    }

    #[test]
    fn test_string_to_vec() {
        let input = get_commands();
        let str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let r = Dive::string_to_vector(str.to_string());
        assert_eq!(r.len(), input.len());
        for i in 0..r.len() {
            assert_eq!(r[i].command, input[i].command);
            assert_eq!(r[i].value, input[i].value);
        }
    }
}
