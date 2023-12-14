use std::fs;
use std::error::Error;

use regex::Regex;

#[derive(Debug)]
struct Game {
    id: i32,
    reveals: Vec<Reveal>,
}

type Reveal = (i32, i32, i32);

fn get_input_lines(output: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input")?.parse()?;
    let lines_iter = input.split("\n");
    for line in lines_iter {
        output.push(line.to_string());
    }
    Ok(())
}

// example: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_line(line: &String) -> Result<Game, Box<dyn Error>> {
    let game_number_regex: regex::Regex = Regex::new(r"^Game (\d+):").unwrap();
    let red_regex: regex::Regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex: regex::Regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex: regex::Regex = Regex::new(r"(\d+) blue").unwrap();
    
    let str_line = line.as_str();
    let Some(game_caps) = game_number_regex.captures(str_line) else {
        return Err("No regex capture!".into());
    };
    let id: i32 = game_caps[1].parse().unwrap();

    let mut reveals: Vec<Reveal> = Vec::new();
    for reveal in line.split(":").collect::<Vec<&str>>()[1].split(";") {
        let r_caps = red_regex.captures(reveal);
        let mut r: i32 = 0;
        if r_caps.is_some() {
            r = r_caps.unwrap()[1].parse::<i32>().unwrap()
        }
        let g_caps = green_regex.captures(reveal);
        let mut g: i32 = 0;
        if g_caps.is_some() {
            g = g_caps.unwrap()[1].parse::<i32>().unwrap()
        }
        let b_caps = blue_regex.captures(reveal);
        let mut b: i32 = 0;
        if b_caps.is_some() {
            b = b_caps.unwrap()[1].parse::<i32>().unwrap()
        }
        reveals.push((r,g,b));
    }
    Ok(Game {
        id: id,
        reveals: reveals,
    })
}

fn is_game_possible(game: &Game) -> bool {
    // "only 12 red cubes, 13 green cubes, and 14 blue cubes"
    for reveal in &game.reveals {
        if reveal.0 > 12 { return false; }
        if reveal.1 > 13 { return false; }
        if reveal.2 > 14 { return false; }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: &mut Vec<String> = &mut Vec::new();
    get_input_lines(lines)?;
    let mut sum: i32 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let game = parse_line(line)?;
        if is_game_possible(&game) { sum += game.id; }
    }
    println!("Sum of possible game IDs: {}", sum);
    Ok(())
}
