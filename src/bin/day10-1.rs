use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

///
/// Instruction trait
///
trait Instruction {
    fn name(&self) -> String;
    fn next_step(&mut self, register: &mut i32);
    fn is_ready(&self) -> bool;
}

impl fmt::Display for dyn Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

///
/// Noop instruction
///
struct Noop {
    executed: bool,
}

impl Noop {
    fn new() -> Self {
        Self { executed: false }
    }
}
impl Instruction for Noop {
    fn name(&self) -> String {
        "noop".to_string()
    }

    fn next_step(&mut self, _: &mut i32) {
        self.executed = true;
    }

    fn is_ready(&self) -> bool {
        self.executed
    }
}

///
/// Add Instrcution
///
struct Add {
    cycle: usize,
    value: i32,
}
impl Add {
    fn new(value: i32) -> Self {
        Add { cycle: 0, value }
    }
}

impl Instruction for Add {
    fn name(&self) -> String {
        "add".to_string()
    }

    fn next_step(&mut self, register: &mut i32) {
        if self.cycle == 1 {
            *register += self.value;
        }
        self.cycle += 1;
    }

    fn is_ready(&self) -> bool {
        self.cycle > 1
    }
}

///
/// CPU
///
struct Cpu {
    x: i32,
    instructions: Vec<Box<dyn Instruction>>,
}

impl Cpu {
    fn new(instructions: Vec<Box<dyn Instruction>>) -> Self {
        Cpu { x: 1, instructions }
    }
    fn run(&mut self) -> i32 {
        let mut result = 0;

        let mut i = 1;
        for instruction in self.instructions.iter_mut() {
            while !instruction.is_ready() {
                if i != 40 && (i == 20 || (i + 20) % 40 == 0) {
                    let signal_strength = i as i32 * self.x;
                    println!(
                        "#{}: SIGNAL STRENGTH ({} * {} = {})",
                        i, i, self.x, signal_strength
                    );
                    result += signal_strength;
                }

                instruction.next_step(&mut self.x);
                i += 1;
            }
        }
        result
    }
}

///
/// Main
///
fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_instructions_from_input_file("input/day10.txt")?;
    let mut cpu = Cpu::new(instructions);
    let result = cpu.run();
    println!("\r\nResult = {}", result);
    Ok(())
}

/// Read instructions from input file
fn read_instructions_from_input_file(
    file_path: &str,
) -> std::io::Result<Vec<Box<dyn Instruction>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut instructions: Vec<Box<dyn Instruction>> = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "addx" => instructions.push(Box::new(Add::new(parts[1].parse::<i32>().unwrap()))),
            "noop" => instructions.push(Box::new(Noop::new())),
            _ => {
                panic!("Unknown instruction");
            }
        }
    }
    Ok(instructions)
}
