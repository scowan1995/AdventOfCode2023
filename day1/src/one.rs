use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[allow(dead_code)]
fn runone() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for myline in reader.lines() {
        match myline {
            Ok(line) => {
                println!("{}", line);
                let mut first_val = 'a';
                let mut found_first = false;
                let mut last_val = 'a';
                for c in line.chars() {
                    if c.is_digit(10) && !found_first {
                        first_val = c;
                        found_first = true;
                    } 
                    if c.is_digit(10) {
                        last_val = c
                    }
                }
                let mut combined_string = String::new();
                println!("first {}", first_val);
                combined_string.push(first_val);
                if last_val.is_digit(10) {
                    println!("second {}", last_val);
                    combined_string.push(last_val);
                }
                let parsed_number: Result<i32, _> = combined_string.parse();

                match parsed_number {
                    Ok(n) => {
                        println!("Successfully parsed to integer: {}", n);
                        sum += n

                        // You can now use 'n' as an integer
                    }
                    Err(e) => {
                        println!("Failed to parse to integer: {}", e);
                    }
                }
            }
            Err(_e) => {
                println!("yikes")
            }
        }
    }
    println!("Total value: {}", sum);
    Ok(())
}
