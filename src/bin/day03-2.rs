use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(groups) = read_input_file("input/day3.txt") {
        let mut sum = 0;

        for group in groups {
            if let Some(rucksack1) = group.first() {
                let mut badge_match_count = 0;
                'outer: for item in rucksack1.chars() {
                    'inner: for rucksack2 in &group[1..=2] {
                        if rucksack2.contains(item) {
                            badge_match_count += 1;
                            if badge_match_count == 2 {
                                let priority = if item.is_uppercase() {
                                    (item as i32) - 'A' as i32 + 27
                                } else {
                                    (item as i32) - 'a' as i32 + 1
                                };
                                println!("{}: {}", item, priority);
                                sum += priority;
                                break 'outer;
                            }
                        } else {
                            badge_match_count = 0;
                            break 'inner;
                        }
                    }
                }
            }
        }
        println!("Total priority: {}", sum);
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<Vec<String>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut groups = vec![];
    let mut group = vec![];
    for line in reader.lines() {
        let line = line?;
        group.push(line);
        if group.len() == 3 {
            groups.push(group.clone());
            group.clear();
        }
    }
    assert!(group.is_empty());
    Ok(groups)
}
