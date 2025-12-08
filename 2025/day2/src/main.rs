fn main() {
    let mut answer: u128 = 0;
    let content: [&str; 11] = [
        "11-22",
        "95-115",
        "998-1012",
        "1188511880-1188511890",
        "222220-222224",
        "1698522-1698528",
        "446443-446449",
        "38593856-38593862",
        "565653-565659",
        "824824821-824824827",
        "2121212118-2121212124",
    ];

    for item in content {
        if let Some((x_str, y_str)) = item.split_once('-') {
            let mut first: u128 = x_str.parse().expect("Failed to parse first");
            let last: u128 = y_str.parse().expect("Failed to parse last");

            while first <= last {
                let first_str = first.to_string();

                if is_invalid(&first_str) {
                    answer += first;
                    println!("{first}")
                }
                first += 1;
            }
        }
    }
    println!("Solution:{answer}")
}

fn is_invalid(number_str: &str) -> bool {
    let middle = number_str.len() / 2;
    let (left, right) = number_str.split_at(middle);
    if left == right {
        return true;
    }
    false
}
