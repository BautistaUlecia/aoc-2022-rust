use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;
    for line in contents.lines(){
        let (first, second) = line.split_at(line.len() / 2);
        part_one += solve(first, second);
    }
    let size = contents.lines().count();
    let mut lines = contents.lines();
    let mut i = 0;
    while (i < size) {
        let three_lines: Vec<_> = lines.by_ref().take(3).collect();
        i += 3;
        part_two += solve_two(three_lines);
    }

    println!("Part 1 total: {}", part_one);
    println!("Part 2 total: {}", part_two);
    fn solve(first: &str, second: &str) -> i32 {
        for char in first.chars(){
            if second.contains(char){
                println!("char found: {}, numerical value = {}", char, calculate(char));
                return calculate(char);
            }
        }
        panic!("No match found between two halves");
    }
    fn solve_two(three_lines: Vec<&str>) -> i32 {
        for c in three_lines.first().unwrap().chars(){
            if three_lines.get(1).unwrap().contains(c) &&
                three_lines.get(2).unwrap().contains(c){
                return calculate(c);
            }
        }
        panic!("No common character found between three lines");

    }

    pub fn calculate(c: char)-> i32{
        if c.is_lowercase(){
            c as i32 - 96 // Ascii value for "A" - 1
        }
        else {
            c as i32 - 64 + 26 // Ascii value for "a" - 1 + 26 for 26 values of a-z
        }
    }
}
