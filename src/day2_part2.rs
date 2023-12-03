pub mod day2_part2 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn day2_part2() {
        let path_str = "input/day2.txt";
        
        let file = File::open(path_str).expect("Cant open file");

        let reader = BufReader::new(file);

        let mut sum: i32 = 0;
        for line in reader.lines() {
            let line = line.expect("Cant read line");
            let split_id: Vec<&str> = line.split(":").collect();
            let min_rgb = calculate_min_rgb_game(split_id[1]);
            let power = calculate_power_game(min_rgb);
            println!("Power: {}", power.to_string());
            sum = sum + power;
        }
        println!("Day 2, Part 2 - Sum: {}", sum);
    }

    fn calculate_power_game(min_rgb: [i32; 3]) -> i32 {
        let mut power: i32 = 1;
        for val in min_rgb {
            if val > 0 {
                power = power * val;
            }
        }
        return power;
    }

    fn calculate_min_rgb_game(game: &str) -> [i32; 3] {
        // println!("Games: {}", game);
        
        let plays: Vec<&str> = game.split(";").collect();
        let mut min_rgb = [0, 0, 0];
        for play in plays {
            min_rgb = calculate_min_rgb_play(play, min_rgb);
        }
        return min_rgb;

    }

    fn calculate_min_rgb_play(play: &str, mut min_rgb: [i32; 3]) -> [i32; 3] {
        let split_play: Vec<&str> = play.split(",").collect();

        for split in split_play {
            let split_colors: Vec<&str> = split.split(" ").collect();
            let value = split_colors[1].parse::<i32>().unwrap();
            let color = split_colors[2];
            // println!("Split: {}, Color: {}, Value: {}", split, color, value);
            if (color == "red" && value > min_rgb[0]) {
                min_rgb[0] = value;
            } else if (color == "green" && value > min_rgb[1]) {
                min_rgb[1] = value;
            } else if (color == "blue" && value > min_rgb[2]) {
                min_rgb[2] = value;
            }
        }
        return min_rgb;
    }
}
