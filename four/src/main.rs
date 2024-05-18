use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut part_one = 0;
    let mut part_two = 0;
    for line in contents.lines(){
        let parts: Vec<&str> = line.split(',').collect();
        let[first, second] = parts.as_slice() else { panic!("Could not separate by comma, input is malformed.") };
        let first_parts: Vec<&str> = first.split('-').collect();
        let [first_lower, first_upper] = first_parts.as_slice() else { panic!("Could not separate by - (whatever its called)") };
        let second_parts: Vec<&str> = second.split('-').collect();
        let [second_lower, second_upper] = second_parts.as_slice() else { panic!("Could not separate by - (whatever its called)") };

        if is_contained(first_lower, first_upper, second_lower, second_upper) {
            part_one += 1;
        }
        if overlaps(first_lower, first_upper, second_lower, second_upper){
            part_two += 1;
        }

    }
    println!("part one = {}", part_one);
    println!("part two = {}", part_two);
    fn is_contained(first_lower:&str, first_upper: &str, second_lower:&str, second_upper:&str) -> bool{
        if to_int(first_lower) <= to_int(second_lower) && to_int(first_upper) >= to_int(second_upper){return true;}// first contains second
        if to_int(first_lower) >= to_int(second_lower) && to_int(first_upper) <= to_int(second_upper){return true;} // second contains first
        return false;
    }
    pub fn overlaps (first_lower:&str, first_upper: &str, second_lower:&str, second_upper:&str) -> bool{
        if to_int(first_upper) >= to_int(second_lower) && to_int(first_lower) <= to_int(second_upper) {return true;}
        return false;
    }
    pub fn to_int(a:&str) -> i32{
        return a.parse::<i32>().expect("Could not convert string into i32");
    }
}