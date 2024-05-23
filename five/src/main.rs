use std::collections::VecDeque;
use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");

    // Hardcoded size, could parse line that only contains numbers to get it.
    let mut stacks : [VecDeque<char>; 9] = Default::default();// defaults to empty vector

    for line in contents.lines() {
        match line.chars().find(|&c| c == '[' || c == 'm') {
            Some('[') => add_to_stacks(line, &mut stacks),
            Some('m') => operate(line, &mut stacks),
            _ => (),
        }

    }
    for stack in stacks{
        println!("{}", stack.iter().last().unwrap());
    }
}
pub fn add_to_stacks(line: &str, stacks: &mut [VecDeque<char>; 9]){
    for (i, char) in line.chars().enumerate(){
        if char.is_alphabetic(){
            stacks[i / 4].push_front(char); // find corresponding array
        }
    }
}
pub fn operate(line: &str, stacks: &mut [VecDeque<char>; 9]) {
    let numbers: Vec<_> = line.split(' ').filter(|word| word.chars().all(|c| c.is_numeric())).collect();
    let amount = numbers[0].parse::<usize>().unwrap();
    let from = numbers[1].parse::<usize>().unwrap() - 1;
    let to = numbers[2].parse::<usize>().unwrap() - 1;

    // one

    // for _ in 0..amount {
    //   let popped = stacks[from].pop_back().unwrap();
    //  stacks[to].push_back(popped);
    // }

    // two

    // let mut temp = stacks[from].split_off(stacks[from].len() - amount);
    // stacks[to].append(&mut temp);
}
