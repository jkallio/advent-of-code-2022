use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn get_hash(&self) -> i32 {
        1000 * self.y + self.x
    }

    fn distance_to(&self, point: &Point) -> f32 {
        f32::sqrt(((point.x - self.x).pow(2) + (point.y - self.y).pow(2)) as f32)
    }
}

/*
struct Grid {
    width: usize,
    height: usize,
    cells: HashMap<i32, bool>,
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    let commands = read_lines_from_file("input/day9.txt")?;

    let mut head_pos = Point::new(0, 0);
    let mut head_route = vec![];
    let mut tail_pos = Point::new(0, 0);
    let mut tail_visited = HashMap::<i32, bool>::new();
    tail_visited.entry(tail_pos.get_hash()).or_insert(true);

    println!("Press Return");
    for (dir, steps) in commands {
        println!("{:?} - {} steps", dir, steps);
        for _ in 0..steps {
            head_route.push(head_pos.clone());
            match dir {
                Direction::Up => head_pos.y += 1,
                Direction::Down => head_pos.y -= 1,
                Direction::Left => head_pos.x -= 1,
                Direction::Right => head_pos.x += 1,
            }

            print!("  Head=({},{})", head_pos.x, head_pos.y);

            let distance = tail_pos.distance_to(&head_pos);
            if distance > 1.5 {
                tail_pos = head_route.last().unwrap().clone();
                tail_visited.entry(tail_pos.get_hash()).or_insert(true);
                print!(
                    "; Tail=({},{}); Distance={}",
                    tail_pos.x, tail_pos.y, distance
                );
            }
            println!();
        }
    }
    println!("Tail visited {} positions", tail_visited.len());

    Ok(())
}
fn read_lines_from_file(file_path: &str) -> std::io::Result<Vec<(Direction, i32)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut commands = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("Invalid direction");
            }
        };
        commands.push((dir, parts[1].parse::<i32>().unwrap()));
    }
    Ok(commands)
}
