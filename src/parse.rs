extern crate nom;

use nom::*;
use std::vec::Vec;
use crate::error::Error;
use std::str;

fn check_end<A> (input: (&str, Vec<A>)) -> Result<Vec<A>, Error> {
    match input {
        ("\n", vec) => Ok(vec),
        _ => Err(Error::ParseError("File not consumed completely"))
    }
}

pub fn p1_1(input: String) -> Result<Vec<usize>, Error> {
        multi::separated_list0(
            character::complete::line_ending,
            combinator::map_res(
                character::complete::digit1::<&str, error::VerboseError<&str>>,
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
    multi::separated_list0(
        character::complete::line_ending,
        combinator::map(
            sequence::pair(
                branch::alt((
                    bytes::complete::tag("forward "),
                    bytes::complete::tag("up "),
                    bytes::complete::tag("down "))), 
                combinator::map_res(
                    character::complete::digit1::<&str, error::VerboseError<&str>>,
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
    multi::separated_list1(
        character::complete::line_ending,
        multi::many1(
            combinator::map(
                branch::alt(
                    (character::complete::char::<&str, error::VerboseError<&str>>('0'),
                    character::complete::char::<&str, error::VerboseError<&str>>('1'))
                ),
                |input| if input == '1' {1} else {0}
            )
        )
    )(input.as_str())
    .map_err(|_| Error::ParseError("Error occured while parsing"))
    .and_then(check_end)
    .and_then(check_length)
}
