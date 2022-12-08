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

fn is_visible(grid: &Grid<u32>, point: &Point) -> bool {
    if point.x == 0 || point.x == grid.width - 1 || point.y == 0 || point.y == grid.height - 1 {
        return true;
    } else {
        let val1 = grid.get(point).unwrap();
        // Up
        let mut y = point.y - 1;
        loop {
            if let Some(val2) = grid.get(&Point::new(point.x, y)) {
                if val2 >= val1 {
                    break;
                }
            }

            if y > 0 {
                y -= 1;
            } else {
                return true;
            }
        }
        // Down
        let mut y = point.y + 1;
        loop {
            if let Some(val2) = grid.get(&Point::new(point.x, y)) {
                if val2 >= val1 {
                    break;
                }
            }

            if y < grid.height {
                y += 1;
            } else {
                return true;
            }
        }
        // Left
        let mut x = point.x - 1;
        loop {
            if let Some(val2) = grid.get(&Point::new(x, point.y)) {
                if val2 >= val1 {
                    break;
                }
            }

            if x > 0 {
                x -= 1;
            } else {
                return true;
            }
        }
        // Right
        let mut x = point.x + 1;
        loop {
            if let Some(val2) = grid.get(&Point::new(x, point.y)) {
                if val2 >= val1 {
                    break;
                }
            }

            if x < grid.width {
                x += 1;
            } else {
                return true;
            }
        }
    }
    println!("{}, {} is not visible", point.x, point.y);
    false
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

        let mut visible_count = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if is_visible(&grid, &Point::new(x, y)) {
                    visible_count += 1;
                }
            }
        }
        println!("visibles = {}", visible_count);
    }
}

fn read_lines_from_file(file_path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
