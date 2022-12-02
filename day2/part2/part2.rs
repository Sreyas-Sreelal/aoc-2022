fn main() {
    let mut output = 0;
    include_str!("../input")
    .split("\r\n")
    .for_each(|x| {
        output += match x {
            "B X" => 1,
            "C X" => 2,
            "A X" => 3,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "C Z" => 7,
            "A Z" => 8,
            "B Z" => 9,
            _=> 0,
            
        }   
    });
    println!("{}",output);
}