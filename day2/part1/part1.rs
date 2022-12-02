fn main() {
    let mut output = 0;
    include_str!("../input")
    .split("\r\n")
    .for_each(|x| {
        let opponent =  x.chars().nth(0).unwrap() as i64;
        let player =  x.chars().nth(2).unwrap() as i64;
        let residue = ((player - opponent) % 3) as f64;
        output+=(4.5* residue * residue - 10.5 * residue) as i64 + 6 + (player - 87);   
    });
    println!("{}",output);
}