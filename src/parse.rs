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
        digit1
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
