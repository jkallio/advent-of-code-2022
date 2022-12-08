use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Copy,
{
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..width * height).map(|_| T::default()).collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    fn point_to_index(&self, point: &Point) -> Option<usize> {
        if point.x >= self.width || point.y >= self.height {
            return None;
        }
        Some(point.y * self.width + point.x)
    }

    pub fn set(&mut self, point: &Point, value: T) -> Option<T> {
        if let Some(cell) = self.get_mut(point) {
            let old_value = *cell;
            *cell = value;
            return Some(old_value);
        }
        None
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.cells.get(self.point_to_index(point)?)
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        let idx = self.point_to_index(point)?;
        self.cells.get_mut(idx)
    }
}

fn get_scenic_score(grid: &Grid<u32>, point: &Point) -> u32 {
    let own_height = grid.get(point).unwrap();

    let mut trees_up = 0;
    if point.y > 0 {
        for y in (0..=point.y - 1).rev() {
            if let Some(tree_height) = grid.get(&Point::new(point.x, y)) {
                trees_up += 1;
                if tree_height >= own_height {
                    break;
                }
            }
        }
    }
    let mut trees_down = 0;
    for y in point.y + 1..grid.height {
        if let Some(tree_height) = grid.get(&Point::new(point.x, y)) {
            trees_down += 1;
            if tree_height >= own_height {
                break;
            }
        }
    }
    let mut trees_left = 0;
    if point.x > 0 {
        for x in (0..=point.x - 1).rev() {
            if let Some(tree_height) = grid.get(&Point::new(x, point.y)) {
                trees_left += 1;
                if tree_height >= own_height {
                    break;
                }
            }
        }
    }
    let mut trees_right = 0;
    for x in point.x + 1..grid.width {
        if let Some(tree_height) = grid.get(&Point::new(x, point.y)) {
            trees_right += 1;
            if tree_height >= own_height {
                break;
            }
        }
    }

    println!(
        "{},{} = {}, {}, {}, {}",
        point.x, point.y, trees_up, trees_down, trees_left, trees_right
    );
    trees_up * trees_down * trees_left * trees_right
}

fn main() {
    if let Ok(lines) = read_lines_from_file("input/day8.txt") {
        let width = lines.first().unwrap().len();
        let height = lines.len();
        let mut grid = Grid::<u32>::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.set(&Point::new(x, y), c.to_digit(10).unwrap());
            }
        }

        let mut hi_score = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let score = get_scenic_score(&grid, &Point::new(x, y));
                if score > hi_score {
                    hi_score = score;
                }
            }
        }
        println!("Highest scenic score = {}", hi_score);
    }
}

fn read_lines_from_file(file_path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
