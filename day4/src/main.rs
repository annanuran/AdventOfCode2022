use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn range_encompassed(first_start: i32, first_end: i32, snd_start: i32, snd_end: i32) -> bool {
// by default, we know that if one range is smaller than the other, it cannot encompass the larger one. Hence, we must only check if the larger range contains the larger one
    let size_of_1: i32 = first_end - first_start;
    let size_of_2: i32 = snd_end - snd_start;

    if size_of_1 > size_of_2 {
        return first_start <= snd_start && first_end >= snd_end;
    }
    else {
        return snd_start <= first_start && snd_end >= first_end;
    }
}

fn range_overlaps(first_start: i32, first_end: i32, snd_start: i32, snd_end: i32) -> bool {
    // a range overlaps if the beginning of a range is inside another range

        let first_start_in_snd =  first_start <= snd_end && first_start >= snd_start;
        let snd_start_in_first = snd_start <= first_end && snd_start >= first_start;
        
    
        return first_start_in_snd || snd_start_in_first;
    
        
    
    }

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_found = 0;
        for line in lines {
            if let Ok(line) = line {
                let ranges: Vec<&str> = line.split(",").collect();
                let range1: Vec<&str> = ranges[0].split("-").collect();
                let range2: Vec<&str> = ranges[1].split("-").collect();
                if range_encompassed(range1[0].parse().unwrap(), range1[1].parse().unwrap(), range2[0].parse().unwrap(), range2[1].parse().unwrap()) {
                    total_found +=1;
                }
            }
        }
        println!("Total ranges encompassed: {}", total_found);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_found = 0;
        for line in lines {
            if let Ok(line) = line {
                let ranges: Vec<&str> = line.split(",").collect();
                let range1: Vec<&str> = ranges[0].split("-").collect();
                let range2: Vec<&str> = ranges[1].split("-").collect();
                if range_overlaps(range1[0].parse().unwrap(), range1[1].parse().unwrap(), range2[0].parse().unwrap(), range2[1].parse().unwrap()) {
                    total_found +=1;
                }
            }
        }
        println!("Total ranges overlapping: {}", total_found);
    }
}
