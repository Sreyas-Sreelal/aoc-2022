#[derive(Debug, Clone, Copy)]
enum OperandType {
    Constant(i64),
    Old,
}
#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiplication(OperandType, OperandType),
    Addition(OperandType, OperandType),
    Unkonwn,
}
#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<i64>,
    operation: Operation,
    // divisble,true,false destinations
    test: (i64, i64, i64),
    inspect_count: i64,
}

fn get_operand_type(data: &str) -> OperandType {
    match data {
        "old" => OperandType::Old,
        _ => {
            if let Ok(constant) = data.parse() {
                OperandType::Constant(constant)
            } else {
                panic!("Invalid operand passed!");
            }
        }
    }
}

fn get_operation(operator: &str, left: OperandType, right: OperandType) -> Operation {
    match operator {
        "*" => Operation::Multiplication(left, right),
        "+" => Operation::Addition(left, right),
        _ => panic!("Invalid operation passed!"),
    }
}
fn do_operation(operation: Operation, old: i64) -> i64 {
    match operation {
        Operation::Addition(OperandType::Old, OperandType::Constant(val))
        | Operation::Addition(OperandType::Constant(val), OperandType::Old) => old + val,
        Operation::Addition(OperandType::Old, OperandType::Old) => old + old,
        Operation::Multiplication(OperandType::Old, OperandType::Constant(val))
        | Operation::Multiplication(OperandType::Constant(val), OperandType::Old) => old * val,
        Operation::Multiplication(OperandType::Old, OperandType::Old) => old * old,
        _ => {
            panic!("Unknonwn Operation!!");
        }
    }
}
fn execute(monkeys: &mut Vec<Monkey>, offset: i64, divide_by_3: bool) {
    for x in 0..monkeys.len() {
        for old in 0..monkeys[x].starting_items.len() {
            monkeys[x].inspect_count += 1;
            let test = monkeys[x].test;
            monkeys[x].starting_items[old] =
                do_operation(monkeys[x].operation, monkeys[x].starting_items[old]);
            let mut new = monkeys[x].starting_items[old];
            if divide_by_3 {
                monkeys[x].starting_items[old] /= 3;
                new /= 3;
            } else {
                new %= offset;
            }

            if monkeys[x].starting_items[old] % test.0 == 0 {
                monkeys[test.1 as usize].starting_items.push(new);
            } else {
                monkeys[test.2 as usize].starting_items.push(new);
            }
        }
        monkeys[x].starting_items = Vec::new();
    }
}
fn main() {
    let starting_re = regex::Regex::new(r"Starting items: (?P<numberlist>.*)").unwrap();
    let operation_re =
        regex::Regex::new(r"Operation: new = (?P<left>.+) (?P<operator>.+) (?P<right>.+)").unwrap();
    let test_re = regex::Regex::new(r"(?P<digit>\d+)").unwrap();
    let input = include_str!("../input").split("\r\n\r\n");
    let mut monkeys = Vec::new();

    for monkey in input {
        let mut section_iter = monkey.lines();
        section_iter.next();
        let mut monkey_detail = Monkey {
            starting_items: Vec::new(),
            operation: Operation::Unkonwn,
            test: (0, 0, 0),
            inspect_count: 0,
        };

        if let Some(starting_items) = starting_re.captures(section_iter.next().unwrap()) {
            monkey_detail.starting_items = starting_items["numberlist"]
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
        }
        if let Some(operation) = operation_re.captures(section_iter.next().unwrap()) {
            let left = get_operand_type(&operation["left"]);
            let right = get_operand_type(&operation["right"]);
            let operator = &operation["operator"];
            monkey_detail.operation = get_operation(operator, left, right);
        }
        if let Some(test) = test_re.captures(section_iter.next().unwrap()) {
            let divisiblity: i64 = test["digit"].parse().unwrap();
            let true_stmt: i64 = test_re.captures(section_iter.next().unwrap()).unwrap()["digit"]
                .parse()
                .unwrap();
            let false_stmt: i64 = test_re.captures(section_iter.next().unwrap()).unwrap()["digit"]
                .parse()
                .unwrap();
            monkey_detail.test = (divisiblity, true_stmt, false_stmt);
        }

        monkeys.push(monkey_detail);
    }
    let mut offset = 1;
    for x in &monkeys {
        offset *= x.test.0;
    }

    let mut monkey_p1 = monkeys.clone();
    for _ in 0..20 {
        execute(&mut monkey_p1, offset, true);
    }

    monkey_p1.sort_by(|x, y| y.inspect_count.cmp(&x.inspect_count));
    println!(
        " part2: {:?}",
        monkey_p1[0].inspect_count * monkey_p1[1].inspect_count
    );

    for _ in 0..10000 {
        execute(&mut monkeys, offset, false);
    }
    monkeys.sort_by(|x, y| y.inspect_count.cmp(&x.inspect_count));
    println!(
        " part2: {:?}",
        monkeys[0].inspect_count * monkeys[1].inspect_count
    );
}
