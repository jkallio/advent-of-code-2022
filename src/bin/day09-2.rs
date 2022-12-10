use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Simple enum for Directions in input file
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Struct for simple point in (x,y) map
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// Implementation for Point
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    // Get simple hash identifier for (x,y) pair so that they can be
    // stored in HashMap
    fn get_hash(&self) -> i32 {
        1000 * self.y + self.x
    }

    fn distance_to(&self, point: &Point) -> f32 {
        f32::sqrt(((point.x - self.x).pow(2) + (point.y - self.y).pow(2)) as f32)
    }
}

/// Debug print the rope in a grid
fn draw_rope(rope: &[Point]) {
    struct Grid {
        width: usize,
        height: usize,
        cells: HashMap<i32, char>,
    }
    let width = 15;
    let height = 15;

    let mut grid = Grid {
        width,
        height,
        cells: HashMap::<i32, char>::new(),
    };

    for knot in rope {
        let mut x = knot.x % width as i32;
        if x < 0 {
            x += width as i32;
        }
        let mut y = knot.y % height as i32;
        if y < 0 {
            y += width as i32;
        }
        grid.cells.entry(grid.width as i32 * y + x).or_insert('#');
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            let key: i32 = grid.width as i32 * y as i32 + x as i32;
            if let Some(c) = grid.cells.get(&key) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/// Main function
fn main() -> Result<(), Box<dyn Error>> {
    use std::cmp::Ordering;
    use std::{thread, time};

    // Parse the input file
    let commands = read_input_file("input/day9.txt")?;

    // Initialize the fixed lenght rope
    let mut rope = vec![];
    let rope_len = 10;
    for _ in 0..rope_len {
        rope.push(Point::new(10, 10));
    }

    // The solution is to find the route that the last knot (tail) takes
    let mut tail_route = HashMap::<i32, bool>::new();

    // Iterate over each command from input file
    for (dir, steps) in commands {
        for _ in 0..steps {
            // Move the head knot of the rope based on input instructions
            let mut head = rope.first_mut().unwrap();
            match dir {
                Direction::Up => head.y -= 1,
                Direction::Down => head.y += 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }

            // Iterate over each knot in the rope and move each knot based
            // on the previous knot position
            let mut prev = head.clone();
            for i in 1..rope.len() {
                let mut knot = rope.get_mut(i).unwrap();
                let distance = knot.distance_to(&prev);

                // If euclidean distance is greater than 1 square (includindg
                // diagonal distance) move the next knot into the direction
                // of the previous knot
                if distance > 1.5 {
                    knot.x += match prev.x.cmp(&knot.x) {
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                        _ => 0,
                    };

                    knot.y += match prev.y.cmp(&knot.y) {
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                        _ => 0,
                    };
                }
                prev = knot.clone();

                // Debug print rope movement
                draw_rope(&rope);
                print!("{}[2J", 27 as char);
                thread::sleep(time::Duration::from_millis(10));
            }

            // Mark tail position
            let tail = rope.last_mut().unwrap();
            tail_route.entry(tail.get_hash()).or_insert(true);
        }
    }

    // Print the final result
    println!("Tail visited {} positions", tail_route.len());

    Ok(())
}

/// Parse the input file as List of Directions and Steps
fn read_input_file(file_path: &str) -> std::io::Result<Vec<(Direction, i32)>> {
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
