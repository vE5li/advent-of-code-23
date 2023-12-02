use std::fs::read_to_string;

/// https://adventofcode.com/2023/day/1

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Split input into lines and filter out empty lines.
    let lines = input.split('\n').filter(|line| !line.is_empty());

    // Digit lookup table for part 1.
    const DIGIT_TABLE_PART_1: &[(&str, usize)] = &[
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    // Digit lookup table for part 2.
    const DIGIT_TABLE_PART_2: &[(&str, usize)] = &[
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Calculate the sum of all calibration values given some input.
    fn calibration_values_sum_for_table<'a>(
        lines: impl Iterator<Item = &'a str>,
        table: &[(&str, usize)],
    ) -> usize {
        // Iterate over the table and find the first occurrence of every entry in the line, then
        // find the match with the smallest position value.
        let get_first_digit = |line: &str| {
            table
                .iter()
                .filter_map(|(name, value)| line.find(name).zip(Some(value)))
                .min_by_key(|(position, _value)| *position)
                .map(|(_position, value)| *value)
                .unwrap()
        };

        // Same as `get_first_digit` but find the last occurrence of every entry and choose the one
        // with the biggest position value.
        let get_last_digit = |line: &str| {
            table
                .iter()
                .filter_map(|(name, value)| line.rfind(name).zip(Some(value)))
                .max_by_key(|(position, _value)| *position)
                .map(|(_position, value)| *value)
                .unwrap()
        };

        // Convert each line to a single calibration value.
        let calibration_values = lines.map(|line| {
            // Get the first and last digit.
            let first_digit = get_first_digit(line);
            let last_digit = get_last_digit(line);

            // Compute final calibration value with the first and last digit.
            first_digit * 10 + last_digit
        });

        // Sum all calibration values.
        calibration_values.sum::<usize>()
    }

    // Solution for part 1.
    println!(
        "The sum of all calibration values using only digits is {}",
        calibration_values_sum_for_table(lines.clone(), DIGIT_TABLE_PART_1)
    );

    // Solution for part 2.
    println!(
        "The sum of all calibration values using digits and words is {}",
        calibration_values_sum_for_table(lines, DIGIT_TABLE_PART_2)
    );
}
