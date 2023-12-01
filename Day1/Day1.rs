use std::fs;
use std::error::Error;

fn get_input_lines(output: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input")?.parse()?;
    let lines_iter = input.split("\n");
    for line in lines_iter {
        output.push(line.to_string());
    }
    Ok(())
}

const NUMBER_LIST: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_number_from_left(line: &String) -> Result<i32, Box<dyn Error>> {
    for i in 0..line.len() {
        for n in 0..10 {
            // dbg!(&line[0..i+1]);
            if line[0..i+1].contains(&n.to_string()) {
                return Ok(n);
            }
            if line[0..i+1].contains(NUMBER_LIST[n as usize]) {
                return Ok(n);
            }
        }
    }
    Err(format!("No number found from left in '{}'", line).into())
}

fn find_number_from_right(line: &String) -> Result<i32, Box<dyn Error>> {
    for i in (0..line.len()).rev() {
        for n in 0..10 {
            // dbg!(&line[i..line.len()]);
            if line[i..line.len()].contains(&n.to_string()) {
                return Ok(n);
            }
            if line[i..line.len()].contains(NUMBER_LIST[n as usize]) {
                return Ok(n);
            }
        }
    }
    Err(format!("No number found from right in '{}'", line).into())
}

fn find_calibration_number(line: &String) -> Result<i32, Box<dyn Error>> {
    let left = find_number_from_left(line)?;
    let right = find_number_from_right(line)?;
    Ok(10*left + right)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: &mut Vec<String> = &mut Vec::new();
    get_input_lines(lines)?;
    let mut sum: i32 = 0;
    for line in lines {
        if line == "" { continue; }
        sum += find_calibration_number(&line)?;
    }
    println!("Sum of all calibration values: {}", sum);
    Ok(())
}
