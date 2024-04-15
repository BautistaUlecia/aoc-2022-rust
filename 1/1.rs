use std::cmp;
use std::fs;
fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut elves: Vec<i32> = vec![];
    let mut max: i32 = 0;
    let mut curr: i32 = 0;

    for line in contents.lines() {
        match line.parse::<i32>() {
            Ok(value) => curr += value,
            Err(_) => {
                max = cmp::max(max, curr);
                elves.push(curr);
                curr = 0;
            }
        }
    }
    elves.sort();
    elves.reverse();

    println!("Biggest elf = {}", elves.first().unwrap());
    let sum_3: i32 = elves.iter().take(3).sum();
    println!("Sum of biggest 3 = {}", sum_3)
}
