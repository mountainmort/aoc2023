pub mod day1_part2 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::str::FromStr;

    pub fn day1_part2() {
        let path_str = "input/day1.txt";
        
        let file = File::open(path_str).expect("Cant open file");

        let reader = BufReader::new(file);

        let mut sum: i32 = 0;
        for line in reader.lines() {
            let line = line.expect("Cant read line");
            sum = sum + get_calibration_value(line);
        }
        println!("Day 1, Part 2 - Sum: {}", sum);
    }

    fn get_calibration_value(line: String) -> i32 {
        // println!("Line: {}", line);
        let first_num = find_digit(&line, false);
        let last_num = find_digit(&line, true);
        let str: String = format!("{}{}", first_num, last_num);
        //println!("Calibration value: {}", str);
        return i32::from_str(&str).unwrap();
    }

    fn find_digit(line: &String, reverse: bool) -> i32 {
        if reverse {
            for i in (0..line.len()).rev() {
                let digit = get_numeric_or_digit_string(line, i);
                if digit != -1 {
                    return digit;
                }
            } 
        } else {
            for i in 0..line.len() {
                let digit = get_numeric_or_digit_string(line, i);
                if digit != -1 {
                    return digit;
                }
            }
        }
        return 0;
    }

    fn get_numeric_or_digit_string(string: &String, index: usize) -> i32 {
        if string.chars().nth(index).unwrap().is_numeric() {
            return string.chars().nth(index).unwrap().to_digit(10).unwrap() as i32;
        } else {
            let suffix: String = string.get(index..string.len()).unwrap().to_string();
            
            if suffix.len() < 3 {
                return -1;
            } else {
                if suffix.starts_with("one") {
                    return 1;
                } else if suffix.starts_with("two") {
                    return 2;
                } else if suffix.starts_with("three") {
                    return 3;
                } else if suffix.starts_with("four") {
                    return 4;
                } else if suffix.starts_with("five") {
                    return 5;
                } else if suffix.starts_with("six") {
                    return 6;
                } else if suffix.starts_with("seven") {
                    return 7;
                } else if suffix.starts_with("eight") {
                    return 8;
                } else if suffix.starts_with("nine") {
                    return 9;
                } else {
                    return -1;
                }
            }
        }
    }
}
