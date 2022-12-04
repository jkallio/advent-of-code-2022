use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

fn main() {
    if let Ok(assignments) = read_input_file("input/day4.txt") {
        let mut count = 0;
        for ass in assignments {
            println!("{:?}", ass);
            if ass.0.min <= ass.1.max && ass.0.max >= ass.1.min {
                count += 1;
            }
        }
        println!("Total count = {}", count);
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<(Range, Range)>> {
    // Local function called to perform same operation twice
    fn split_sections(section: &str) -> Range {
        let parts: Vec<&str> = section.split('-').collect();
        Range {
            min: parts[0].parse::<u32>().unwrap(),
            max: parts[1].parse::<u32>().unwrap(),
        }
    }

    // Parse input file and return List of range pairs
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut assignments = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        assignments.push((split_sections(parts[0]), split_sections(parts[1])));
    }
    Ok(assignments)
}
