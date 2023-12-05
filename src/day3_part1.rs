pub mod day3_part1 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::str::FromStr;


    pub fn day3_part1() {
        let path_str = "input/day3.txt";
        
        let file = File::open(path_str).expect("Cant open file");

        let reader = BufReader::new(file);

        let mut lines = Vec::<Vec<char>>::new();
        for line in reader.lines() {
            let line = line.expect("Cant read line");
            lines.push(line.chars().collect());
        }

        let mut sum: i32 = 0;
        for y in 0..lines.len() {
            // println!("Line {}", y);
            
            let mut cur_num_str = "".to_owned();
            let mut cur_num_str_is_part = false;
            for x in 0..lines[y].len() {
                let cur_char = lines[y][x];
                if cur_char.is_numeric() {
                    cur_num_str = format!("{}{}", cur_num_str.to_owned(), cur_char);
                    if has_symbol_neighbor(&lines, y, x) {
                        cur_num_str_is_part = true;
                    }
                } else {
                    if cur_num_str != "" && cur_num_str_is_part {
                        let cur_num = i32::from_str(cur_num_str.as_str()).unwrap();
                        sum = sum + cur_num;
                        // println!("Current Num: {} - Sum: {}", cur_num, sum);   
                    }
                    cur_num_str = "".to_owned();
                    cur_num_str_is_part = false;
                }
            }
            if cur_num_str != "" && cur_num_str_is_part {
                let cur_num = i32::from_str(cur_num_str.as_str()).unwrap();
                sum = sum + cur_num;
                // println!("Current Num: {} - Sum: {}", cur_num, sum);
            }
        }

        println!("Day 3, Part 1 - Sum: {}", sum);
    }

    fn has_symbol_neighbor(lines: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
        // check left
        if x > 0 {
            if !is_numeric_or_dot(lines[y][x-1]) {
                return true;
            }
        }

        // check right
        if x < lines[y].len() - 1 {
            if !is_numeric_or_dot(lines[y][x+1]) {
                return true;
            }
        }

        // check up
        if y > 0 {
            if !is_numeric_or_dot(lines[y-1][x]) {
                return true;
            }
        }

        // check down
        if y < lines.len() - 1 {
            if !is_numeric_or_dot(lines[y+1][x]) {
                return true;
            }
        }

        // check up left
        if y > 0 && x > 0 {
            if !is_numeric_or_dot(lines[y-1][x-1]) {
                return true;
            }
        }

        // check up right
        if y > 0 && x < lines[y].len() - 1 {
            if !is_numeric_or_dot(lines[y-1][x+1]) {
                return true;
            }
        }

        // check down left
        if y < lines.len() - 1 && x > 0 {
            if !is_numeric_or_dot(lines[y+1][x-1]) {
                return true;
            }
        }

        // check down right 
        if y < lines.len() - 1 && x < lines[y].len() - 1 {
            if !is_numeric_or_dot(lines[y+1][x+1]) {
                return true;
            }
        }

        return false;
    }

    fn is_numeric_or_dot(c: char) -> bool {
        return c.is_numeric() || c == '.';
    }
}
