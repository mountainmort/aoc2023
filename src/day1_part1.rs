pub mod day1_part1 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::str::FromStr;


    pub fn day1_part1() {
        let path_str = "input/day1.txt";
        
        let file = File::open(path_str).expect("Cant open file");

        let reader = BufReader::new(file);

        let mut sum: i32 = 0;
        for line in reader.lines() {
            let line = line.expect("Cant read line");
            sum = sum + get_calibration_value(line);
        }
        println!("Day 1, Part 1 - Sum: {}", sum);
    }

    fn get_calibration_value(line: String) -> i32 {

        let chars: Vec<char> = line.chars().collect();
        let first_num = chars.iter().find(|c| c.is_numeric()).unwrap();
        let last_num = chars.iter().rfind(|c| c.is_numeric()).unwrap();
        let str: String = format!("{}{}", first_num, last_num);
        return i32::from_str(&str).unwrap();
    }
}
