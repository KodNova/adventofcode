use std::fs;

fn main() -> std::io::Result<()> {
    let mut answer: u128 = 0;
    let content = fs::read_to_string("input")?;

    for item in content
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        if let Some((x_str, y_str)) = item.split_once('-') {
            let mut first: u128 = x_str.parse().expect("Failed to parse first");
            let last: u128 = y_str.parse().expect("Failed to parse last");

            while first <= last {
                if is_invalid(&first) {
                    answer += first;
                    println!("{first}")
                }
                first += 1;
            }
        }
    }
    println!("Solution:{answer}");
    Ok(())
}

fn is_invalid(number: &u128) -> bool {
    let number_str: String = number.to_string();
    let num_len = number_str.len();

    if num_len <= 1 {
        return false;
    }

    for chunk_size in 1..=(num_len / 2) {
        if !num_len.is_multiple_of(chunk_size) {
            continue;
        }

        let chunk: String = number_str.chars().take(chunk_size).collect();
        if number_str == chunk.repeat(num_len / chunk_size) {
            return true;
        }
    }

    false
}
