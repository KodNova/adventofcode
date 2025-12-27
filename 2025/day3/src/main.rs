use std::fs;

fn main() {
    let _place_hold: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    let contents = fs::read_to_string("input").expect("failed to read file");

    let answer: u32 = contents.lines().map(find_joltage).sum();

    let testing_answer: u32 = _place_hold
        .iter()
        .map(|num_str| find_joltage(num_str))
        .sum();

    println!("Testing Solution: {}", testing_answer);
    println!("Solution: {}", answer);
}

fn find_joltage(num_str: &str) -> u32 {
    num_str
        .chars()
        .enumerate()
        .take(num_str.len() - 1)
        .fold(0, |highest_yet, (index, ch)| {
            let merged = ch.to_string() + &highest_number_in_str(num_str, index);
            let num: u32 = merged.parse().expect("failed to parse into number");
            highest_yet.max(num)
        })
}

fn highest_number_in_str(num_str: &str, start_index: usize) -> String {
    let new_num_str = &num_str[start_index + 1..];
    new_num_str
        .chars()
        .fold(0, |highest: u32, ch: char| {
            let num: u32 = ch.to_digit(10).expect("failed the make ch into number");
            highest.max(num)
        })
        .to_string()
}
