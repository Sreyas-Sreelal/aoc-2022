use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

type FileTree = BTreeMap<String, BTreeSet<String>>;
lazy_static! {
    pub static ref FILE: Regex = Regex::new(r"(?P<size>\d+) (?P<name>.*)").unwrap();
    pub static ref DIR: Regex = Regex::new(r"dir (?P<directory>.*)").unwrap();
}
fn compute_dir_size(
    dir_size: &BTreeMap<String, usize>,
    dirs: &FileTree,
    directory_name: String,
) -> usize {
    let mut size = 0;
    let dir = &dirs[&directory_name];

    for content in dir {
        if let Some(file) = FILE.captures(content) {
            size += file["size"].parse::<usize>().unwrap();
        } else if let Some(directory) = DIR.captures(content) {
            if let Some(dsize) = dir_size.get(&directory["directory"].to_string()) {
                size += dsize;
            } else {
                size += compute_dir_size(dir_size, dirs, directory["directory"].to_string());
            }
        } else {
            continue;
        }
    }
    size
}
fn main() {
    let mut dirs: FileTree = BTreeMap::new();
    let mut cwd: Vec<String> = Vec::new();
    cwd.push("".to_string());
    let input = include_str!("../input");
    let instruction = Regex::new(r"\$\s+(?P<command>\w+)(:?\s+)?(?P<arg>.*)?").unwrap();
    let mut lines = input.lines();
    let mut line = lines.next();

    loop {
        if line.is_none() {
            break;
        }

        if let Some(capture) = instruction.captures(line.unwrap()) {
            let command = &capture["command"];
            match command {
                "cd" => {
                    dirs.entry(cwd.join("/") + "/").or_insert(BTreeSet::new());

                    let dir = dirs.get_mut(&(cwd.join("/") + "/").to_owned()).unwrap();
                    let destination = &capture["arg"];

                    if destination == ".." {
                        cwd.pop();
                    } else if destination == "/" {
                        cwd = vec!["".to_string()];
                    } else {
                        dir.insert("dir ".to_owned() + &cwd.join("/") + "/" + destination + "/");
                        cwd.push(capture["arg"].to_string());
                        dirs.entry(cwd.join("/") + "/").or_insert(BTreeSet::new());
                    }

                    line = lines.next();
                    continue;
                }
                "ls" => {
                    let dir = dirs.get_mut(&(cwd.join("/") + "/")).unwrap();
                    loop {
                        line = lines.next();
                        if line.is_none() {
                            break;
                        }
                        if line.unwrap().starts_with('$') {
                            break;
                        }
                        if let Some(directory) = DIR.captures(line.unwrap()) {
                            dir.insert(
                                "dir ".to_owned()
                                    + &cwd.join("/")
                                    + "/"
                                    + &directory["directory"]
                                    + "/",
                            );
                        } else {
                            dir.insert(String::from(line.unwrap()));
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    let mut output = 0;
    let mut dir_sizes: BTreeMap<String, usize> = BTreeMap::new();

    for x in dirs.keys() {
        let size = compute_dir_size(&dir_sizes, &dirs, x.to_string());
        dir_sizes.insert(
            x.to_string(),
            size,
        );
        
        if size <= 100000 {
            output += size;
        }
    }
    println!("part 1 {}", output);

    let total_filesize = 70000000;
    let update_size = 30000000;
    let current_size = dir_sizes["/"];
    let free_space = total_filesize - current_size;
    let required_size = update_size - free_space;
    let mut candidates: Vec<usize> = dir_sizes
        .values()
        .filter(|x| **x >= required_size)
        .copied()
        .collect();
    candidates.sort();
    println!("part2 : {}", candidates.first().unwrap());
}
