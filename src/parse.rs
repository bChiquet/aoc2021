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
