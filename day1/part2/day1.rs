fn main() {
    let mut output:Vec<i64>  = include_str!("../input")
        .split("\r\n\r\n")
        .map(|x| x.split("\r\n").map(|y| y.parse::<i64>().unwrap()).sum()).collect::<Vec<i64>>();
    output.sort();
    output.reverse();
    println!("{}", &output[0..3].iter().sum::<i64>());
}
