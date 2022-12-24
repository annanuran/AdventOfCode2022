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
    let mut current_total_points = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut total_points = 0;
                play_hand
            }
        }
    }
}
