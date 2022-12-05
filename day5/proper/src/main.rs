use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

type StackTable = BTreeMap<i32, Vec<char>>;

enum Instruction {
    Move(i32, i32, i32),
}

struct CargoCrane {
    stack_table: StackTable,
    instructions: Vec<Instruction>,
}

fn parse_stacks(data: &'static str) -> Option<StackTable> {
    let mut chars = data.chars();
    let mut stack_table: StackTable = BTreeMap::new();
    let mut stack_identifier = -1;
    loop {
        stack_identifier += 1;
        match chars.next() {
            Some('[') => {
                if let Some(cargo) = chars.next() {
                    stack_identifier += 1;
                    let identifier = stack_identifier / 4 + 1;
                    stack_table.entry(identifier).or_insert_with(Vec::new);
                    let stack = stack_table.get_mut(&identifier).unwrap();
                    stack.insert(0, cargo);
                    if let Some(c) = chars.next() {
                        if c != ']' {
                            return None;
                        }
                        stack_identifier += 1;
                    }
                } else {
                    return None;
                }
            }
            Some(c) => {
                if c.is_ascii_digit() {
                    break;
                }
                if c == '\n' || c == '\r' {
                    stack_identifier = -1;
                }
                continue;
            }
            _ => return None,
        }
    }
    Some(stack_table)
}

fn parse_instructions(input: &'static str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let re =
        Regex::new(r"move\s+(?P<count>\d+)\s+from\s+(?P<from>\d+)\s+to\s+(?P<to>\d+)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).expect("Invalid Instruction format!");
        let count: i32 = captures["count"]
            .parse()
            .expect("count must be an integer!");
        let from: i32 = captures["from"]
            .parse()
            .expect("from stack must be an integer!");
        let to: i32 = captures["to"]
            .parse()
            .expect("to stack must be an integer!");
        instructions.push(Instruction::Move(count, from, to));
    }
    instructions
}
impl CargoCrane {
    fn new(tokens: &'static str) -> Self {
        let mut token_iter = tokens.split("\r\n\r\n");
        let data = token_iter.next().unwrap();
        let instructions = token_iter.next().unwrap();
        if let Some(table) = parse_stacks(data) {
            CargoCrane {
                stack_table: table,
                instructions: parse_instructions(instructions),
            }
        } else {
            panic!("Invalid syntax for data");
        }
    }
    fn execute(&mut self, persist_order: bool) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Move(count, from, to) => {
                    let mut lifted_cargos = VecDeque::new();
                    let source = self.stack_table.get_mut(from).unwrap();

                    for _ in 0..*count {
                        if let Some(cargo) = source.pop() {
                            lifted_cargos.push_back(cargo);
                        }
                    }
                    let destination = self.stack_table.get_mut(to).unwrap();
                    if persist_order {
                        while let Some(cargo) = lifted_cargos.pop_back() {
                            destination.push(cargo);
                        }
                    } else {
                        while let Some(cargo) = lifted_cargos.pop_front() {
                            destination.push(cargo);
                        }
                    }
                }
            }
        }
    }
    fn print_tops(&self) {
        for stack in self.stack_table.values() {
            if let Some(last) = stack.last() {
                print!("{}", last);
            }
        }
    }
}
fn main() {
    let input = include_str!("../input");
    let mut cg = CargoCrane::new(input);
    cg.execute(true);
    cg.print_tops();
}
