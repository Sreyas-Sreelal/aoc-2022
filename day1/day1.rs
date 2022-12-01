fn main() {
    let output: i32 = include_str!("input")
        .split("\r\n\r\n")
        .map(|x| x.split("\r\n").map(|y| y.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();
    println!("{}", output);
}
