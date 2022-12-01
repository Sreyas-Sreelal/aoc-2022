fn main() {
    let output: i64 = include_str!("../input")
        .split("\r\n\r\n")
        .map(|x| x.split("\r\n").map(|y| y.parse::<i64>().unwrap()).sum())
        .max()
        .unwrap();
    println!("{}", output);
}
