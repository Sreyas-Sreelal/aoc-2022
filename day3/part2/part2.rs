use std::collections::{HashSet,HashMap};

fn main() {
    let hashes:HashMap<char,i64> = ('A'..='z').map(|x| (x,x as i64 - if x< 'a' {38} else {96})).collect();
    let mut sum = 0;
    include_str!("../input").split("\r\n").collect::<Vec<&'static str>>().chunks(3).for_each(|x| {
        let first:HashSet<char> = x[0].chars().collect();
        let second:HashSet<char> = x[1].chars().collect();
        let third:HashSet<char> = x[2].chars().collect();
        first.intersection(&(second.intersection(&third).copied().collect()))
        .for_each(|x| sum+=hashes[x]);
    });
    println!("{}",sum);
}