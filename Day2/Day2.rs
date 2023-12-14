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

type MinCubes = (i32, i32, i32);

fn get_min_possible_cubes(game: &Game) -> MinCubes {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;
    for reveal in &game.reveals {
        if reveal.0 > max_r { max_r = reveal.0}
        if reveal.1 > max_g { max_g = reveal.1}
        if reveal.2 > max_b { max_b = reveal.2}
    }
    (max_r, max_g, max_b)
}

fn get_power(game: &Game) -> i32 {
    let min_cubes = get_min_possible_cubes(game);
    min_cubes.0 * min_cubes.1 * min_cubes.2
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: &mut Vec<String> = &mut Vec::new();
    get_input_lines(lines)?;
    let mut sum: i32 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let game = parse_line(line)?;
        sum += get_power(&game);
    }
    println!("Sum of game powers: {}", sum);
    Ok(())
}
