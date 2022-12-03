use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    if let Ok(lines) = read_input_file("input/day2.txt") {
        let mut score = 0;
        for (rpc1, rpc2) in lines {
            println!("{:?},{:?}", rpc1, rpc2);
            score += match rpc2 {
                RPC::Rock => 1,
                RPC::Paper => 2,
                RPC::Scissors => 3,
            };

            score += if rpc1 == rpc2 {
                3
            } else if rpc1 == RPC::Rock && rpc2 == RPC::Paper
                || rpc1 == RPC::Paper && rpc2 == RPC::Scissors
                || rpc1 == RPC::Scissors && rpc2 == RPC::Rock
            {
                6
            } else {
                0
            };
        }
        println!("Total score = {}", score);
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<(RPC, RPC)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rpcs = vec![];
    for line in reader.lines() {
        let line = line?;
        let iter = line.split_whitespace();
        let mut left = RPC::Rock;
        let mut right = RPC::Rock;
        for (i, val) in iter.enumerate() {
            match i {
                0 => {
                    left = match val {
                        "A" => RPC::Rock,
                        "B" => RPC::Paper,
                        "C" => RPC::Scissors,
                        _ => {
                            panic!("Invalid ABC");
                        }
                    };
                }
                1 => {
                    right = match val {
                        "X" => RPC::Rock,
                        "Y" => RPC::Paper,
                        "Z" => RPC::Scissors,
                        _ => {
                            panic!("Invalid XYZ");
                        }
                    };
                }
                _ => {
                    panic!("Invalid amount of parts after split")
                }
            }
        }
        rpcs.push((left, right));
    }
    Ok(rpcs)
}
