extern crate nom;

use nom::{
    multi::{
        separated_list0,
        separated_list1,
        many0,
        many1,
        count
    },
    bytes::complete::{
        tag
    },
    character::complete::{
        line_ending,
        char,
        digit1,
        one_of
    },
    combinator::{
        map_res,
        map
    },
    sequence::{
        pair,
        preceded
    },
    branch::{
        alt
    },
    *
};
use std::vec::Vec;
use crate::error::Error;
use std::str;

fn check_end<A> (input: (&str, A)) -> Result<A, Error> {
    match input {
        ("\n", parsed) => Ok(parsed),
        _ => Err(Error::ParseError("File not consumed completely"))
    }
}

pub fn p1_1(input: String) -> Result<Vec<usize>, Error> {
        separated_list0(
            line_ending,
            map_res(
                digit1::<&str, error::VerboseError<&str>>,
                str::parse)
            )(input.as_str())
    .map_err(|_| Error::ParseError(""))
    .and_then(check_end)
}

#[derive(Debug)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize)
}

pub fn p2_1(input: String) -> Result<Vec<Command>, Error> {
    separated_list0(
        line_ending,
        map(
            sequence::pair(
                branch::alt((
                    tag("forward "),
                    tag("up "),
                    tag("down "))), 
                map_res(
                    digit1::<&str, error::VerboseError<&str>>,
                    str::parse
                )
            ),
            |(order, variation)| match order {
                "forward " => Command::Forward (variation),
                "up "      => Command::Up(variation),
                "down "    => Command::Down(variation),
                // v doesn't happen by previous parsing
                _         => Command::Forward(0)
            }))(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

fn check_length<A>(input: Vec<Vec<A>>) -> Result<Vec<Vec<A>>, Error> {
    let mut ins: Vec<usize> = input.iter().map(|v| v.len()).collect();
    ins.dedup();
    if ins.len() == 1 {Ok(input)}
    else {Err(Error::ParseError("Input codes have different sizes"))}
}

pub fn p3_1(input: String) -> Result<Vec<Vec<usize>>, Error> {
    separated_list1(
        line_ending,
        many1(
            map(
                alt(
                    (char::<&str, error::VerboseError<&str>>('0'),
                     char::<&str, error::VerboseError<&str>>('1'))
                ),
                |input| if input == '1' {1} else {0}
            )
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
    .and_then(check_length)
}

pub type Grid = Vec<Vec<usize>>;
pub fn p4_1(input: String) -> Result<(Vec<usize>, Vec<Grid>), Error> {
    pair(
        separated_list1(
            char(','),
            map_res(
                digit1::<&str, error::VerboseError<&str>>,
                str::parse
            )
        ),
        many1(
            preceded(
                count(line_ending, 2),
                separated_list1(
                    line_ending,
                    preceded(
                        many0(char(' ')),
                        separated_list1(
                            many1(char(' ')),
                            map_res(
                                digit1::<&str, error::VerboseError<&str>>,
                                str::parse
                            )
                        )
                    )
                )
            )
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

pub type Line = (Coord, Coord);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {pub x: usize, pub y: usize}

fn integer(input: &str) -> IResult<&str, usize> {
    map_res(
        digit1,
        str::parse
    )(input)
}

fn coord(input: &str) -> IResult<&str, Coord> {
    map(
        pair(
            integer,
            preceded(
                char(','),
                integer 
            )
        ),
        |(x,y)| Coord {x:x, y:y}
    )(input)
}

pub fn p5_1(input: String) -> Result<Vec<Line>, Error> {
    separated_list1(
        line_ending,
        pair(
            coord,
            preceded(
                tag(" -> "),
                coord
            )
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

pub fn p6_1(input: String) -> Result<Vec<usize>, Error> {
    separated_list1(
        char(','),
        integer
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

fn digit_word_list(input: &str) -> IResult<&str, Vec<String>> {
    use itertools::Itertools;
    separated_list1(
        char(' '),
        map(
            many1(one_of("abcdefg")),
            |chars| chars.into_iter().sorted().collect()
        )
    )(input)
}

pub fn p8_1(input: String) -> Result<Vec<Vec<String>>, Error> {
    separated_list1(
        line_ending,
        preceded(
            digit_word_list,
            preceded(
                tag(" | "),
                digit_word_list
            ) 
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

pub fn p8_2(input: String) -> Result<Vec<(Vec<String>,Vec<String>)>, Error> {
    separated_list1(
        line_ending,
        pair(
            digit_word_list,
            preceded(
                tag(" | "),
                digit_word_list
            ) 
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

pub fn p9_1(input: String)-> Result<Vec<Vec<usize>>, Error> { 
    separated_list1(
        line_ending,
        map_res(
            digit1::<&str, error::VerboseError<&str>>,
            |digits| digits.chars()
                .map(|char| str::parse(char.encode_utf8(&mut [0;1])))
                .collect()
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}

#[derive(Debug)]
pub enum Delimiter {
   Open(DelimiterType),
   Close(DelimiterType)
}

#[derive(Debug, PartialEq, Eq)]
pub enum DelimiterType {
    Parens,
    Brace,
    Bracket,
    Angle
}

fn delimiter(input: &str) -> IResult<&str, Delimiter> {
    use Delimiter::*;
    use DelimiterType::*;
    map_res(
        one_of("<{[()]}>"),
        |char| match char {
            '<' => Ok(Open(Angle)),
            '{' => Ok(Open(Brace)),
            '[' => Ok(Open(Bracket)),
            '(' => Ok(Open(Parens)),
            ')' => Ok(Close(Parens)),
            ']' => Ok(Close(Bracket)),
            '}' => Ok(Close(Brace)),
            '>' => Ok(Close(Angle)),
            _   => Err("unexpected delimiter found")
        }
    )(input)
}

pub fn p10_1(input: String) -> Result<Vec<Vec<Delimiter>>, Error> {
    separated_list1(
        line_ending,
        many1(
            delimiter
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
}
