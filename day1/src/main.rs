use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut current_maxes = Vec::new();
    let mut current_total: i32  = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                match line.chars().next() {
                    Some(character) =>  {
                        if line.chars().next().unwrap().is_numeric() {
                            let curr_amount: i32 = line.parse().unwrap();
                            current_total = current_total + curr_amount;
                        }
                    },
                    None => {
                        current_maxes.push(current_total);
                        current_maxes.sort_by(|a, b| b.cmp(a));
                        current_maxes.resize(3, -1);
                        current_total = 0;
                    }
                }
            }
        }
    }
    println!("{:?}", current_maxes);
    let sum: i32 = current_maxes.iter().fold(0, |mut sum, &val| {sum += val; sum});
    println!("{}", sum);
}
