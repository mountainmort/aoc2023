pub mod day2_part1 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn day2_part1() {
        let path_str = "input/day2.txt";
        
        let file = File::open(path_str).expect("Cant open file");

        let reader = BufReader::new(file);

        let mut sum: i32 = 0;
        for line in reader.lines() {
            let line = line.expect("Cant read line");
            let split_id: Vec<&str> = line.split(":").collect();
            if is_valid_game(split_id[1]) { 
                let split_game: Vec<&str> = split_id[0].split(" ").collect();
                let id = split_game[1].parse::<i32>().unwrap();
                // println!("Id: {}", id.to_string());
                sum = sum + id; 
            }
            
        }
        println!("Day 2, Part 1 - Sum: {}", sum);
    }

    fn is_valid_game(game: &str) -> bool {
        // println!("Games: {}", game);
        
        let plays: Vec<&str> = game.split(";").collect();
        for play in plays {
            if !is_valid_play(play) { return false; }
        }
        return true;

    }

    fn is_valid_play(play: &str) -> bool {
        let split_play: Vec<&str> = play.split(",").collect();

        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        for split in split_play {
            let split_colors: Vec<&str> = split.split(" ").collect();
            let value = split_colors[1].parse::<i32>().unwrap();
            let color = split_colors[2];
            // println!("Split: {}, Color: {}, Value: {}", split, color, value);
            if (color == "red" && value > max_red) || (color == "green" && value > max_green) || (color == "blue" && value > max_blue) { return false; }
        }

        return true;
    }

}
