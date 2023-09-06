use std::fs::File;
use intersection::hash_set;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_n_line_chunks(filename: &str, num_chunks: usize) -> Vec<Vec<String>> {
    let mut file_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| match line {
            Ok(l) => {
                file_lines.push(l)
            },
            Err(_) => panic!()
        });
        return file_lines.chunks(num_chunks).map(|chunk| chunk.to_vec()).collect();
    }
    else {
        panic!("read lines err");
    }
}

fn get_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        return (item as u32) - 38; // convert uppercase ascii to 27-65 range
    }
    else if item.is_ascii_lowercase(){
        return (item as u32) - 96;  // and lowercase to 1-26 range
    }
    else {
        return 0;
    }
}

fn main() {
    // part 1
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_priority = 0;
        for rucksack_contents in lines {
            if let Ok(rucksack_contents) = rucksack_contents {
                let compartment_1: HashSet<char>  = (&rucksack_contents[..(rucksack_contents.len() / 2)]).chars().into_iter().collect();
                let compartment_2:HashSet<char> = (&rucksack_contents[(rucksack_contents.len() / 2)..]).chars().into_iter().collect();
                
                let priority: u32 = compartment_1.intersection(&compartment_2).into_iter().map(|chr| get_priority(*chr)).sum();
                total_priority += priority
            }
        }
        println!("Total priority: {}", total_priority);
    }

    // part 2
    let mut total_priority: u32 = 0;
    let lines = read_n_line_chunks("./input.txt", 3);
    for group in lines {
        // convert each line into a "hashset", get intersection
        let sets: Vec<Vec<char>> = group.iter().map(|string| string.chars().collect()).collect();
        // map the function to get priority value over the intersection of the three sets
        let priority: u32 = hash_set::intersection(sets).into_iter().map(|item| get_priority(item)).sum();

        total_priority += priority;

    }
    println!("Total priority (part 2): {}", total_priority)
    
}
