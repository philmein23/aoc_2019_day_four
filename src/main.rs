use std::fs;
use std::io;

fn main() -> std::io::Result<()> {
    let (low, high) = parse_input("input.txt")?;

    let count = get_valid_password_count(low, high);
    println!("Answer: {:?}", count);

    Ok(())
}

fn get_valid_password_count(low: u32, high: u32) -> usize {
    (low..=high)
        .filter(|&num| satisfies_password_criteria(&num))
        .count()
}

// Part 1 & Part 2
fn satisfies_password_criteria(num: &u32) -> bool {
    let mut freq = [0; 10];
    let digits = parse_digits(num);

    digits.iter().for_each(|&d| freq[d as usize] += 1);
    freq.iter().any(|&f| f == 2) && digits.windows(2).all(|d| d[0] <= d[1])
}

// Part 1
fn has_same_digits(digits: &Vec<u32>) -> bool {
    digits.windows(2).any(|d| d[0] == d[1])
}

fn parse_digits(to_be_parsed: &u32) -> Vec<u32> {
    let parsed = to_be_parsed.to_string();
    parsed
        .chars()
        .map(|s| s.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn parse_input(input: &str) -> Result<(u32, u32), io::Error> {
    let input = fs::read_to_string(input).unwrap();
    let mut tokenized = input.split("-");
    let low = tokenized
        .next()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .unwrap();
    let high = tokenized
        .next()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .unwrap();

    Ok((low, high))
}

mod tests {
    #[test]
    fn verify_valid_passwords() {
        use super::*;

        let test_case_one = 111133;
        let test_case_two = 122234;
        let test_case_three = 122255;

        assert_eq!(satisfies_password_criteria(&test_case_one), true);
        assert_eq!(satisfies_password_criteria(&test_case_two), false);
        assert_eq!(satisfies_password_criteria(&test_case_three), true);
    }
}
