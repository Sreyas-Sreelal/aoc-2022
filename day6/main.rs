fn find_start_of_message_marker(input:&'static str,sequence_window:usize) -> usize{
    let length = input.len();
    for x in 0..length-sequence_window {
        let mut word = (&input[x..x+sequence_window]).chars().collect::<Vec<char>>();
        word.sort();
        word.dedup();
        if word.len() == sequence_window {
            return x+sequence_window;
        }
    }
    return 0;
}

fn main() {
    println!("part1 : {}",find_start_of_message_marker(include_str!("input"),4));
    println!("part2 : {}",find_start_of_message_marker(include_str!("input"),14));
}