use nom::IResult;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{digit1, alphanumeric1, char, multispace1, multispace0};
use nom::sequence::{preceded,tuple, terminated};
use nom::sequence::{separated_pair, delimited};
use nom::multi::{many0, many1};


#[derive(Debug,PartialEq)]
pub struct Handful {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

#[derive(Debug,PartialEq)]
pub struct Game {
    pub id: i32,
    pub handfuls: Vec<Handful>
}

fn parse_game(line: &str) -> IResult<&str, &str> {
   tag("Game ")(line)
}

fn parse_game_id(line: &str) -> IResult<&str,&str> {
    preceded(parse_game, digit1)(line)
}
fn _parse_handful_section_after_start(line:  &str) -> IResult<&str,&str> {
    preceded(char(';'), preceded(multispace1, take_till(|c| c == ';')))(line)
}

fn _parse_handful_section_start(line:  &str) -> IResult<&str,&str> {
    preceded(char(':'), preceded(multispace1, take_till(|c| c == ';')))(line)
}

fn parse_handful_sections(line: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((_parse_handful_section_start, _parse_handful_section_after_start)))(line)
}
fn parse_colour_start(line: &str) -> IResult<&str,&str> {
    preceded(multispace0, alt((tag("red"), tag("green"), tag("blue"))))(line)
}

fn parse_number(line: &str) -> IResult<&str,&str> {
    preceded(multispace0, digit1)(line)
}

fn parse_colour(line: &str) -> IResult<&str, (&str, &str)> {
    tuple((parse_number, parse_colour_start))(line)
}

fn _parse_handfuls(line: &str) -> IResult<&str,Vec<(&str, &str)>> {
    many0(alt((preceded(tag(", "), parse_colour), parse_colour)))(line)
}

fn parse_handfuls(line: &str) -> Result<Vec<(i32, &str)>, _> {
    let (rem, handful) = _parse_handfuls(line).unwrap();
    let handfuls: Result<Vec<(i32, &str)>, _> = handful.iter().map(|&(s, x)| Ok((s.parse::<i32>()?, x))) .collect();
    match handfuls {
        Ok(vec) => {return Ok(vec)}
        Err(e) => {return Err(e)}
    }

}


fn main() {
    let max_hand: Handful = Handful{ red: 12, green: 14, blue: 13};
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for myline in reader.lines() {
    }

    let test_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let (rem, id) = parse_game_id(test_str).unwrap();
    let (rem, sections) = parse_handful_sections(rem).unwrap();
    println!("id: {}", id);
    println!("sections: {:?}", sections);
    for s in &sections {
        let (_rem, hand) = parse_handfuls(s).unwrap();
        print!("hand: {:?}\n", hand)
    }
    println!("rem: {}", rem);
}
