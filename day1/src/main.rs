use std::{fs, collections::HashMap};

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let mut elves: HashMap<u8, u32> = HashMap::new();
    let mut elf_index = 0;
    let mut total_calories:u32 = 0;
    for string in file.lines() {
        if string.is_empty() {
            elves.insert(elf_index, total_calories);
            elf_index += 1;
            total_calories = 0;
        }
        else {
            total_calories += string.to_string().parse::<u32>().unwrap();
        }
    }

    let mut elves_vec: Vec<(&u8, &u32)> = elves.iter().collect();

    elves_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut count = 0;
    for (elf, calories) in elves_vec {
        println!("{} {}", elf, calories);
        count += 1;
        if count == 3 {
            break
        }
    }
}
