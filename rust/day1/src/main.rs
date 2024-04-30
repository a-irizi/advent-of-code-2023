fn main() {
    let file_name = "input.txt";
    let calibration: u64 = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let digit_chars = line
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<Vec<_>>();

            let (first_digit, last_digit) = if digit_chars.len() > 1 {
                let last_element_index = digit_chars.len() - 1;
                (digit_chars[0], digit_chars[last_element_index])
            } else {
                (digit_chars[0], digit_chars[0])
            };

            format!("{}{}", first_digit, last_digit)
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    println!("Calibration value is {calibration}");
}
