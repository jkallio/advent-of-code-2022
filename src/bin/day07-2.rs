use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Default)]
struct FileNode {
    is_file: bool,
    name: String,
    size: u32,
    parent: Option<FileNodePtr>,
    children: Vec<FileNodePtr>,
}

/// Type definition for smart pointer
type FileNodePtr = Rc<RefCell<FileNode>>;

/// Recursive function for printing the entire node hierarchy
fn print_node_tree(node: FileNodePtr, tabs: u32) {
    for _ in 0..tabs {
        print!("  ");
    }
    if node.borrow().is_file {
        println!(
            "- {} (file, size={})",
            node.borrow().name,
            node.borrow().size
        );
    } else {
        println!("- {} (dir)", node.borrow().name);
    }
    for child_node in node.borrow().children.iter() {
        print_node_tree(child_node.clone(), tabs + 1);
    }
}

/// Recursive function for calculating total size of the node and its children
fn calculate_node_size(node: FileNodePtr) -> u32 {
    let mut size = node.borrow().size;
    for child_node in node.borrow().children.iter() {
        size += calculate_node_size(child_node.clone());
    }
    size
}

/// Recursive function for finding folders larger than limit
fn sum_of_small_folders(node: FileNodePtr, limit: u32, smallest: &mut Vec<(String, u32)>) -> u32 {
    let mut size = node.borrow().size;
    for child_node in node.borrow().children.iter() {
        size += sum_of_small_folders(child_node.clone(), limit, &mut *smallest);
    }
    if !node.borrow().is_file && size >= limit {
        smallest.push((node.borrow().name.to_string(), size));
    }
    size
}

/// Main Function
fn main() {
    if let Ok(file_tree) = parse_input_file("input/day7.txt") {
        print_node_tree(file_tree.clone(), 0);
        calculate_node_size(file_tree.clone());

        let total_size = calculate_node_size(file_tree.clone());
        println!("\r\nTotal size of file hierarchy = {}", total_size);

        let size_required = 30000000 - (70000000 - total_size);
        println!("Size required to free = {}", size_required);

        let mut smallest = vec![];
        sum_of_small_folders(file_tree, size_required, &mut smallest);

        let mut m = 0;
        println!("\r\nLargest folders:");
        for s in smallest {
            if m == 0 || s.1 < m {
                m = s.1;
            }
            println!("  - {}: {}", s.0, s.1);
        }
        println!("Smallest one is => {}", m);
    }
}

/// Parse input file and construct the file node hierarchy
fn parse_input_file(file_path: &str) -> std::io::Result<FileNodePtr> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let root_node = Rc::new(RefCell::new(FileNode::default()));
    let mut current_node = root_node.clone();

    for line in reader.lines() {
        let line = line?;

        // Commands start with '$'
        if line.starts_with('$') {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // Commands are either "cd" or "ls"
            if parts[1] == "cd" {
                let name = parts[2].to_string();

                // Handle "cd" to root directory
                if name.contains('/') {
                    root_node.borrow_mut().name = name;
                    current_node = root_node.clone();
                // Handle "cd" to parent directory
                } else if name == ".." {
                    if let Some(parent) = &current_node.clone().borrow().parent {
                        current_node = parent.clone();
                    }
                // Handle "cd" to one of the child directories
                } else {
                    for child_node in current_node.clone().borrow().children.iter() {
                        if child_node.borrow().name == name {
                            current_node = child_node.clone();
                        }
                    }
                }
            }
        // Other lines must be `ls` output lines
        // Directories start with "dir" keyword
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[1].to_string();

            let node = if line.starts_with("dir") {
                Rc::new(RefCell::new(FileNode {
                    is_file: false,
                    name,
                    parent: Some(current_node.clone()),
                    ..Default::default()
                }))
            } else {
                Rc::new(RefCell::new(FileNode {
                    is_file: true,
                    name,
                    parent: Some(current_node.clone()),
                    size: parts[0].parse::<u32>().unwrap(),
                    ..Default::default()
                }))
            };

            current_node.borrow_mut().children.push(node);
        }
    }

    Ok(root_node)
}
