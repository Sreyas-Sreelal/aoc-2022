use std::str::Chars;
use std::collections::BTreeMap;

fn get_number(input: &mut Chars) -> i32 {
    input.next();
    let mut number: String = String::new();
    loop {
        let c = input.next();
        if c.is_some() {
            let c = c.unwrap();
            if c.is_digit(10) {
                number.push(c);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    input.next();
    return number.parse().unwrap();
}
fn main() {
    let mut stack_table: BTreeMap<i32, Vec<char>> = BTreeMap::new();
    let input = include_str!("input");
    let mut input = input.chars();
    let mut stack_idx = -1;
    loop {
        stack_idx += 1;
        match input.next() {
            Some('[') => {
                let idx = (stack_idx / 4) + 1;
                let letter = input.next().unwrap();
                input.next();
                stack_idx += 2;
                if !stack_table.contains_key(&idx) {
                    stack_table.insert(idx, Vec::new());
                }
                let stack = stack_table.get_mut(&idx).unwrap();
                stack.insert(0, letter);
            }
            Some('m') => {
                while let Some(c) = input.next() {
                    if c == 'e' {
                        break;
                    }
                }
                let count: i32 = get_number(&mut input);
                while let Some(c) = input.next() {
                    if c == 'm' {
                        break;
                    }
                }

                let from: i32 = get_number(&mut input);

                while let Some(c) = input.next() {
                    if c == 'o' {
                        break;
                    }
                }
                let to: i32 = get_number(&mut input);
                
                for _ in 0..count {
                    let from = stack_table.get_mut(&from).unwrap().pop();
                    let to = stack_table.get_mut(&to).unwrap();

                    if let Some(from) = from {
                        to.push(from);
                    }
                }
                
            }
            Some(c) => {
                if c == '\n' || c == '\r' {
                    stack_idx = -1;
                    continue;
                }
                if c.is_whitespace() || c.is_digit(10) {
                    continue;
                }
            }

            _ => break,
        }
    }
    println!("{:?}", stack_table);
    for (_, y) in stack_table {
        if let Some(character) = y.last() {
            print!("{}", character);
        }
    }
}
