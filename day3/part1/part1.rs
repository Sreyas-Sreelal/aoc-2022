use std::collections::{HashSet,HashMap};

fn main() {
    let hashes:HashMap<char,i64> = ('A'..='z').map(|x| (x,x as i64 - if x< 'a' {38} else {96})).collect();
    let mut sum = 0;
    include_str!("../input").split("\r\n").for_each(|x| {
        let first:HashSet<char> = x[0..x.len()/2].chars().collect();
        let second:HashSet<char> = x[x.len()/2..x.len()].chars().collect();
        first.intersection(&second).for_each(|x| sum+=hashes[x]);
    });
    println!("{}",sum);
}