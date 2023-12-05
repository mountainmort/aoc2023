pub mod day3_part2 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::str::FromStr;


    pub fn day3_part2() {
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
            
            for x in 0..lines[y].len() {
                let cur_char = lines[y][x];
                if cur_char == '*' {
                    // println!("Star at {}, {}", x, y);
                    let adjacent_numbers = find_adjacent_numbers(&lines, y, x);
                    if adjacent_numbers.len() == 2 {
                        sum = sum + (adjacent_numbers[0] * adjacent_numbers[1]);
                        // println!("Sum: {}", sum);
                    } else {
                        // println!("Error: Found {} adjacent numbers", adjacent_numbers.len());
                    }
                }
            }
        }

        println!("Day 3, Part 2 - Sum: {}", sum);
    }

    fn find_adjacent_numbers(lines: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<i32> {
        let mut adjacent_numbers = Vec::<i32>::new();
        
        // check left
        if x > 0 {
            if lines[y][x-1].is_numeric() {
                let mut cur_num_str = lines[y][x-1].to_string();
                let mut cur_x = x - 1;
                while cur_x > 0 && lines[y][cur_x-1].is_numeric() {
                    cur_num_str = format!("{}{}", lines[y][cur_x-1], cur_num_str);
                    cur_x = cur_x - 1;
                }
                adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                // println!("Found num left: {}", cur_num_str);
            }
        }

        // check right
        if x < lines[y].len() - 1 {
            if lines[y][x+1].is_numeric() {
                let mut cur_num_str = lines[y][x+1].to_string();
                let mut cur_x = x + 1;
                while cur_x < lines[y].len() - 1 && lines[y][cur_x+1].is_numeric() {
                    cur_num_str = format!("{}{}", cur_num_str, lines[y][cur_x+1]);
                    cur_x = cur_x + 1;
                }
                adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                // println!("Found num right: {}", cur_num_str);
            }
        }

        // check up
        if y > 0 {
            if lines[y-1][x].is_numeric() {
                let mut cur_num_str = lines[y-1][x].to_string();

                // println!("Initial: {}", cur_num_str);
                // check left
                let mut cur_x = x - 1;
                while cur_x > 0 && lines[y-1][cur_x].is_numeric() {
                    cur_num_str = format!("{}{}", lines[y-1][cur_x], cur_num_str);
                    if cur_x == 0 {
                        break;
                    } else {
                        cur_x = cur_x - 1;
                    }
                }
                // println!("Scroll left: {} - {}", cur_x, x);


                // check right
                cur_x = x + 1;
                // println!("Scroll right: {} - {}", cur_x, x);
                while cur_x <= lines[y-1].len() - 1 && lines[y-1][cur_x].is_numeric() {
                    cur_num_str = format!("{}{}", cur_num_str, lines[y-1][cur_x]);
                    cur_x = cur_x + 1;
                }
                // println!("Finished scrolling: {}", cur_num_str);

                adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                // println!("Found num above: {}", cur_num_str);
            } else {

                // check up left
                if x > 0 && lines[y-1][x-1].is_numeric() {
                    let mut cur_num_str = lines[y-1][x-1].to_string();

                    // scroll left
                    let mut cur_x = x - 1;
                    while cur_x > 0 && lines[y-1][cur_x-1].is_numeric() {
                        cur_num_str = format!("{}{}", lines[y-1][cur_x-1], cur_num_str);
                        cur_x = cur_x - 1;
                    }
                    adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                    // println!("Found num up left: {}", cur_num_str);
                }
                
                // check up right
                if x < lines[y].len() - 1 && lines[y-1][x+1].is_numeric() {
                    let mut cur_num_str = lines[y-1][x+1].to_string();

                    // scroll right
                    let mut cur_x = x + 1;
                    while cur_x < lines[y-1].len() - 1 && lines[y-1][cur_x+1].is_numeric() {
                        cur_num_str = format!("{}{}", cur_num_str, lines[y-1][cur_x+1]);
                        cur_x = cur_x + 1;
                    }
                    adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                    // println!("Found num up right: {}", cur_num_str);
                }
            }
        }

        // check down
        if y < lines.len() - 1 {
            
            if lines[y+1][x].is_numeric() {
                let mut cur_num_str = lines[y+1][x].to_string();

                // println!("Initial: {}", cur_num_str);
                // check left
                let mut cur_x = x - 1;
                while cur_x > 0 && lines[y+1][cur_x].is_numeric() {
                    cur_num_str = format!("{}{}", lines[y+1][cur_x], cur_num_str);
                    if cur_x == 0 {
                        break;
                    } else {
                        cur_x = cur_x - 1;
                    }
                }
                // println!("Scroll left: {} - {}", cur_x, x);


                // check right
                cur_x = x + 1;
                // println!("Scroll right: {} - {}", cur_x, x);
                while cur_x <= lines[y+1].len() - 1 && lines[y+1][cur_x].is_numeric() {
                    cur_num_str = format!("{}{}", cur_num_str, lines[y+1][cur_x]);
                    cur_x = cur_x + 1;
                }
                //println!("Finished scrolling: {}", cur_num_str);

                adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                // println!("Found num below: {}", cur_num_str);
            } else {

                // check down left
                if x > 0 && lines[y+1][x-1].is_numeric() {
                    let mut cur_num_str = lines[y+1][x-1].to_string();

                    // check left
                    let mut cur_x = x - 1;
                    while cur_x > 0 && lines[y+1][cur_x-1].is_numeric() {
                        cur_num_str = format!("{}{}", lines[y+1][cur_x-1], cur_num_str);
                        cur_x = cur_x - 1;
                    }
                    adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                    // println!("Found num down left: {}", cur_num_str);
                }
                
                // check down right
                if x < lines[y+1].len() - 1 && lines[y+1][x+1].is_numeric() {
                    let mut cur_num_str = lines[y+1][x+1].to_string();

                    // check left
                    let mut cur_x = x + 1;
                    while cur_x < lines[y+1].len() - 1 && lines[y+1][cur_x+1].is_numeric() {
                        cur_num_str = format!("{}{}", cur_num_str, lines[y+1][cur_x+1]);
                        cur_x = cur_x + 1;
                    }
                    adjacent_numbers.push(i32::from_str(cur_num_str.as_str()).unwrap());
                    // println!("Found num down right: {}", cur_num_str);
                }
            }
        }
        return adjacent_numbers;
    }

}
