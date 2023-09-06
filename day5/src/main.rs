use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut curr_line = 0;
    let mut stacks: [VecDeque<char>; 9] = Default::default();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if curr_line < 8 {
                    // build stacks

                    line.chars().enumerate().for_each(|(col, character)|
                        if character.is_alphabetic() {
                            stacks[(((col as i32)-1) / 4) as usize].push_front(character)
                        }
                    )

                }
                if curr_line > 9 {
                    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
                    for (_, [number, source, dest]) in re.captures_iter(&line).map(|c| c.extract()) {
                        for _ in  0..number.parse().unwrap() {
                            let moving = stacks[source.parse::<i32>().unwrap() as usize -1].pop_back().unwrap();
                            stacks[dest.parse::<i32>().unwrap() as usize -1].push_back(moving);
                        }
                    }
                }
                curr_line += 1;
            }
        }
        let mut end_str = String::new();
        for mut i in stacks {
            end_str.push(i.pop_back().unwrap());
        }
        println!("{}", end_str);
    }
    let mut curr_line = 0;
    let mut stacks: [VecDeque<char>; 9] = Default::default();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if curr_line < 8 {
                    // build stacks

                    line.chars().enumerate().for_each(|(col, character)|
                        if character.is_alphabetic() {
                            stacks[(((col as i32)-1) / 4) as usize].push_front(character)
                        }
                    )

                }
                if curr_line > 9 {
                    let mut stack_to_move: VecDeque<char> = Default::default();
                    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
                    for (_, [number, source, dest]) in re.captures_iter(&line).map(|c| c.extract()) {
                        let num = number.parse().unwrap();
                        let src_stack = source.parse::<i32>().unwrap() as usize-1;
                        let dest_stack = dest.parse::<i32>().unwrap() as usize-1;
                        for _ in  0..num {
                            stack_to_move.push_front(stacks[src_stack].pop_back().unwrap()); // push to front so we reverse the order
                        }
                        for _ in 0..num {
                            stacks[dest_stack].push_back(stack_to_move.pop_front().unwrap())
                        }
                    }
                }
                curr_line += 1;
            }
        }
        let mut end_str = String::new();
        for mut i in stacks {
            end_str.push(i.pop_back().unwrap());
        }
        println!("Part 2: {}", end_str);
    }
}
