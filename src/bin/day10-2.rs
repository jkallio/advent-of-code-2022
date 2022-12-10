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

struct Crt {
    width: usize,
    cursor: usize,
    pixels: Vec<bool>,
}

///
/// CRT
///
impl Crt {
    fn new(width: usize) -> Self {
        Crt {
            width,
            cursor: 1,
            pixels: vec![],
        }
    }

    fn step(&mut self, pixel_pos: i32) {
        let pixel_range = (pixel_pos - 1)..=(pixel_pos + 1);
        let lit = pixel_range.contains(&((self.cursor as i32 - 1) % self.width as i32));
        self.pixels.push(lit);
        self.cursor += 1;
    }

    fn draw(&self) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            let x = if *pixel { '#' } else { ' ' };
            print!("{}", x);
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }
}

///
/// CPU
///
struct Cpu {
    x: i32,
    crt: Crt,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            crt: Crt::new(40),
        }
    }

    /// Iterate over given set of instructions and run each cycle on every
    /// instruction.
    fn run(&mut self, instructions: &mut [Box<dyn Instruction>]) {
        for instruction in instructions.iter_mut() {
            while !instruction.is_ready() {
                self.crt.step(self.x);
                instruction.next_step(&mut self.x);
            }
        }
    }
}

///
/// Main
///
fn main() -> Result<(), Box<dyn Error>> {
    let mut instructions = read_instructions_from_input_file("input/day10.txt")?;
    let mut cpu = Cpu::new();
    cpu.run(&mut instructions);
    cpu.crt.draw();
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
