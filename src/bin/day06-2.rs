use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

fn main() {
    if let Ok(mut file) = File::open("input/day6.txt") {
        let mut bytes = vec![];
        if file.read_to_end(&mut bytes).is_ok() {
            let mut byte_count = 0;
            let mut win = VecDeque::<u8>::new();
            for b in bytes {
                win.push_back(b);
                if win.len() > 14 {
                    win.pop_front();
                    byte_count += 1;
                }
                if win.len() == 14 && has_unique_elements(&win) {
                    break;
                }
            }

            println!("Byte count = {}", byte_count + 14);
        }
    }
}
