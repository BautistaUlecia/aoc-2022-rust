use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");

    // Primera idea: leo bloque de 4 si son iguales avanzo si son distintos devuelvo indice final
    // 2 pointers?
    let mut left = 0;
    // let mut right = 4; part one
    let mut right = 14;
    while right < contents.len(){
        if check_uniqueness(&contents[left..right]){
            println!("Start of packet marker found on character number {}", right);
            return;
        }
        left += 1;
        right += 1;
    }

    fn check_uniqueness(slice: &str) -> bool{
        let set: HashSet<_> = slice.chars().collect();
        //set.len() == 4 part one
        set.len() == 14
    }
}
