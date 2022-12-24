use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod hands;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// X = rock = A
// Y = paper = B
// Z = Scissors = C
// 
fn main() {
    let mut self_points = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {

                let self_hand: hands::Hand = match line.chars().nth(2).unwrap() as u32 {
                    88 => hands::Hand::Rock,
                    89 => hands::Hand::Paper,
                    90 => hands::Hand::Scissors,
                    _ => panic!("invalid line")
                };
                let other_hand: hands::Hand = match line.chars().nth(0).unwrap() as u32 {
                    65 => hands::Hand::Rock,
                    66 => hands::Hand::Paper,
                    67 => hands::Hand::Scissors,
                    _ => panic!("invalid line")
                };
                let outcome = hands::play_hand(self_hand, other_hand);
                self_points += match outcome {
                    Win => 6,
                    Lose => 0,
                    Draw => 3   
                };
                self_points += match self_hand {
                    hands::Hand::Rock => 1,
                    hands::Hand::Paper => 2,
                    hands::Hand::Scissors => 3
                }
            }
        }
    }
    println!("{}", self_points);
}
