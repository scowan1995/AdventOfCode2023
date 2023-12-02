use std::fs::File;
use std::convert::TryFrom;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::fmt;
use std::io::Error;
use std::num::ParseIntError;



// this is not needed, leftover from earlier work, cant be bothered to remove
struct NumberAndOffset {
    number: Option<char>,
    offset: i32
}
impl fmt::Display for NumberAndOffset{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self.number {
            Some(x) => {
                write!(f, "number: {}, offset: {}", x, self.offset)
            }
            None => {write!(f, "None, {}", self.offset)}
        }
    }
}

fn get_number(string: &Vec<char>, index: usize, _leftover_size: i32) -> NumberAndOffset {
    // must be non-overlapping e,g, twone3 should be 23 not 21
    // if leftover_size != 0 {
    //    return NumberAndOffset {number: None, offset: leftover_size - 1}
    //}

    let mut nums = HashMap::new();
    nums.insert("one".to_string(), '1');
    nums.insert("two".to_string(), '2');
    nums.insert("three".to_string(), '3');
    nums.insert("four".to_string(), '4');
    nums.insert("five".to_string(), '5');
    nums.insert("six".to_string(), '6');
    nums.insert("seven".to_string(), '7');
    nums.insert("eight".to_string(), '8');
    nums.insert("nine".to_string(), '9');
    if string[index].is_digit(10) {
        let mychar: char = string[index];
        return NumberAndOffset {number: Some(mychar), offset: 0};
    }
    for (written_number, &as_char) in nums.iter() {
        if index + written_number.len() <= string.len() {
            let char_chunk = &string[index..(index + written_number.len())];
            if char_chunk == written_number.chars().collect::<Vec<char>>() {
                let expected_offset = i32::try_from(written_number.len());
                match expected_offset {
                    Ok(a_number) => { 
                    return NumberAndOffset {number: Some(as_char), offset: a_number - 1};
                    }
                    Err(_e) => {panic!("Could not convert to i32")}
                }

            }
        }
    }
    return NumberAndOffset {number: None, offset: 0}

}

#[derive(Debug)]
pub enum MyError {
    Io(Error),
    ParseInt(ParseIntError)
}

impl From<Error> for MyError {
    fn from(other: io::Error) -> MyError {
        MyError::Io(other)
    }
}

impl From<ParseIntError> for MyError {
    fn from(other: ParseIntError) -> MyError {
        MyError::ParseInt(other)
    }
}


pub fn runtwo(filename: &str) -> Result<i32, MyError> {
    println!("starting run two");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for myline in reader.lines() {
        match myline {
            Ok(l) => {
                let line:Vec<char> = l.chars().collect(); // I think this will be faster instead of
                                                          // iterating over the  string a million
                                                          // times for indexing
                let mut first_val = 'a';
                let mut found_first = false;
                let mut last_val = 'a';
                let mut last_offset = 0;
                for (index, _c) in line.iter().enumerate() {
                    let opt_number: NumberAndOffset = get_number(&line, index, last_offset);
                    match opt_number {
                        NumberAndOffset {number: None, offset} => { last_offset = offset}
                        NumberAndOffset {number: Some(number), offset} => {
                            if !found_first {
                                first_val = number;
                                found_first = true;
                            } 
                            last_val = number;
                            last_offset = offset;
                        }
                    }
                }
                let mut combined_string = String::new();
                combined_string.push(first_val);
                if last_val.is_digit(10) {
                    combined_string.push(last_val);
                }
                let parsed_number: Result<i32, _> = combined_string.parse();

                match parsed_number {
                    Ok(n) => {
                        sum += n
                    }
                    Err(e) => {
                        return Err(MyError::ParseInt(e))
                    }
                }
            }
            Err(_e) => {
                println!("yikes")
            }
        }
    }
    println!("Total value: {}", sum);
    return Ok(sum)
}
