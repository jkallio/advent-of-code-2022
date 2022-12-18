use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////
/// [`ElementList`]
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementList(Vec<Element>);

/// Order of two [`ElementList`]s is determined by following rules:
/// 1) If both elements are numbers, they're in right order if left is smaller
/// 2) If `left` list runs out of elements first, they're in right order
/// 3) If `right` list runs out of elements first, they're in wrong orders
/// 4) If both lists are equal length the order cannot be determined (`None`)
impl Ord for ElementList {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.0.len() {
            let l = self.0.get(i).unwrap();
            if let Some(r) = other.0.get(i) {
                // If both elements are numbers, they're in right order if left is smaller
                if l.is_number() && r.is_number() {
                    match l.cmp(r) {
                        // If Left is smaller (right order)
                        Ordering::Less => return Ordering::Less,
                        // If Right is smaller (wrong order)
                        Ordering::Greater => return Ordering::Greater,
                        // If equal, continue
                        _ => (),
                    }
                } else {
                    // Call recursively to check if the lists are in right order
                    let l_list = l.convert_to_list();
                    let r_list = r.convert_to_list();
                    let result = l_list.cmp(&r_list);
                    if result != Ordering::Equal {
                        return result;
                    }
                }
            } else {
                // Right ran out of elements (wrong order)
                return Ordering::Greater;
            }
        }
        if self.0.len() < other.0.len() {
            // Left ran out of elements (right order)
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl PartialOrd for ElementList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

////////////////////////////////////////////////////////////////////////////////
/// [`Element`] represents a single element in a list
////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
enum Element {
    Number(i32),
    List(ElementList),
}

impl Element {
    fn is_number(&self) -> bool {
        matches!(self, Element::Number(_))
    }

    fn convert_to_list(&self) -> ElementList {
        match self {
            Element::List(list) => list.clone(),
            Element::Number(number) => ElementList(vec![Element::Number(*number)]),
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
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Number(a), Element::Number(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::List(l) => {
                write!(f, "[")?;
                for (i, e) in l.0.iter().enumerate() {
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

////////////////////////////////////////////////////////////////////////////////
// Main function
////////////////////////////////////////////////////////////////////////////////
fn main() {
    let mut packets = read_input_file_as_packets("input/day13.txt");

    // Add divider packets [[2]] and [[6]]
    packets.push(ElementList(vec![Element::Number(2)]));
    packets.push(ElementList(vec![Element::Number(6)]));

    // Sort packets
    packets.sort();

    // Find the divider packet indices and calculate the decoder key
    let mut divider_index1 = 0;
    let mut divider_index2 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if packet.0.len() == 1 {
            if packet.0[0] == Element::Number(2) {
                divider_index1 = i + 1;
            } else if packet.0[0] == Element::Number(6) {
                divider_index2 = i + 1;
            }
        }
        println!("{:?}", packet);
    }
    println!("\r\nDecoder Key = {}", divider_index1 * divider_index2);
}

/// Read the input file
fn read_input_file_as_packets(filename: &str) -> Vec<ElementList> {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    let mut packets = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let packet = recursive_parse_elements(&line[1..], &mut 0);
            packets.push(packet);
        }
    }
    packets
}

/// Recursive function for parsing recursive list of elements from a string
fn recursive_parse_elements(s: &str, chars_read: &mut usize) -> ElementList {
    let mut elements = ElementList(vec![]);
    let mut value = String::new();

    let mut i = 0;
    while let Some(c) = s.chars().nth(i) {
        match c {
            '0'..='9' => value.push(c),
            ',' => {
                if !value.is_empty() {
                    elements.0.push(Element::Number(value.parse().unwrap()));
                    value.clear();
                }
            }
            '[' => {
                let mut chars_read = 0;
                elements.0.push(Element::List(recursive_parse_elements(
                    &s[i + 1..],
                    &mut chars_read,
                )));
                i += chars_read;
            }
            ']' => {
                if !value.is_empty() {
                    elements.0.push(Element::Number(value.parse().unwrap()));
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
