use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Wrapper struct for holding walked path
#[derive(PartialEq, Eq)]
struct Path {
    path: Vec<Point>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.path.len().cmp(&self.path.len())
    }
}

/// Point in 2D space
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

/// Implement Point struct
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub const ZERO: Point = Point { x: 0, y: 0 };
}

/// Heightmap with start and end points
struct HeightMap {
    map: HashMap<u64, u32>,
    start: Vec<Point>,
    end: Point,
}

/// Function for hashing x,y coordinates
fn hash(p: &Point) -> u64 {
    ((p.x as u64) << 32) | (p.y as u64)
}

/// Function fo dehashing x,y coordinates
fn dehash(hash: u64) -> Point {
    let x = (hash >> 32) as i32;
    let y = (hash & 0xFFFFFFFF) as i32;
    Point::new(x, y)
}

/// Find min and max x and y coordinates
fn find_min_max(map: &HashMap<u64, u32>) -> (Point, Point) {
    let mut min = Point::ZERO;
    let mut max = Point::ZERO;
    for key in map.keys() {
        let p = dehash(*key);
        if p.x < min.x {
            min.x = p.x;
        }
        if p.x > max.x {
            max.x = p.x;
        }
        if p.y < min.y {
            min.y = p.y;
        }
        if p.y > max.y {
            max.y = p.y;
        }
    }
    (Point { x: min.x, y: min.y }, Point { x: max.x, y: max.y })
}

/// Print function for graph map where key is hash of x,y coordinates
/// and value tells the height in current position
fn print_graph_map(map: &HashMap<u64, u32>) {
    let (min, max) = find_min_max(map);

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let key = hash(&Point::new(x, y));
            if let Some(height) = map.get(&key) {
                if *height >= 10 {
                    print!("{}", (65 + (*height - 10)) as u8 as char);
                } else {
                    print!("{}", height);
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/// Read input file as graph map
fn read_graph_map(filename: &str) -> Result<HeightMap, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<u64, u32> = HashMap::new();
    let mut start = vec![];
    let mut end = Point::ZERO;

    // Read the file one line at a time
    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        // Read line one character at a time
        for (x, c) in line.chars().enumerate() {
            let point = Point::new(x as i32, y as i32);
            let c = match &c {
                'a' | 'S' => {
                    start.push(point.clone());
                    'a'
                }
                'E' => {
                    end = point.clone();
                    'z'
                }
                _ => c,
            };
            let num = c as u32 - 97;
            map.insert(hash(&point), num);
        }
    }
    Ok(HeightMap { map, start, end })
}

/// Get neighbors of a point
fn get_neighbors(map: &HashMap<u64, u32>, point: Point) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let x = point.x;
    let y = point.y;
    let points = vec![
        Point::new(x - 1, y),
        Point::new(x + 1, y),
        Point::new(x, y - 1),
        Point::new(x, y + 1),
    ];
    for p in points {
        if map.contains_key(&hash(&p)) {
            neighbors.push(p);
        }
    }
    neighbors
}

/// Main function
fn main() {
    let height_map = read_graph_map("input/day12.txt").expect("Could not read input file");
    print_graph_map(&height_map.map);
    let mut shortest = std::u32::MAX as usize;
    for start in height_map.start {
        if let Some(shortest_path) = shortest_path(&height_map.map, &start, &height_map.end) {
            if shortest_path.path.len() < shortest {
                shortest = shortest_path.path.len();
                println!("Path len = {}", shortest_path.path.len() - 1);
            }
        }
    }
    println!("Shortest path: {}", shortest - 1);
}

/// Find shortest path between two points using dijkstra algorithm
fn shortest_path(map: &HashMap<u64, u32>, start: &Point, end: &Point) -> Option<Path> {
    let mut visited: HashSet<u64> = HashSet::new();
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();
    heap.push(Path { path: vec![*start] });
    while let Some(path) = heap.pop() {
        let point = path.path.last().unwrap().clone();
        let path = path.path;
        let point_height = map.get(&hash(&point)).unwrap();
        if point == *end {
            return Some(Path { path });
        }
        if visited.contains(&hash(&point)) {
            continue;
        }
        visited.insert(hash(&point));
        for neighbor in get_neighbors(map, point) {
            if !visited.contains(&hash(&neighbor)) {
                let neighbor_height = map.get(&hash(&neighbor)).unwrap();
                if neighbor_height > point_height && neighbor_height - point_height > 1 {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(neighbor);
                heap.push(Path { path: new_path });
            }
        }
    }
    None
}
