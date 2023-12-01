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

fn find_number_from_left(line: &String) -> Result<i32, Box<dyn Error>> {
    for c in line.bytes() {
        if c >= b'0' && c <= b'9' {
            return Ok((c - b'0') as i32);
        }
        
    }
    Err(format!("No number found in '{}'", line).into())
}

fn find_number_from_right(line: &String) -> Result<i32, Box<dyn Error>> {
    for c in line.bytes().rev() {
        if c >= b'0' && c <= b'9' {
            return Ok((c - b'0') as i32);
        }
    }
    Err(format!("No number found in '{}'", line).into())
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
