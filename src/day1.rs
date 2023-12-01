use std::fs;
use std::str::{Lines};

pub fn day1() {
    crate::advent::day_intro(1);
    const FILE_PATH: &str = "/usr/src/myapp/src/day1.txt";

    let _contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("Calibration total (first step) : {}", sum_calibrations(_contents.lines(), false));
    println!("Calibration total (second step) : {}", sum_calibrations(_contents.lines(), true));
}

pub fn sum_calibrations(lines:Lines, replace_spelled: bool) -> i32 {
    let mut calibration_total = 0;
    for line in lines {
        calibration_total += calibration_value(line, replace_spelled);
    }
    calibration_total
}


pub fn calibration_value(_line: &str, replace_spelled: bool) -> i32 {
    let mut _binding;
    if replace_spelled {
        _binding = replace_spelled_digits(_line);
    } else {
        _binding = _line.to_string();
    }
    let transformed_line = _binding.as_str();
    let firstdigit = extract_first_digit(transformed_line);
    let lastdigit = extract_last_digit(transformed_line);

    return firstdigit.unwrap_or(0) * 10 + lastdigit.unwrap_or(0);

    fn extract_first_digit(line: &str) -> Option<i32> {
        for character in line.chars() {
            if character.is_digit(10) {
                return Option::from(character.to_digit(10).unwrap() as i32);
            }
        }
        None
    }
    fn extract_last_digit(line: &str) -> Option<i32> {
        return extract_first_digit(line.chars().rev().collect::<String>().as_str());
    }
}

fn replace_spelled_digits(_line: &str) -> String {
    let _binding = _line;
    return _binding
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
    ;
}

#[cfg(test)]
mod tests {
    use crate::day1::{calibration_value, sum_calibrations};

    #[test]
    fn calibration_value_one_digit_in_line() {
        let _line = "treb7uchet";
        assert_eq!(77, calibration_value(_line, true))
    }

    #[test]
    fn calibration_value_two_digits_at_line_boundaries() {
        let _line = "1abc2";
        assert_eq!(12, calibration_value(_line, true))
    }

    #[test]
    fn calibration_value_two_digits_inside_line() {
        let _line = "pqr3stu8vwx";
        assert_eq!(38, calibration_value(_line, true))
    }

    #[test]
    fn calibration_value_several_digits_in_line() {
        let _line = "a1b2c3d4e5f";
        assert_eq!(15, calibration_value(_line, true))
    }

    #[test]
    fn calibration_value_two_spelled_in_line() {
        let _line = "two1";
        assert_eq!(21, calibration_value(_line, true))
    }

    #[test]
    fn calibration_value_four_and_nine_spelled_in_line() {
        let _line = "aafour1bnineb";
        assert_eq!(49, calibration_value(_line, true))
    }
    #[test]
    fn calibration_value_mixed_spelled_in_line() {
        let line = "xtwone3four";
        assert_eq!(24, calibration_value(line, true))
    }
    #[test]
    fn calibration_value_mixed_spelled_in_line_alt() {
        let line = "nineighthree";
        assert_eq!(93, calibration_value(line, true))
    }

    #[test]
    fn calibration_total_two_line() {
        let lines = "treb7uchet\n1abc2";
        assert_eq!(89, sum_calibrations(lines.lines(), true))
    }
}
