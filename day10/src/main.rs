use regex::Regex;
#[derive(Debug)]
enum Commands {
    Addx(i32),
    Noop,
}
fn execute(cmd: &Commands, register: &mut i32) {
    match cmd {
        Commands::Addx(operand) => {
            *register += operand;
        }
        Commands::Noop => {}
    }
}
fn main() {
    let input = include_str!("../input");
    let re = Regex::new(r"(?P<command>\w+)(:?\s+)?(?P<operand>-?\d+)?").unwrap();
    let mut tokens: Vec<Commands> = Vec::new();
    let mut screen: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut row = 0;
    let mut column = 0;

    for lines in input.lines() {
        let capture = re.captures(lines).expect("invalid command");
        let command = &capture["command"];
        match command {
            "addx" => tokens.push(Commands::Addx(
                capture["operand"]
                    .parse()
                    .expect("invalid operand for Addx"),
            )),
            "noop" => tokens.push(Commands::Noop),
            _ => panic!("Invalid command {}", command),
        }
    }
    let mut register_x: i32 = 1;
    let mut cycle = 1;
    let mut iter = tokens.iter();
    let mut waiting = None;
    let mut output = 0;
    let needed_cycles = [20, 60, 100, 140, 180, 220];
    loop {
        if needed_cycles.contains(&cycle) {
            output += cycle * register_x;
        }

        if column == 40 {
            column = 0;
            row += 1;
        }

        if column == (register_x - 1) as usize
            || column == register_x as usize
            || column == (register_x + 1) as usize
        {
            screen[row][column] = '#';
        }
        if waiting.is_none() {
            let command = iter.next();
            if command.is_none() {
                break;
            }
            let command = command.unwrap();
            match command {
                Commands::Addx(_) => {
                    waiting = Some((command, 2));
                }
                Commands::Noop => {
                    waiting = Some((command, 1));
                }
            }
        }
        let mut temp = waiting.unwrap();
        temp.1 -= 1;
        if temp.1 == 0 {
            execute(temp.0, &mut register_x);
            waiting = None;
        } else {
            waiting = Some(temp);
        }
        cycle += 1;
        column += 1;
    }

    println!("part 1 {:?}", output);
    for row in screen.iter() {
        for column in row.iter() {
            print!("{}", column);
        }
        println!();
    }
}
