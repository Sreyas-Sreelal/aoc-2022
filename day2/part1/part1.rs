fn main() {
    let mut output = 0;
    include_str!("../input")
    .split("\r\n")
    .for_each(|x| {
        let opponent =  x.chars().nth(0).unwrap() ;
        let player =  x.chars().nth(2).unwrap();
        let residue = ((player as i64 - opponent as i64) % 3) as f64;
        output+=(4.5* residue * residue - 10.5 * residue) as i64 + 6 + (player as i64 - 87);   
    });
    println!("{}",output);
}