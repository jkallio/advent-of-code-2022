use core::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(PartialEq)]
enum Operation {
    Add,
    Multiply,
    Power,
}

struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    throw_to: (u32, u32),
    test_num: u32,
    operation: Operation,
    operation_num: u32,
    inspections: u32,
}

type MonkeyPtr = Rc<RefCell<Monkey>>;
impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: VecDeque::new(),
            throw_to: (0, 0),
            test_num: 0,
            operation: Operation::Add,
            operation_num: 0,
            inspections: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let monkeys = read_lines_from_file("input/day11.txt")?;
    for round in 1..=20 {
        println!("\r\nRound #{}", round);

        for id in 0..monkeys.len() {
            let mut monkey = monkeys.get(&(id as u32)).unwrap().borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspections += 1;
                let mut worry_level = match monkey.operation {
                    Operation::Add => item + monkey.operation_num,
                    Operation::Multiply => item * monkey.operation_num,
                    Operation::Power => item.pow(monkey.operation_num),
                };

                worry_level = ((worry_level as f32 / 3.0).floor()) as u32;
                let throw_id = if worry_level % monkey.test_num == 0 {
                    monkey.throw_to.0
                } else {
                    monkey.throw_to.1
                };
                let mut target = monkeys.get(&(throw_id as u32)).unwrap().borrow_mut();
                target.items.push_back(worry_level);
            }
        }

        let mut inspections = vec![];
        for id in 0..monkeys.len() {
            let monkey = monkeys.get(&(id as u32)).unwrap().borrow();
            print!("Monkey {}: ", monkey.id);
            for item in &monkey.items {
                print!("{}, ", item);
            }
            println!();
            inspections.push(monkey.inspections);
        }

        if round == 20 {
            inspections.sort_by(|a, b| b.cmp(a));
            let monkey_business = inspections[0] * inspections[1];
            println!("\r\nMonkey Business = {}", monkey_business);
        }
    }
    Ok(())
}

fn read_lines_from_file(file_path: &str) -> std::io::Result<HashMap<u32, MonkeyPtr>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut monkeys = HashMap::<u32, MonkeyPtr>::new();

    let mut monkey = Rc::new(RefCell::new(Monkey::new()));
    for it in reader.lines() {
        let line = it?;
        println!("{}", line);

        // Parse Monkey ID
        if line.contains("Monkey") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut monkey_id: String = parts[1].to_string();
            let colon = monkey_id.pop();
            assert!(colon == Some(':'));
            monkey.borrow_mut().id = monkey_id.parse::<u32>().unwrap();
        // Parse Item list
        } else if line.contains("Starting items") {
            let parts: Vec<&str> = line.split(':').collect();
            let mut items: Vec<&str> = parts[1].split(',').collect();
            for item in items.iter_mut() {
                monkey
                    .borrow_mut()
                    .items
                    .push_back(item.trim().parse::<u32>().unwrap());
            }
        // Parse mathematical operation and the operation number
        } else if line.contains("Operation") {
            if line.contains("old * old") {
                monkey.borrow_mut().operation = Operation::Power;
                monkey.borrow_mut().operation_num = 2;
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();

                monkey.borrow_mut().operation_num = parts.last().unwrap().parse::<u32>().unwrap();
                monkey.borrow_mut().operation = if line.contains('+') {
                    Operation::Add
                } else if line.contains('*') {
                    Operation::Multiply
                } else {
                    panic!("Invalid operation");
                };
            }
        // Parse Test logic (divisible by what number)
        } else if line.contains("Test") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            monkey.borrow_mut().test_num = parts.last().unwrap().parse::<u32>().unwrap();
        // Parse `true` path
        } else if line.contains("true") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            monkey.borrow_mut().throw_to.0 = parts.last().unwrap().parse::<u32>().unwrap();
        // Parse `false` path
        } else if line.contains("false") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            monkey.borrow_mut().throw_to.1 = parts.last().unwrap().parse::<u32>().unwrap();
        }

        if line.is_empty() {
            monkeys
                .entry(monkey.borrow().id)
                .or_insert_with(|| monkey.clone());
            monkey = Rc::new(RefCell::new(Monkey::new()));
        }
    }
    monkeys
        .entry(monkey.borrow().id)
        .or_insert_with(|| monkey.clone());
    Ok(monkeys)
}
