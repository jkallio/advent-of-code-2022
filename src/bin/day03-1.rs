use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(lines) = read_input_file("input/day3.txt") {
        let mut sum = 0;
        for line in lines {
            'outer: for a in line.0.chars() {
                for b in line.1.chars() {
                    if a == b {
                        let priority = if a.is_uppercase() {
                            (a as i32) - 'A' as i32 + 27
                        } else {
                            (a as i32) - 'a' as i32 + 1
                        };
                        println!("{}: {}", a, priority);
                        sum += priority;
                        break 'outer;
                    }
                }
            }
        }
        println!("Sum of priorities = {}", sum);
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<(String, String)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut compartments = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts = line.split_at(line.len() / 2);
        assert!(parts.0.len() == parts.1.len());
        compartments.push((parts.0.to_string(), parts.1.to_string()));
    }
    Ok(compartments)
}
