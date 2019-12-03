use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::convert::TryInto;

enum Opcode {
    Add,
    Multiply,
    Exit
}

struct IntcodeProgram {
    values: Vec<u32>,
    curr: usize,
    next: usize,
}

impl From<String> for IntcodeProgram {
    fn from(program: String) -> Self {
        let values = program.split(",")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        IntcodeProgram {
            values,
            curr: 0,
            next: 4,
        }
    }
}

impl Iterator for IntcodeProgram {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<[u32; 4]> {
        let new_next = self.curr + self.next;

        self.curr = new_next;
        self.next = 4;

        let opcode = self.values[self.curr];
        let start = self.curr;
        let end = self.curr + 4;

        println!("start: {}, end: {}", start, end);

        let mut subprogram = [0; 4];
        match opcode {
            1 | 2 => {
                subprogram.copy_from_slice(&self.values[start..end]);
                Some(subprogram)
            },
            99 => None,
            _ => None // Err
        }
    }
}

impl IntcodeProgram {
    fn execute(mut self) {
        // println!("> {:?}", self.next()[0]);

        for subprogram in self {
            let opcode = subprogram[0]; 
            let pos1 : usize = subprogram[1].try_into().unwrap(); 
            let pos2 : usize = subprogram[2].try_into().unwrap(); 
            let pos3 : usize = subprogram[3].try_into().unwrap();
            match opcode {
                1 => {
                    self.values[pos3] = &(self.values[pos1]) + &(self.values[pos2]);
                },
                2 => {
                    self.values[pos3] = &(self.values[pos1]) * &(self.values[pos2]);
                },
                _ => ()
            }

            println!("> Opcode: {} on {} {} {}", opcode, pos1, pos2, pos3);
        }

        
    }
}

fn two_a() -> io::Result<()> {
    let mut file = File::open("src/day2input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let program = IntcodeProgram::from(contents);
    program.execute();    

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
    println!("Hello, world!");
    // one_a();
    // one_b();
    two_a();
}
