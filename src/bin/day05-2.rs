use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
struct Command {
    from: usize,
    to: usize,
    amount: u32,
}

#[derive(Default)]
struct InputData {
    pub board: HashMap<usize, VecDeque<char>>,
    pub commands: Vec<Command>,
}

fn main() {
    if let Ok(mut data) = parse_input_file("input/day5.txt") {
        for cmd in data.commands.iter() {
            let mut stack = vec![];

            if let Some(from_stack) = data.board.get_mut(&cmd.from) {
                for _ in 0..cmd.amount {
                    if let Some(ch) = from_stack.pop_back() {
                        stack.push(ch);
                    }
                }
            }

            if let Some(to_stack) = data.board.get_mut(&cmd.to) {
                for ch in stack.iter().rev() {
                    to_stack.push_back(*ch);
                }
            }

            // Debug print
            println!("\r\nmove {} from {} to {}", cmd.amount, cmd.from, cmd.to);
            for val in data.board.iter() {
                println!("{:?}", val);
            }
        }

        // Print the solution
        println!("\r\nThe solution is:");
        for i in 1..=9 {
            if let Some(stack) = data.board.get(&(i as usize)) {
                if let Some(c) = stack.back() {
                    print!("{}", c);
                }
            }
        }
        println!();
    } else {
        panic!("Failed to read input file");
    }
}

fn parse_input_file(file_path: &str) -> std::io::Result<InputData> {
    let file = File::open(file_path)?;
    let mut data = InputData::default();
    let mut lines = BufReader::new(file).lines();

    // Parse crate layout into map of deques
    while let Some(line) = lines.by_ref().next() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_alphabetic() {
                let key = ((i as f32 / 4.0) as f32).ceil() as usize;
                let v = data.board.entry(key).or_default();
                v.push_front(c);
            }
        }
    }

    // Parse command list
    while let Some(line) = lines.by_ref().next() {
        let line = line?;
        let parts: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        assert!(parts.len() == 6);

        data.commands.push(Command {
            from: parts[3].parse::<usize>().unwrap(),
            to: parts[5].parse::<usize>().unwrap(),
            amount: parts[1].parse::<u32>().unwrap(),
        });
    }
    Ok(data)
}
