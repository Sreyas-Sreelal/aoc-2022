use std::collections::HashSet;

fn main() {
    let mut count = 0;
    let mut count2 = 0;
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    include_str!("input").split("\r\n").for_each(|x| {
        let captures =  re.captures(x).unwrap();
        let first:i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let second:i64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let third:i64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let fourth:i64 = captures.get(4).unwrap().as_str().parse().unwrap();
        let set_one:HashSet<i64> = (first..=second).collect();
        let set_two:HashSet<i64> = (third..=fourth).collect();
        if set_one.is_subset(&set_two) || set_two.is_subset(&set_one) {
            count += 1;
        }
        if !set_one.is_disjoint(&set_two) {
            count2+=1;
        }
    });
    println!("part 1: {} part2 : {}",count,count2);
}