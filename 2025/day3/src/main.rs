use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("failed to read file");

    let answer: u128 = contents.lines().map(find_joltage_two_digits).sum();
    let answer2: u128 = contents.lines().map(find_joltage_twelve_digits).sum();

    println!("Solution: {}", answer);
    println!("Solution2: {}", answer2);
}

fn find_joltage_two_digits(num_str: &str) -> u128 {
    num_str
        .char_indices()
        .filter(|(_, ch)| ch.is_ascii_digit())
        .filter(|(index, _)| *index < num_str.len() - 1)
        .map(|(index, ch)| {
            let dig = ch.to_digit(10).expect("error ch to digit") as u128;
            let rest = &num_str[index + 1..];
            dig * 10 + highest_digit_in_str(rest)
        })
        .max()
        .expect("whoa there")
}

fn find_joltage_twelve_digits(num_str: &str) -> u128 {
    let digits_needed = 12;
    let chars: Vec<char> = num_str.chars().collect();
    let len = chars.len();

    let (result, _) =
        (1..=digits_needed)
            .rev()
            .fold((String::new(), 0), |(mut result, start_idx), remaining| {
                let end_idx = len - remaining + 1;

                let (max_pos, _) = (start_idx..end_idx)
                    .map(|i| (i, chars[i].to_digit(10).unwrap()))
                    .fold((start_idx, 0), |(best_i, best_d), (i, d)| {
                        if d > best_d { (i, d) } else { (best_i, best_d) }
                    });

                result.push(chars[max_pos]);
                (result, max_pos + 1)
            });

    result.parse().expect("failed to parse")
}

fn highest_digit_in_str(num_str: &str) -> u128 {
    num_str
        .chars()
        .map(|ch| ch.to_digit(10).expect("error ch to digit") as u128)
        .max()
        .expect("not a number")
}
