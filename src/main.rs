use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::io::prelude::*;
use std::convert::TryInto;

fn load_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn three_a() -> io::Result<()> {
    let contents = load_input("src/day3input.txt").unwrap();

    type Point = (usize, usize);
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 2000]; 2000];
    let mut crosses: Vec<Point> = vec![];

    let wires = contents
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<String>().unwrap())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut cursor : Point = (1000, 1000);
    for instruction in &wires[0] {
        let dir = &instruction[0..1];
        let steps = &instruction[1..].parse::<usize>().unwrap();
        match dir {
            "L" => {
                let start = cursor.0 - steps;
                let end = cursor.0;
                let y = cursor.1;
                println!("Left {}, {}", start, end);
                for x in start..=end {
                    grid[x][y] = true;
                }
            },
            "R" => {
                let start = cursor.0;
                let end = cursor.0 + steps;
                let y = cursor.1;
                println!("Right {}, {}", start, end);
                for x in start..=end {
                    grid[x][y] = true;
                }
            },
            "U" => {
                let start = cursor.1;
                let end = cursor.1 + steps;
                let x = cursor.0;
                println!("Up {}, {}", start, end);
                for y in start..=end {
                    grid[x][y] = true;
                }
            },
            "D" => {
                let start = cursor.1 - steps;
                let end = cursor.1;
                let x = cursor.0;
                println!("Down {}, {}", start, end);
                for y in start..=end {
                    grid[x][y] = true;
                }
            },
            _ => {},
        }
    }

    for instruction in &wires[1] {
        let dir = &instruction[0..1];
        let steps = &instruction[1..].parse::<usize>().unwrap();
        match dir {
            "L" => {
                let start = cursor.0 - steps;
                let end = cursor.0;
                let y = cursor.1;
                println!("Left {}, {}", start, end);
                for x in start..=end {
                    if grid[x][y] {
                        crosses.push((x, y));
                    }
                }
            },
            "R" => {
                let start = cursor.0;
                let end = cursor.0 + steps;
                let y = cursor.1;
                println!("Right {}, {}", start, end);
                for x in start..=end {
                    if grid[x][y] {
                        crosses.push((x, y));
                    }

                }
            },
            "U" => {
                let start = cursor.1;
                let end = cursor.1 + steps;
                let x = cursor.0;
                println!("Up {}, {}", start, end);
                for y in start..=end {
                    if grid[x][y] {
                        crosses.push((x, y));
                    }
                }
            },
            "D" => {
                let start = cursor.1 - steps;
                let end = cursor.1;
                let x = cursor.0;
                println!("Down {}, {}", start, end);
                for y in start..=end {
                    if grid[x][y] {
                        crosses.push((x, y));
                    }
                }
            },
            _ => {},
        }
    }

    let mut min = 1000;
    for (x, y) in crosses {
        if x == 1000 && y == 1000 {
            continue;
        }
        let offset_x: i32 = x.try_into().unwrap();
        let offset_y: i32 = y.try_into().unwrap();
        let manh_dist = (offset_x - 1000).abs() + (offset_y - 1000).abs();
        min = std::cmp::min(min, manh_dist);
        println!("manhattain distance for {} + {} is {}, new min is {}", offset_x - 1000, offset_y - 1000, manh_dist, min);
    }


    Ok(())
}




struct IntcodeProgramRunner {
    program: IntcodeProgram,
}

struct IntcodeProgram {
    values: Vec<u32>,
    curr: usize,
}

impl From<String> for IntcodeProgram {
    fn from(program: String) -> Self {
        let values = program.split(",")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        IntcodeProgram {
            values,
            curr: 0,
        }
    }
}

impl Iterator for IntcodeProgram {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let opcode = self.values[self.curr];
        let pos1 : usize = self.values[self.curr + 1].try_into().unwrap();
        let pos2 : usize = self.values[self.curr + 2].try_into().unwrap();
        let pos3 : usize = self.values[self.curr + 3].try_into().unwrap();
        let new_next = self.curr + 4;
        self.curr = new_next;

        match opcode {
            1 => {
                self.values[pos3] = &(self.values[pos1]) + &(self.values[pos2]);
                Some(self.values[0])
            },
            2 => {
                self.values[pos3] = &(self.values[pos1]) * &(self.values[pos2]);
                Some(self.values[0])
            },
            99 => {
                None
            },
            _ => None // Err
        }
    }
}

impl IntcodeProgramRunner {
    fn execute(self) -> u32 {
        let mut result: u32 = 0;
        for iteration in self.program {
            result = iteration;
        }
        result
    }
}

fn two_a() -> io::Result<()> {
    let contents = load_input("src/day2input.txt").unwrap();
    let mut runner = IntcodeProgramRunner {
        program: IntcodeProgram::from(contents),
    };

    // restore the gravity assist program
    runner.program.values[1] = 12;
    runner.program.values[2] = 2;

    let result = runner.execute();

    println!("Day 2A: {}", result);

    Ok(())
}

fn two_b() -> io::Result<()> {
    let contents = load_input("src/day2input.txt").unwrap();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut runner = IntcodeProgramRunner {
                program: IntcodeProgram::from(contents.clone()),
            };

            runner.program.values[1] = noun;
            runner.program.values[2] = verb;

            let result = runner.execute();
            if result == 19690720 {
                println!("Day 2B: {}", (100 * noun) + verb);
                break;
            }

        }
    }

    Ok(())
}




fn calc_fuel(m: f64) -> f64 {
    (m / 3.0).trunc() - 2.0
}

fn one_b() -> io::Result<()> {
    let file = File::open("src/day1input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: f64 = 0.0;

    for line in reader.lines() {
        let fuel_for_module = calc_fuel(line.unwrap().parse::<f64>().unwrap());
        let mut total_fuel = fuel_for_module;

        let mut fuel_for_fuel = fuel_for_module;
        while calc_fuel(fuel_for_fuel) > 0.0 {
            fuel_for_fuel = calc_fuel(fuel_for_fuel);
            total_fuel += fuel_for_fuel;
        }

        sum += total_fuel;
    }

    println!("Day 1B: {}", sum);
    Ok(())
}


/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module, take
/// its mass, divide by three, round down, and subtract 2.
fn one_a() -> io::Result<()> {
    let file = File::open("src/day1input.txt")?;
    let reader = BufReader::new(file);
    let sum: f64 = reader.lines()
        .map(|m| { calc_fuel(m.unwrap().parse::<f64>().unwrap()) })
        .sum();

    println!("Day 1A: {}", sum);
    Ok(())
}

fn main() {
    one_a();
    one_b();
    two_a();
    two_b();
    three_a();
}
