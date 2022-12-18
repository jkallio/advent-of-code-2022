use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// [`Element`] represents a single element in a list
#[derive(Clone)]
enum Element {
    Number(i32),
    List(Vec<Element>),
}

impl Element {
    fn is_number(&self) -> bool {
        matches!(self, Element::Number(_))
    }

    fn convert_to_list(&self) -> Vec<Element> {
        match self {
            Element::List(list) => list.clone(),
            Element::Number(number) => vec![Element::Number(*number)],
        }
    }
}

impl Eq for Element {}
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Element::Number(a), Element::Number(b)) => a == b,
            _ => false,
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Number(a), Element::Number(b)) => a.cmp(b),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::List(l) => {
                write!(f, "[")?;
                for (i, e) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", e)?;
                }
                write!(f, "]")
            }
        }
    }
}

/// Function that checks if two elements are in correct order using some rules
/// defined in the problem description
/// 1) If both elements are numbers, they're in right order if left is smaller
/// 2) If `left` list runs out of elements first, they're in right order
/// 3) If `right` list runs out of elements first, they're in wrong orders
/// 4) If both lists are equal length the order cannot be determined (`None`)
fn check_order(left: &[Element], right: &[Element]) -> Option<bool> {
    for i in 0..left.len() {
        let l = left.get(i).unwrap();
        if let Some(r) = right.get(i) {
            // If both elements are numbers, they're in right order if left is smaller
            if l.is_number() && r.is_number() {
                match l.cmp(r) {
                    // If Left is smaller (right order)
                    std::cmp::Ordering::Less => return Some(true),
                    // If Right is smaller (wrong order)
                    std::cmp::Ordering::Greater => return Some(false),
                    // If equal, continue
                    _ => (),
                }
            } else {
                // Call recursively to check if the lists are in right order
                let result = check_order(&l.convert_to_list(), &r.convert_to_list());
                if result.is_some() {
                    return result;
                }
            }
        } else {
            // Right ran out of elements (wrong order)
            return Some(false);
        }
    }
    if left.len() < right.len() {
        // Left ran out of elements (right order)
        return Some(true);
    }
    None
}

// Main function
fn main() {
    let element_pairs = read_input_as_element_pairs("input/day13.txt");

    let mut num = 0;
    for (i, pair) in element_pairs.iter().enumerate() {
        println!("{:?}", pair.0);
        println!("{:?}", pair.1);

        if let Some(result) = check_order(&pair.0, &pair.1) {
            if result {
                num += i + 1;
            }
        } else {
            panic!("Invalid input");
        }
        println!();
    }
    println!("Num of right orders: {}", num);
}

/// Read the input file
fn read_input_as_element_pairs(filename: &str) -> Vec<(Vec<Element>, Vec<Element>)> {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    let mut element_pairs = vec![];
    let mut left = vec![];
    let mut right = vec![];

    let mut left_read = false;
    for line in reader.lines() {
        let line = line.unwrap();

        // Empty line works as a separator for element pairs
        if line.is_empty() {
            element_pairs.push((left.clone(), right.clone()));
            left = vec![];
            right = vec![];
            left_read = false;
            continue;
        }

        if !left_read {
            left = recursive_parse_elements(&line[1..], &mut 0);
            left_read = true;
        } else {
            right = recursive_parse_elements(&line[1..], &mut 0);
        }
    }
    element_pairs.push((left, right));
    element_pairs
}

/// Recursive function for parsing recursive list of elements from a string
fn recursive_parse_elements(s: &str, chars_read: &mut usize) -> Vec<Element> {
    let mut elements = vec![];
    let mut value = String::new();

    let mut i = 0;
    while let Some(c) = s.chars().nth(i) {
        match c {
            '0'..='9' => value.push(c),
            ',' => {
                if !value.is_empty() {
                    elements.push(Element::Number(value.parse().unwrap()));
                    value.clear();
                }
            }
            '[' => {
                let mut chars_read = 0;
                elements.push(Element::List(recursive_parse_elements(
                    &s[i + 1..],
                    &mut chars_read,
                )));
                i += chars_read;
            }
            ']' => {
                if !value.is_empty() {
                    elements.push(Element::Number(value.parse().unwrap()));
                    value.clear();
                }
                *chars_read += i + 1;
                return elements;
            }
            _ => (),
        }
        i += 1;
    }
    panic!("unterminated list");
}
