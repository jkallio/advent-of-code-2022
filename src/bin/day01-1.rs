fn main() {
    if let Ok(calories) = read_input_file("input/day1.txt") {
        if let Some(max_calories) = calories.iter().max() {
            println!("Max calories = {}", max_calories);
        }
    }
}

fn read_input_file(file_path: &str) -> std::io::Result<Vec<i32>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut buf = Vec::<i32>::new();

    let mut total_calories: i32 = 0;
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            total_calories += line.parse::<i32>().unwrap();
        } else {
            buf.push(total_calories);
            total_calories = 0;
        }
    }
    buf.push(total_calories);
    Ok(buf)
}
