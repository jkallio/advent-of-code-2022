use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum RPCResult {
    Lose,
    Draw,
    Win,
}

fn main() {
    if let Ok(lines) = read_input_file("input/day2.txt") {
        let mut score = 0;
        for (rpc1, result) in lines {
            let rpc2 = match result {
                RPCResult::Win => match rpc1 {
                    RPC::Rock => RPC::Paper,
                    RPC::Paper => RPC::Scissors,
                    RPC::Scissors => RPC::Rock,
                },
                RPCResult::Draw => rpc1,
                RPCResult::Lose => match rpc1 {
                    RPC::Rock => RPC::Scissors,
                    RPC::Paper => RPC::Rock,
                    RPC::Scissors => RPC::Paper,
                },
            };

            // Add result based on selected hand
            score += match rpc2 {
                RPC::Rock => 1,
                RPC::Paper => 2,
                RPC::Scissors => 3,
            };

            // Add result for win/draw/lose
            score += match result {
                RPCResult::Win => 6,
                RPCResult::Draw => 3,
                RPCResult::Lose => 0,
            };
        }
        println!("Total score = {}", score);
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<(RPC, RPCResult)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rpcs = vec![];
    for line in reader.lines() {
        let line = line?;
        let iter = line.split_whitespace();
        let mut rpc = RPC::Rock;
        let mut rpc_result = RPCResult::Lose;
        for (i, val) in iter.enumerate() {
            match i {
                0 => {
                    rpc = match val {
                        "A" => RPC::Rock,
                        "B" => RPC::Paper,
                        "C" => RPC::Scissors,
                        _ => {
                            panic!("Invalid ABC");
                        }
                    };
                }
                1 => {
                    rpc_result = match val {
                        "X" => RPCResult::Lose,
                        "Y" => RPCResult::Draw,
                        "Z" => RPCResult::Win,
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
        rpcs.push((rpc, rpc_result));
    }
    Ok(rpcs)
}
