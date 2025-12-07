use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("file fail");
    let mut dial = 50;
    let mut answer = 0;

    for line in content.lines() {
        let key = line.trim();
        let direction = key.starts_with("R");
        let amount: i32 = key[1..].parse().expect("Failed to parse number from key");

        match direction {
            true => {
                dial += amount;
            }
            false => {
                dial -= amount;
            }
        }

        dial %= 100;

        if dial == 0 {
            answer += 1;
        }
    }
    println!("Solution: {answer}")
}

