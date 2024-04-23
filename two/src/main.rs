use std::fs;

// rock 1 (A, X) , paper 2 (B, Y) , scissors 3 (C, Z)
// lost 0, tie 3, win 6

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    for line in contents.lines() {
        let his = line.chars().nth(0).unwrap();
        let mine = line.chars().nth(2).unwrap();
        let choices = (his, mine);
        part_one += play(choices);
        part_two += play_2(choices);
    }

    println!("Part one = {}. Part two = {}", part_one, part_two);

    fn play(choices: (char, char)) -> i32 {
        match choices {
            ('A', mine) => if mine == 'X' { 4 } else if mine == 'Y' { 8 } else { 3 },
            ('B', mine) => if mine == 'X' { 1 } else if mine == 'Y' { 5 } else { 9 },
            ('C', mine) => if mine == 'X' { 7 } else if mine == 'Y' { 2 } else { 6 },
            _ => 0,
        }
    }
    fn play_2(choices: (char, char)) -> i32 {
        match choices {
            ('A', mine) => if mine == 'X' { 3 } else if mine == 'Y' { 4 } else { 8 },
            ('B', mine) => if mine == 'X' { 1 } else if mine == 'Y' { 5 } else { 9 },
            ('C', mine) => if mine == 'X' { 2 } else if mine == 'Y' { 6 } else { 7 },
            _ => 0,
        }
    }
}
