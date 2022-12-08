use std::collections::BTreeSet;
type HeightMap = Vec<Vec<(i32, u32)>>;

fn look(heightmap: &HeightMap, viewable_tree: &mut BTreeSet<i32>) {
    for row in heightmap {
        let mut max_height = -1;
        for (id, height) in row {
            if *height as i32 > max_height {
                max_height = *height as i32;
                viewable_tree.insert(*id);
            }
        }
    }
}
fn scenic_score(heightmap: &HeightMap) -> i32 {
    let mut best = 0;
    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let mut score = 1;
            let (_, height) = heightmap[i][j];
            let mut count;
            //up
            let mut up = i as i32 - 1;
            count = 0;
            while up > -1 {
                let (_, scanned_height) = heightmap[up as usize][j];
                if scanned_height < height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
                up -= 1;
            }
            score *= count;

            //left
            let mut left = j as i32 - 1;
            count = 0;
            while left > -1 {
                let (_, scanned_height) = heightmap[i][left as usize];
                if scanned_height < height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
                left -= 1;
            }
            score *= count;

            //right
            let mut right = j + 1;
            count = 0;
            while right < heightmap[i].len() {
                let (_, scanned_height) = heightmap[i][right];
                if scanned_height < height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
                right += 1;
            }
            score *= count;

            //down
            let mut down = i + 1;
            count = 0;
            while down < heightmap[i].len() {
                let (_, scanned_height) = heightmap[down][j];
                if scanned_height < height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
                down += 1;
            }
            score *= count;
            if best < score {
                best = score;
            }
        }
    }
    best
}
fn main() {
    let input = include_str!("../input");
    let mut viewable_trees: BTreeSet<i32> = BTreeSet::new();
    let mut view: HeightMap = Vec::new();
    let mut idx = -1;
    let mut len = 0;
    for line in input.lines() {
        view.push(
            line.chars()
                .map(|x| {
                    idx += 1;
                    (idx, x.to_digit(10).unwrap())
                })
                .collect::<Vec<(i32, u32)>>(),
        );
    }
    println!("scenic score {}", scenic_score(&view));
    look(&view, &mut viewable_trees);

    for x in &mut view {
        x.reverse();
        len = x.len();
    }
    look(&view, &mut viewable_trees);
    let mut iters: Vec<_> = view.clone().into_iter().map(|x| x.into_iter()).collect();

    view = (0..len)
        .map(|_| iters.iter_mut().map(|x| x.next().unwrap()).collect())
        .collect();

    look(&view, &mut viewable_trees);
    for x in &mut view {
        x.reverse();
    }
    look(&view, &mut viewable_trees);
    println!("viewable trees: {}", viewable_trees.len());
}
