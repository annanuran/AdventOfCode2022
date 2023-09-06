use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::hands::Beats;
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
    // part 1
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
                println!("{:?}", self_hand);
                let other_hand: hands::Hand = match line.chars().nth(0).unwrap() as u32 {
                    65 => hands::Hand::Rock,
                    66 => hands::Hand::Paper,
                    67 => hands::Hand::Scissors,
                    _ => panic!("invalid line")
                };
                let outcome = hands::play_hand(self_hand, other_hand);
                self_points += match outcome {
                    hands::HandResult::Win => 6,
                    hands::HandResult::Lose => 0,
                    hands::HandResult::Draw => 3   
                };
                self_points += match self_hand {
                    hands::Hand::Rock => 1,
                    hands::Hand::Paper => 2,
                    hands::Hand::Scissors => 3
                }
            }
        }
    }
    println!("Part 1: {}", self_points);
    self_points = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let other_hand: hands::Hand = match line.chars().nth(0).unwrap() as u32 {
                    65 => hands::Hand::Rock,
                    66 => hands::Hand::Paper,
                    67 => hands::Hand::Scissors,
                    _ => panic!("invalid line {}", line.chars().nth(0).unwrap() as u32)
                };
                let outcome: hands::HandResult = match line.chars().nth(2).unwrap() as u32 {
                    88 => hands::HandResult::Lose,
                    89 => hands::HandResult::Draw,
                    90 => hands::HandResult::Win,
                    _ => panic!("invalid line {}", line)
                };
                let hand_to_play = match outcome {
                    hands::HandResult::Lose => other_hand.beats(),
                    hands::HandResult::Draw => other_hand,
                    hands::HandResult::Win => other_hand.beats().beats()
                };
                println!("Other played {:?}, need to {:?} so play {:?}", other_hand, outcome, hand_to_play);
                let mut new_points = 0;
                new_points += match outcome {
                    hands::HandResult::Win => 6,
                    hands::HandResult::Lose => 0,
                    hands::HandResult::Draw => 3   
                };
                new_points += match hand_to_play {
                    hands::Hand::Rock => 1,
                    hands::Hand::Paper => 2,
                    hands::Hand::Scissors => 3
                };

                println!("Got a {:?} with {:?}, earned {:?} points", outcome, hand_to_play, new_points);
                self_points += new_points
            }
        }
        println!("Part 2 {}", self_points)
    } 
}
