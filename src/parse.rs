extern crate nom;

use nom::*;
use std::vec::Vec;
use crate::error::Error;
use std::str;

fn check_end (input: (&str, Vec<usize>)) -> Result<Vec <usize>, Error> {
    match input {
        ("\n", vec) => Ok(vec),
        _ => Err(Error::ParseError)
    }
}

pub fn p1_1(input: String) -> Result<Vec<usize>, Error> {
        multi::separated_list0(
            character::complete::line_ending,
            combinator::map_res(
                character::complete::digit1::<&str, error::VerboseError<&str>>,
                str::parse)
            )(input.as_str())
    .map_err(|_| Error::ParseError)
    .and_then(check_end)
}
