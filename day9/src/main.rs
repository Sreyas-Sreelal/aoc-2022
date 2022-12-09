use std::collections::BTreeSet;

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    f64::sqrt(((x1 - x2) as f64).powf(2.0) + ((y1 - y2) as f64).powf(2.0)) as i32
}
enum Slope {
    DIAGONAL,
    HORIZONTAL,
    VERTICAL,
}
fn check_slope(h_x: &i32, h_y: &i32, t_x: &i32, t_y: &i32) -> Slope {
    if h_x != t_x && h_y != t_y {
        Slope::DIAGONAL
    } else if h_y == t_y {
        Slope::HORIZONTAL
    } else {
        Slope::VERTICAL
    }
}
fn compute_tail_pos(rope:&mut Vec<(i32,i32)>,idx:usize) {
    match check_slope(&rope[idx-1].0, &rope[idx-1].1, &rope[idx].0, &rope[idx].1) {
        Slope::DIAGONAL => {
            if rope[idx-1].1 > rope[idx].1 {
                rope[idx].1 += 1;
            } else {
                rope[idx].1 -= 1;
            }
            if rope[idx-1].0 > rope[idx].0 {
                rope[idx].0 += 1;
            } else {
                rope[idx].0 -= 1;
            }
        }
        Slope::HORIZONTAL => {
            if rope[idx-1].0 > rope[idx].0 {
                rope[idx].0 += 1;
            } else {
                rope[idx].0 -= 1;
            }
        }
        Slope::VERTICAL => {
            if rope[idx-1].1 > rope[idx].1 {
                rope[idx].1 += 1;
            } else {
                rope[idx].1 -= 1;
            }
        }
    }
}
fn main() {
    let input = include_str!("../input");
    let mut rope = Vec::new();
    let mut knot_locations = Vec::new();
    for i in 0..10 {
        rope.push((0,0));
        knot_locations.push(BTreeSet::new());
        knot_locations[i].insert((0,0));
    }

    for line in input.lines() {
        let mut line_split = line.split(" ");
        let code = line_split.next().unwrap();
        let number: i32 = line_split.next().unwrap().parse().unwrap();

        match code {
            "R" => {
                
                    for _ in 0..number {
                        rope[0].0 += 1;
                        knot_locations[0].insert((rope[0].0, rope[0].1));
                        for idx in 1..10 {
                            if distance(rope[idx-1].0, rope[idx-1].1, rope[idx].0, rope[idx].1) == 2 {
                                compute_tail_pos(&mut rope,idx);
                                knot_locations[idx].insert((rope[idx].0, rope[idx].1));
                            }
                        }
                        
                    }
                
            }
            "U" => {
                for _ in 0..number {
                    rope[0].1 += 1;
                    knot_locations[0].insert((rope[0].0, rope[0].1));
                    for idx in 1..10 {
                        if distance(rope[idx-1].0, rope[idx-1].1, rope[idx].0, rope[idx].1) == 2 {
                            compute_tail_pos(&mut rope,idx);
                            knot_locations[idx].insert((rope[idx].0, rope[idx].1));
                        }
                    }
                    
                }
            }
            "L" => {
                for _ in 0..number {
                    rope[0].0 -= 1;
                    knot_locations[0].insert((rope[0].0, rope[0].1));
                    for idx in 1..10 {
                        if distance(rope[idx-1].0, rope[idx-1].1, rope[idx].0, rope[idx].1) == 2 {
                            compute_tail_pos(&mut rope,idx);
                            knot_locations[idx].insert((rope[idx].0, rope[idx].1));
                        }
                    }
                    
                }
            }
            "D" => {
                for _ in 0..number {
                    rope[0].1 -= 1;
                    knot_locations[0].insert((rope[0].0, rope[0].1));
                    for idx in 1..10 {
                        if distance(rope[idx-1].0, rope[idx-1].1, rope[idx].0, rope[idx].1) == 2 {
                            compute_tail_pos(&mut rope,idx);
                            knot_locations[idx].insert((rope[idx].0, rope[idx].1));
                        }
                    }
                    
                }
            }
            _ => panic!("Invalid Code!!"),
        }
    }
    println!("part 1: {}", knot_locations[1].len());
    println!("part 2: {}", knot_locations[9].len());
}
